## Rust Hyper Multipart (RFC 7578)

[![Travis](https://img.shields.io/travis/ferristseng/rust-hyper-multipart-rfc7578.svg)](https://travis-ci.org/ferristseng/rust-hyper-multipart-rfc7578)
[![Crates.io](https://img.shields.io/crates/v/hyper-multipart-rfc7578.svg)](https://crates.io/crates/hyper-multipart-rfc7578)
[![Docs.rs](https://docs.rs/hyper-multipart-rfc7578/badge.svg)](https://docs.rs/hyper-multipart-rfc7578/)

This crate contains an implementation of the multipart/form-data media
type described in [RFC 7578](https://tools.ietf.org/html/rfc7578) for
hyper.

Currently, only the client-side is implemented.

### Usage

```toml
[dependencies]
hyper-multipart-rfc7578 = "0.1.0-alpha3"
```

Because the name of this library is really wordy, I recommend shortening it:

```rust
extern crate hyper_multipart_rfc7578 as hyper_multipart;
```

Using this requires a hyper client compatible with the `multipart::Body`
data structure (see the documentation for more detailed examples):

```rust

use futures::Future;
use hyper::{Method, Request, Client};
use hyper_multipart_rfc7578::client::{self, multipart};

let client = Client::new();
let mut req_builder = Request::get("http://localhost/upload");
let mut form = multipart::Form::default();

form.add_text("test", "Hello World");
let req = form.set_body(&mut req_builder).unwrap();

tokio::run(client.request(req).map(|_| ()).map_err(|_| ()));
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
