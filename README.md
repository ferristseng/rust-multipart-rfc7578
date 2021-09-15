[![Workflow Status](https://github.com/ferristseng/rust-multipart-rfc7578/workflows/Rust/badge.svg)](https://github.com/ferristseng/rust-multipart-rfc7578/actions?query=workflow%3A%22Rust%22)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

## Rust Multipart (RFC 7578)

### Components

| Name   | Documentation                                  | Crate                                             |
| ------ | -----------------------------------------------| ------------------------------------------------- |
| common | [![Docs][common docs badge]][common docs link] | [![Crate][common crate badge]][common crate link] |
| actix  | [![Docs][actix docs badge]][actix docs link]   | [![Crate][actix crate badge]][actix crate link]   |
| hyper  | [![Docs][hyper docs badge]][hyper docs link]   | [![Crate][hyper crate badge]][hyper crate link]   |

This crate contains an implementation of the multipart/form-data media
type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).

### Usage

Add either the Hyper implementation or the Actix implementation to
your Cargo.toml file:

#### Actix:

```toml
[dependencies]
actix-multipart-rfc7578 = "0.6"
```

and import:

```rust
extern crate actix_multipart_rfc7578 as multipart;
```

#### Hyper:

```toml
[dependencies]
hyper-multipart-rfc7578 = "0.5"
```

and import:

```rust
extern crate hyper_multipart_rfc7578 as multipart;
```


## Note on Server Implementation

I don't have any plans on implementing the server-side of this any time soon. I ended up implementing the client-side because I couldn't find any good libraries that were compatible with hyper >= 0.11.

Please feel free to submit a pull request, I would gladly review it!

## Alternatives

  * [abonander/multipart](https://github.com/abonander/multipart)
  * [abonander/multipart-async](https://crates.io/crates/multipart-async)
  * [mikedilger/formdata](https://github.com/mikedilger/formdata)
  * [jeizsm/rust-multipart-rfc7578](https://github.com/jeizsm/rust-multipart-rfc7578)

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[common docs badge]: https://docs.rs/common-multipart-rfc7578/badge.svg "common-multipart-rfc7578 documentation"
[common docs link]: https://docs.rs/common-multipart-rfc7578
[common crate badge]: https://img.shields.io/crates/v/common-multipart-rfc7578.svg "common-multipart-rfc7578 crates.io"
[common crate link]: https://crates.io/crates/common-multipart-rfc7578
[actix docs badge]: https://docs.rs/actix-multipart-rfc7578/badge.svg "actix-multipart-rfc7578 documentation"
[actix docs link]: https://docs.rs/actix-multipart-rfc7578
[actix crate badge]: https://img.shields.io/crates/v/actix-multipart-rfc7578.svg "actix-multipart-rfc7578 crates.io"
[actix crate link]: https://crates.io/crates/actix-multipart-rfc7578
[hyper docs badge]: https://docs.rs/hyper-multipart-rfc7578/badge.svg "hyper-multipart-rfc7578 documentation"
[hyper docs link]: https://docs.rs/hyper-multipart-rfc7578
[hyper crate badge]: https://img.shields.io/crates/v/hyper-multipart-rfc7578.svg "hyper-multipart-rfc7578 crates.io"
[hyper crate link]: https://crates.io/crates/hyper-multipart-rfc7578
