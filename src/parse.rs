use std::error::Error;
use crate::ip2c::{Code, Ip2Code};

use std::io;
use std::io::BufRead;
use std::fs;
use std::path::Path;

enum Entity {
    Ipv4(u32, u32),
    Ipv6(u128, u128),
}

pub enum EntityState {
    Assigned,
    Allocated,
    Reserved,
    Available,
    Unknown,
}

pub struct CodeEntity {
    entity: Entity,
    state: EntityState,
    code: [u8; 2],
}


pub fn parse_line(line: &String) -> Option<CodeEntity> {
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

    let code = sl[1].as_bytes();
    let code = if code.len() == 2 { [code[0], code[1]] } else { None? };

    let state = match sl[6] {
        "allocated" => EntityState::Allocated,
        "assigned" => EntityState::Assigned,
        "reserved" => EntityState::Reserved,
        "available" => EntityState::Available,
        "" => None?,
        _ => EntityState::Unknown
    };

    let entity = match sl[2] {
        "ipv4" => {
            let d = parse_ipv4_range(sl[3], sl[4]).ok()?;
            Entity::Ipv4(d[0], d[1])
        }
        "ipv6" => {
            let d = parse_ipv6_range(sl[3], sl[4]).ok()?;
            Entity::Ipv6(d[0], d[1])
        }
        "asn" => None?,
        _ => None?
    };

    Some(CodeEntity {
        entity,
        state,
        code,
    })
}

fn parse_ipv4_range(ip_str: &str, add: &str) -> Result<[u32; 2], Box<dyn Error>> {
    use std::net::Ipv4Addr;
    let ip: Ipv4Addr = ip_str.parse()?;
    let mut start: u32 = 0;
    let mut i: u32 = 0;
    for n in ip.octets() {
        start += (n as u32) << ((3 - i) << 3);
        i += 1;
    }
    let add: u32 = add.parse()?;
    Ok([start, start + add])
}

fn parse_ipv6_range(ip_str: &str, mask: &str) -> Result<[u128; 2], Box<dyn Error>> {
    use std::net::Ipv6Addr;
    let ip: Ipv6Addr = ip_str.parse()?;
    let mut start: u128 = 0;
    let mut i: u128 = 0;
    for n in ip.octets() {
        start += (n as u128) << ((15 - i) << 3);
        i += 1;
    }
    let mask: u8 = mask.parse()?;
    Ok([start, start + (1 << mask)])
}

pub fn put_to_tree(tree: &mut Ip2Code, entity: CodeEntity) -> Result<(), Box<dyn Error>> {
    match entity.entity {
        Entity::Ipv4(x, y) => { tree.ipv4.put(x, y, entity.code)? }
        Entity::Ipv6(x, y) => { tree.ipv6.put(x, y, entity.code)? }
    }
    Ok(())
}

pub fn load_from_dir(tree: &mut Ip2Code, dir_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
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


pub fn load_from_file(tree: &mut Ip2Code, file_path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
    let f = fs::File::open(file_path)?;
    let mut r = io::BufReader::new(f);
    let mut buf = String::new();
    let mut index = 0;
    while let line = r.read_line(&mut buf)? {
        index += 1;
        if line == 0 {
            break;
        }
        if let Some(entity) = parse_line(&buf) {
            put_to_tree(tree, entity)?
        }
        buf.clear();
    };
    Ok(())
}
