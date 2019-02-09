[![Build Status](https://travis-ci.org/ferristseng/rust-multipart-rfc7578.svg?branch=master)](https://travis-ci.org/ferristseng/rust-multipart-rfc7578)

## Rust Multipart (RFC 7578) (0.1.0)

This crate contains an implementation of the multipart/form-data media
type described in [RFC 7578](https://tools.ietf.org/html/rfc7578).

### Usage

Add either the Hyper implementation or the Actix implementation to
your Cargo.toml file:

#### Actix:

```toml
[dependencies]
actix-multipart-rfc7578 = "0.1.0"
```

and import:

```rust
extern crate actix_multipart_rfc7578 as multipart;
```

#### Hyper:

```toml
[dependencies]
actix-multipart-rfc7578 = "0.1.0"
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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
