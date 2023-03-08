//! # IP2C
//!
//! A lib to get the location info from IP address.
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
//! Also, you can use [IpTree] to build IP city location map, eg.
//! ```
//! use ip2c::IpTree;
//!
//! let mut map = IpTree::new();
//! map.ipv4.insert("101.204.128.0-101.204.130.0".parse().unwrap(), ("sichuan", "chengdu")).unwrap();
//! map.ipv4.insert("208.0.0.0/22".parse().unwrap(), ("beijing", "beijing")).unwrap();
//! map.ipv4.insert("208.1.3.6".parse().unwrap(), ("beijing", "beijing")).unwrap();
//! assert_eq!(map.ipv4.query("101.204.129.1".parse().unwrap()), Some(&("sichuan", "chengdu")));
//! assert_eq!(map.ipv4.query("208.0.3.47".parse().unwrap()), Some(&("beijing", "beijing")));
//! assert_eq!(map.ipv4.query("208.11.0.9".parse().unwrap()), None);
//! ```

pub mod itree;
mod ip2c;
pub mod util;
pub mod rir;


pub use crate::itree::*;
pub use crate::ip2c::*;
