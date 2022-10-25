use std::fmt::{Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::itree::IntervalTreeMap;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Code {
    raw: [u8; 2],
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPv4(pub(crate) u32);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPv6(pub(crate) u128);

impl Display for IPv4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&Ipv4Addr::from(self.0), f)
    }
}

impl Display for IPv6 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&Ipv6Addr::from(self.0), f)
    }
}

impl From<u32> for IPv4 {
    fn from(i: u32) -> Self {
        IPv4(i)
    }
}

impl Into<u32> for IPv4 {
    fn into(self) -> u32 {
        self.0
    }
}

impl From<Ipv4Addr> for IPv4 {
    fn from(ip: Ipv4Addr) -> Self {
        IPv4(ip.into())
    }
}

impl Into<Ipv4Addr> for IPv4 {
    fn into(self) -> Ipv4Addr {
        Ipv4Addr::from(self.0)
    }
}

impl From<u128> for IPv6 {
    fn from(i: u128) -> Self {
        IPv6(i)
    }
}

impl Into<u128> for IPv6 {
    fn into(self) -> u128 {
        self.0
    }
}

impl From<Ipv6Addr> for IPv6 {
    fn from(ip: Ipv6Addr) -> Self {
        IPv6(ip.into())
    }
}

impl Into<Ipv6Addr> for IPv6 {
    fn into(self) -> Ipv6Addr {
        Ipv6Addr::from(self.0)
    }
}

pub struct IP2C {
    pub ipv4: IntervalTreeMap<IPv4, Code>,
    pub ipv6: IntervalTreeMap<IPv6, Code>,
}

impl IP2C {
    pub fn new() -> Self {
        IP2C {
            ipv4: IntervalTreeMap::new(),
            ipv6: IntervalTreeMap::new(),
        }
    }
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
