#[cfg(test)]
mod tests {
    use crate::itree::{Interval, IntervalTreeMap};
    use crate::*;

    #[test]
    fn parse_eg_data() {
        let mut map = IP2C::new();
        println!("load data...");
        let _ = load_from_dir(&mut map, "./data");
        println!("unknown_ipv4_segments:");
        show_unknown_ipv4_segments(&map.ipv4);
        println!("known_ipv6_code:");
        show_known_ipv6_code(&map.ipv6);
        assert_eq!(map.ipv4.len() > 0, true);
        assert_eq!(map.ipv6.len() > 0, true);
    }

    fn show_unknown_ipv4_segments(tree: &IntervalTreeMap<IPv4, Code>) {
        let mut pre_y = 0;
        let max_y = u32::MAX;
        let mut not_included = Vec::new();
        for (k, _v) in tree.get_tree() {
            let (x, y) = match k {
                Interval::Range(a, b) => (a.0, b.0),
                Interval::Point(a) => (a.0, if a.0 < u32::MAX { a.0 + 1 } else { a.0 }),
            };
            assert_eq!(pre_y <= x, true);
            if x > pre_y { not_included.push(Interval::Range(IPv4(pre_y), IPv4(x))) }
            pre_y = y;
        }
        if max_y > pre_y {
            not_included.push(Interval::Range(IPv4(pre_y), IPv4(max_y)));
            not_included.push(Interval::Point(IPv4(max_y)));
        }

        for item in not_included {
            println!("{}", item)
        }
    }

    fn show_known_ipv6_code(tree: &IntervalTreeMap<IPv6, Code>) {
        for (k, v) in tree.get_tree() {
            println!("{}    {}", k, v)
        }
    }
}
