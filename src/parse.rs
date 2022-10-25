use std::error::Error;
use crate::ip2c::{IPv4, IPv6, IP2C, Code};

use std::io;
use std::io::BufRead;
use std::fs;
use std::path::Path;
use crate::Interval;

enum IpRange {
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
    range: IpRange,
    // state: IpState,
    code: Code,
}


pub fn parse_line(line: &String) -> Option<Entity> {
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
    if sl[0].starts_with('#') {
        None?
    }

    let code = Code::new(sl[1])?;

    // let state = match sl[6] {
    //     "allocated" => IpState::Allocated,
    //     "assigned" => IpState::Assigned,
    //     "reserved" => IpState::Reserved,
    //     "available" => IpState::Available,
    //     "" => None?,
    //     _ => IpState::Unknown //"intranet" in eg.txt
    // };

    let range = match sl[2] {
        "ipv4" => IpRange::Ipv4(parse_ipv4_range(sl[3], sl[4]).ok()?),
        "ipv6" => IpRange::Ipv6(parse_ipv6_range(sl[3], sl[4]).ok()?),
        _ => None?
    };

    Some(Entity {
        range,
        // state,
        code,
    })
}

fn parse_ipv4_range(ip_str: &str, add: &str) -> Result<Interval<IPv4>, Box<dyn Error>> {
    use std::net::Ipv4Addr;
    let ip: Ipv4Addr = ip_str.parse()?;
    let start: u32 = ip.into();
    let add: u32 = add.parse()?;
    Ok(if add == 1 { Interval::Point(start.into()) } else { Interval::Range(start.into(), start.wrapping_add(add).into()) })
}

fn parse_ipv6_range(ip_str: &str, mask: &str) -> Result<Interval<IPv6>, Box<dyn Error>> {
    use std::net::Ipv6Addr;
    let ip: Ipv6Addr = ip_str.parse()?;
    let start: u128 = ip.into();
    let mask: u8 = mask.parse()?;
    Ok(if mask == 0 { Interval::Point(start.into()) } else { Interval::Range(start.into(), start.wrapping_add(1 << mask).into()) })
}

pub fn put_to_tree(tree: &mut IP2C, entity: Entity) -> Result<(), Box<dyn Error>> {
    match entity.range {
        IpRange::Ipv4(k) => tree.ipv4.put_interval(k, entity.code)?,
        IpRange::Ipv6(k) => tree.ipv6.put_interval(k, entity.code)?,
    }
    Ok(())
}

pub fn load_from_dir(tree: &mut IP2C, dir_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(dir_path)?;
    for f in dir {
        match f {
            Err(e) => return Err(Box::new(e)),
            Ok(f) => {
                if !f.file_type()?.is_dir() {
                    if let Some(filename) = f.file_name().to_str() {
                        if filename.ends_with(".txt") {
                            load_from_file(tree, f.path())?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


pub fn load_from_file(tree: &mut IP2C, file_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
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
            put_to_tree(tree, entity)?
        }
        buf.clear();
    }
    Ok(())
}
