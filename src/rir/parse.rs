use std::error::Error;

use std::io;
use std::io::BufRead;
use std::fs;

use std::path::Path;

use crate::Interval;
use crate::ip2c::{IPv4, IPv6};
use crate::rir::country_region_code::{IpCountryRegionCode, Code};

pub enum IpRange {
    Ipv4(Interval<IPv4>),
    Ipv6(Interval<IPv6>),
}

pub enum IpState {
    Assigned,
    Allocated,
    Reserved,
    Available,
    Unknown,
}

pub struct Entity {
    pub range: IpRange,
    pub state: IpState,
    pub code: Code,
}


pub fn parse_line(line: &String) -> Option<Entity> {
    if line.starts_with('#') {
        None?
    }
    let slices = line.splitn(8, "|");
    let mut sl = [""; 8];
    let mut i = 0;
    for s in slices {
        sl[i] = s;
        i += 1;
        if i >= 8 {
            break;
        }
    }

    let code = Code::new(sl[1])?;

    let state = match sl[6] {
        "allocated" => IpState::Allocated,
        "assigned" => IpState::Assigned,
        "reserved" => IpState::Reserved,
        "available" => IpState::Available,
        "" => None?,
        _ => IpState::Unknown, //"intranet" in example.txt
    };

    let range = match sl[2] {
        "ipv4" => IpRange::Ipv4(parse_ipv4_range(sl[3], sl[4]).ok()?),
        "ipv6" => IpRange::Ipv6(parse_ipv6_range(sl[3], sl[4]).ok()?),
        _ => None?
    };

    Some(Entity {
        range,
        state,
        code,
    })
}

fn parse_ipv4_range(ip_str: &str, add: &str) -> Result<Interval<IPv4>, Box<dyn Error>> {
    let ip: IPv4 = ip_str.parse()?;
    let add: u32 = add.parse()?;
    Ok(if add == 1 { Interval::Point(ip) } else { Interval::Range(ip, ip.0.wrapping_add(add).into()) })
}

fn parse_ipv6_range(ip_str: &str, mask: &str) -> Result<Interval<IPv6>, Box<dyn Error>> {
    let ip: IPv6 = ip_str.parse()?;
    let mask: u8 = mask.parse()?;
    Ok(if mask == 0 { Interval::Point(ip) } else { Interval::Range(ip, ip.0.wrapping_add(1 << mask).into()) })
}

impl IpCountryRegionCode {
    pub fn add_entity(&mut self, entity: Entity) -> Result<(), Box<dyn Error>> {
        match entity.range {
            IpRange::Ipv4(k) => self.ipv4.insert_interval(k, entity.code)?,
            IpRange::Ipv6(k) => self.ipv6.insert_interval(k, entity.code)?,
        }
        Ok(())
    }

    pub fn load_from_dir(&mut self, dir_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let dir = fs::read_dir(dir_path)?;
        for f in dir {
            match f {
                Err(e) => return Err(Box::new(e)),
                Ok(f) => {
                    if !f.file_type()?.is_dir() {
                        if let Some(filename) = f.file_name().to_str() {
                            if filename.ends_with(".txt") {
                                self.load_from_file(f.path())?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let f = fs::File::open(file_path)?;
        let mut r = io::BufReader::new(f);
        let mut buf = String::new();
        // let mut index = 0;
        loop {
            // index += 1;
            let line = r.read_line(&mut buf)?;
            if line == 0 {
                break;
            }
            if let Some(entity) = parse_line(&buf) {
                self.add_entity(entity)?
            }
            buf.clear();
        }
        Ok(())
    }
}
