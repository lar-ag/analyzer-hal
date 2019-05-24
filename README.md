# analyZer-hal-mock

![Minimum Rust Version][min-rust-badge]

This is a collection of types

[Docs](https://docs.rs/embedded-hal-mock/)


## Status

- [x] Pump
- [x] Relay
- [x] Sensor
- [ ] Valve
- [ ] Lamp
- [ ] Relay
- [ ] Autosampler
- [ ] 
- [x] Serial
- [ ] RNG
- [x] I/O pins
- [ ] Timers
- [ ] ...

Pull requests for more mock implementations are welcome! :)


## no\_std

Currently this crate is not `no_std`. If you think this is important, let me
know.


## Usage

See [docs](https://docs.rs/embedded-hal-mock/).


## Development Version of `embedded-hal`

If you would like to use the current development version of `embedded-hal` (or any other version), 
so long as they are API compatible you can use a patch field in your `Cargo.toml` file to override 
the dependency version.

```yaml
[patch.crates-io]
embedded-hal = { git = "https://github.com/rust-embedded/embedded-hal" }
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


<!-- Badges -->
[min-rust-badge]: https://img.shields.io/badge/rustc-1.31+-blue.svg