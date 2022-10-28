pub mod parse;
mod test;

use crate::ip2c::*;

/// Codes for the representation of names of countries and regions
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CountryRegionCode {
    raw: [u8; 2],
}

use std::fmt::{Display, Formatter};
use std::net::IpAddr;

impl Display for CountryRegionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl CountryRegionCode {
    pub fn new(name: &str) -> Option<CountryRegionCode> {
        let bytes = name.as_bytes();
        if bytes.len() == 2 {
            Some(CountryRegionCode { raw: [bytes[0], bytes[1]] })
        } else {
            None
        }
    }

    pub fn name(&self) -> &str {
        use std::str::from_utf8;
        from_utf8(&self.raw).unwrap()
    }
}

pub type IpCodeMap = IpTree<CountryRegionCode>;

impl IpCodeMap {
    /// query [CountryRegionCode] of ip
    pub fn query(&self, ip: IpAddr) -> Option<CountryRegionCode> {
        Some(match ip {
            IpAddr::V4(ip) => *self.ipv4.query(ip.into())?,
            IpAddr::V6(ip) => *self.ipv6.query(ip.into())?,
        })
    }
}
