# ipinfodb-rs
Rust wrapper for ipinfodb api.

## Usage
```rust
extern crate ipinfodb;

// query for current ip
ipinfodb::query("YOUR_API_TOKEN", None);

// or you can specify ip to check geolocation for
ipinfodb::query("YOUR_API_TOKEN", Some("8.8.8.8".parse()?));

```

## License
MIT/Apache-2.0