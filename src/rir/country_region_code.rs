use std::fmt::{Display, Formatter};
use std::net::IpAddr;

use crate::ip2c::IpTree;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Code {
    raw: [u8; 2],
}

impl Code {
    pub fn new(name: &str) -> Option<Code> {
        let bytes = name.as_bytes();
        if bytes.len() == 2 {
            Some(Code { raw: [bytes[0], bytes[1]] })
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        use std::str::from_utf8;
        from_utf8(&self.raw).unwrap()
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub type IpCountryRegionCode = IpTree<Code>;

impl IpCountryRegionCode {
    pub fn query(&self, ip: IpAddr) -> Option<Code> {
        Some(*match ip {
            IpAddr::V4(ip) => self.ipv4.query(ip.into())?,
            IpAddr::V6(ip) => self.ipv6.query(ip.into())?,
        })
    }
}
