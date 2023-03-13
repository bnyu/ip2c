# IP2C

Get the codes for the representation of names of countries and regions from IP address.

Add `ip2c` to your *Cargo.toml* file

```toml
[dependencies]
ip2c = "0.1.5"
```

You can directly use `rir::IpCodeMap` build IP to Codes for the representation of names of countries and regions.

```rust
use ip2c::rir::IpCodeMap;

let mut map = IpCodeMap::new();
map.load_from_dir("./data").expect("load rir txt info failed");
let code = map.query("127.0.0.1".parse().unwrap());
println!("{}", code.unwrap());
 ```

> this library do not provide rir information files, you need download yourself.  
> see [example.txt](./data/example.txt) about download url and official txt format.

Also, you can use `IpTree` to build IP city location map, eg:

```rust
use ip2c::IpTree;

let mut map = IpTree::new();
map.ipv4.insert("101.204.128.0-101.204.130.0".parse().unwrap(), ("Sichuan", "Chengdu")).unwrap();
map.ipv4.insert("43.128.148.100-200".parse().unwrap(), ("South Korea", "Seoul")).unwrap();
map.ipv4.insert("124.156.239.210".parse().unwrap(), ("Japan", "Tokyo")).unwrap();
map.ipv4.insert("123.117.21.0/24".parse().unwrap(), ("Beijing", "Beijing")).unwrap();
map.ipv4.insert("123.117.253.1/20".parse().unwrap(), ("Beijing", "Beijing")).unwrap();
assert_eq!(map.ipv4.query("101.204.129.1".parse().unwrap()), Some(&("Sichuan", "Chengdu")));
assert_eq!(map.ipv4.query("123.117.21.10".parse().unwrap()), Some(&("Beijing", "Beijing")));
assert_eq!(map.ipv4.query("208.123.10.95".parse().unwrap()), None);
 ```

## [doc](https://docs.rs/ip2c/latest/ip2c/)
