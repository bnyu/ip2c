mod itree;
mod ip2c;
mod parse;

pub use ip2c::*;
pub use parse::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut tree = Ip2Code::new();
        let r = load_from_dir(&mut tree, "./data").unwrap();
        println!("{}", tree.ipv4.len());
        println!("{}", tree.ipv6.len());
        assert_eq!(tree.ipv4.len() > 0, true);
        assert_eq!(tree.ipv6.len() > 0, true);
    }
}
