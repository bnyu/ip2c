use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::{Interval, IPv4, IPv6};

#[derive(Debug)]
pub struct ParseIpv4ScopeError;

impl Display for ParseIpv4ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("parse ipv4 scope error")
    }
}

impl Error for ParseIpv4ScopeError {}

impl FromStr for Interval<IPv4> {
    type Err = ParseIpv4ScopeError;

    /// eg: parse 100.0.0.1-100 to Scope(100.0.0.1, 100.0.0.100)
    /// eg: parse 100.0.0.0-101.0.0.0 to Scope(100.0.0.0, 101.0.0.0)
    /// eg: parse 100.0.0.1/20 to Scope(100.0.0.0, 100.0.15.255)
    /// eg: parse 100.0.0.200 to Scope(100.0.0.200, 100.0.0.200)
    ///
    /// ```
    /// use std::str::FromStr;
    /// use ip2c::{Interval, IPv4};
    ///
    /// let v = Interval::<IPv4>::from_str("100.0.0.1-100").unwrap();
    /// let ip = IPv4::from_str("100.0.0.1").unwrap();
    /// let ip1 = IPv4::from_str("100.0.0.100").unwrap();
    /// println!("{}", v.to_string());
    /// assert_eq!(v, Interval(ip, ip1));
    ///
    /// let v = Interval::<IPv4>::from_str("100.0.0.0-101.0.0.0").unwrap();
    /// let ip = IPv4::from_str("100.0.0.0").unwrap();
    /// let ip1 = IPv4::from_str("101.0.0.0").unwrap();
    /// println!("{}", v.to_string());
    /// assert_eq!(v, Interval(ip, ip1));
    ///
    /// let v = Interval::<IPv4>::from_str("100.0.0.1/20").unwrap();
    /// let ip = IPv4::from_str("100.0.0.0").unwrap();
    /// let ip1 = IPv4::from_str("100.0.15.255").unwrap();
    /// println!("{}", v.to_string());
    /// assert_eq!(v, Interval(ip, ip1));
    ///
    /// let v = Interval::<IPv4>::from_str("100.0.0.200").unwrap();
    /// let ip = IPv4::from_str("100.0.0.200").unwrap();
    /// let ip1 = IPv4::from_str("100.0.0.200").unwrap();
    /// println!("{}", v.to_string());
    /// assert_eq!(v, Interval(ip, ip1));
    /// ```
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 31 {
            return Err(ParseIpv4ScopeError);
        };
        return if let Some(i) = s.find('-') {
            let (a, b) = s.split_at(i);
            let b = &b[1..];
            let Ok(ip) = IPv4::from_str(a) else {
                return Err(ParseIpv4ScopeError);
            };
            if b.contains('.') {
                let Ok(ip1) = IPv4::from_str(b) else {
                    return Err(ParseIpv4ScopeError);
                };
                Ok(Interval(ip, ip1))
            } else {
                let Ok(n) = b.parse::<u8>() else {
                    return Err(ParseIpv4ScopeError);
                };
                let ip1 = IPv4((ip.0 & (!255)) + (n as u32));
                Ok(Interval(ip, ip1))
            }
        } else if let Some(i) = s.find('/') {
            let (a, b) = s.split_at(i);
            let b = &b[1..];
            let Ok(ip) = IPv4::from_str(a) else {
                return Err(ParseIpv4ScopeError);
            };
            let Ok(n) = b.parse::<u8>() else {
                return Err(ParseIpv4ScopeError);
            };
            if n == 0 {
                if ip.0 == 0 {
                    Ok(Interval(IPv4(0), IPv4(u32::MAX)))
                } else {
                    Err(ParseIpv4ScopeError)
                }
            } else if n == 32 {
                Ok(Interval(ip, ip))
            } else if n < 32 {
                let add: u32 = (1 << (32 - n)) - 1;
                let mask: u32 = !add;
                let ip0 = IPv4(ip.0 & mask);
                let ip1 = IPv4(ip0.0 + add);
                Ok(Interval(ip0, ip1))
            } else {
                Err(ParseIpv4ScopeError)
            }
        } else {
            let Ok(ip) = IPv4::from_str(s) else {
                return Err(ParseIpv4ScopeError);
            };
            Ok(Interval(ip, ip))
        };
    }
}


#[derive(Debug)]
pub struct ParseIpv6ScopeError;

impl Display for ParseIpv6ScopeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("parse ipv6 scope error")
    }
}

impl Error for ParseIpv6ScopeError {}

impl FromStr for Interval<IPv6> {
    type Err = ParseIpv6ScopeError;

    /// eg: parse from ipv6 or ipv6 network segment
    /// ```
    /// use std::str::FromStr;
    /// use ip2c::{Interval, IPv6};
    ///
    /// let v = Interval::<IPv6>::from_str("::1/120").unwrap();
    /// let ip = IPv6::from_str("::").unwrap();
    /// let ip1 = IPv6::from_str("::0.0.0.255").unwrap();
    /// println!("{}", v.to_string());
    /// assert_eq!(v, Interval(ip, ip1));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 48 || !s.contains(':') {
            return Err(ParseIpv6ScopeError);
        };
        return if let Some(i) = s.find('/') {
            let (a, b) = s.split_at(i);
            let b = &b[1..];
            let Ok(ip) = IPv6::from_str(a) else {
                return Err(ParseIpv6ScopeError);
            };
            let Ok(n) = b.parse::<u8>() else {
                return Err(ParseIpv6ScopeError);
            };
            if n == 0 {
                if ip.0 == 0 {
                    Ok(Interval(IPv6(0), IPv6(u128::MAX)))
                } else {
                    Err(ParseIpv6ScopeError)
                }
            } else if n == 128 {
                Ok(Interval(ip, ip))
            } else if n < 128 {
                let add: u128 = (1 << (128 - n)) - 1;
                let mask: u128 = !add;
                let ip0 = IPv6(ip.0 & mask);
                let ip1 = IPv6(ip0.0 + add);
                Ok(Interval(ip0, ip1))
            } else {
                Err(ParseIpv6ScopeError)
            }
        } else {
            let Ok(ip) = IPv6::from_str(s) else {
                return Err(ParseIpv6ScopeError);
            };
            Ok(Interval(ip, ip))
        };
    }
}
