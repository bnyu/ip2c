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
        let mut pre_y = 0;
        let max_y = u32::MAX;
        let mut not_included = Vec::new();
        for (k, _v) in tree.tree() {
            let (x, y) = match k {
                Interval::Range(a, b) => (a.0, b.0),
                Interval::Scope(a, b) => (a.0, if b.0 < u32::MAX { b.0 + 1 } else { b.0 }),
            };
            assert_eq!(pre_y <= x, true);
            if x > pre_y { not_included.push(Interval::Range(IPv4(pre_y), IPv4(x))) }
            pre_y = y;
        }
        if max_y > pre_y {
            not_included.push(Interval::Range(IPv4(pre_y), IPv4(max_y)));
            not_included.push(Interval::Scope(IPv4(max_y), IPv4(max_y)));
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
