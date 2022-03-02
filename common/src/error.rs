// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::io::Error as IoError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to write multipart content: {0:?}")]
    ContentRead(IoError),
}

impl Into<IoError> for Error {
    fn into(self) -> IoError {
        match self {
            Error::ContentRead(io) => io,
        }
    }
}
