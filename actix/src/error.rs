// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use actix_http::error::ResponseError;
use common_multipart;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to send multipart: {0:?}")]
    MultipartError(common_multipart::client::Error),
}

impl ResponseError for Error {}
