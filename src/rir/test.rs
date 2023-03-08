#[cfg(test)]
mod tests {
    use crate::itree::Interval;
    use crate::ip2c::*;
    use crate::rir::{IpCodeMap, CountryRegionCode};

    #[test]
    fn parse_eg_data() {
        let mut map = IpCodeMap::new();
        println!("load data...");
        let _ = map.load_from_dir("./data").expect("dirty data");
        println!("unknown_ipv4_segments:");
        show_unknown_ipv4_segments(&map.ipv4);
        println!("known_ipv6_code:");
        show_known_ipv6_code(&map.ipv6);
        let r = map.query("127.0.0.1".parse().unwrap());
        assert_eq!(r.is_some(), true);
        assert_eq!(map.ipv4.len() > 0, true);
        assert_eq!(map.ipv6.len() > 0, true);
    }

    fn show_unknown_ipv4_segments(tree: &Ipv4Tree<CountryRegionCode>) {
        let mut has_pre = false;
        let mut pre_y = 0;
        let mut not_included = Vec::new();
        for (k, _v) in tree.tree() {
            let (x, y) = (k.0.0, k.1.0);
            assert_eq!(!has_pre || pre_y < x, true);
            if has_pre {
                if pre_y < x - 1 {
                    not_included.push(Interval(IPv4(pre_y + 1), IPv4(x - 1)))
                }
            } else {
                has_pre = true;
                if x > 0 {
                    not_included.push(Interval(IPv4(0), IPv4(x - 1)))
                }
            }
            pre_y = y
        }
        if pre_y < u32::MAX {
            if has_pre {
                not_included.push(Interval(IPv4(pre_y + 1), IPv4(u32::MAX)))
            } else {
                not_included.push(Interval(IPv4(0), IPv4(u32::MAX)))
            }
        }

        for item in not_included {
            println!("{}", item)
        }
    }

    fn show_known_ipv6_code(tree: &Ipv6Tree<CountryRegionCode>) {
        for (k, v) in tree.tree() {
            println!("{}    {}", k, v)
        }
    }
}
