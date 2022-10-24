use crate::itree::IntervalTreeMap;

pub type Code = [u8; 2];

pub struct Ip2Code {
    pub ipv4: IntervalTreeMap<u32, Code>,
    pub ipv6: IntervalTreeMap<u128, Code>,
}

impl Ip2Code {
    pub fn new() -> Self {
        Ip2Code {
            ipv4: IntervalTreeMap::new(),
            ipv6: IntervalTreeMap::new(),
        }
    }
}
