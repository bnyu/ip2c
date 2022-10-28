//! # IP2C
//!
//! `ip2c` is a lib to get the location info from IP address.
//!
//! You can directly use [rir::IpCodeMap] build IP to Codes for the representation of names of countries and regions.
//!
//! ```
//! use ip2c::rir::IpCodeMap;
//!
//! let mut map = IpCodeMap::new();
//! map.load_from_dir("./data").expect("load rir txt info failed");
//! let code = map.query("127.0.0.1".parse().unwrap());
//! println!("{}", code.unwrap());
//! ```
//!
//! Also you can use [IpTree] to build IP city location map, eg.
//! ```
//! use ip2c::{Interval, IpTree};
//! let mut map = IpTree::new();
//! map.ipv4.insert("101.204.128.0".parse().unwrap(), "101.204.130.0".parse().unwrap(), "chengdu").unwrap();
//! assert_eq!(map.ipv4.query("101.204.129.1".parse().unwrap()), Some(&"chengdu"));
//! ```

pub mod itree;
mod ip2c;
pub mod rir;


pub use crate::itree::*;
pub use crate::ip2c::*;
