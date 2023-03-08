use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::net::{Ipv4Addr, Ipv6Addr, AddrParseError};
use crate::itree::{IntervalTreeMap};

/// similar with [Ipv4Addr]
/// use `.into()` and `.from()` to convert between them
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPv4(pub(crate) u32);

/// similar with [Ipv6Addr]
/// use `.into()` and `.from()` to convert between them
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPv6(pub(crate) u128);

pub type Ipv4Tree<T> = IntervalTreeMap<IPv4, T>;
pub type Ipv6Tree<T> = IntervalTreeMap<IPv6, T>;

/// both [Ipv4Tree] and [Ipv6Tree]
pub struct IpTree<T> {
    pub ipv4: Ipv4Tree<T>,
    pub ipv6: Ipv6Tree<T>,
}

impl<T> IpTree<T> {
    pub fn new() -> Self {
        IpTree {
            ipv4: Ipv4Tree::new(),
            ipv6: Ipv6Tree::new(),
        }
    }
}

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

impl FromStr for IPv4 {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ipv4Addr::from_str(s)?.into())
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

impl FromStr for IPv6 {
    type Err = AddrParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ipv6Addr::from_str(s)?.into())
    }
}

impl Into<Ipv6Addr> for IPv6 {
    fn into(self) -> Ipv6Addr {
        Ipv6Addr::from(self.0)
    }
}
