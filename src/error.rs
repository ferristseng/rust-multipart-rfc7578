// Copyright 2017 rust-hyper-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use std::{fmt, error::Error as StdError, io::Error as IoError};

#[derive(Debug)]
pub enum Error {
    HeaderWrite(IoError),
    BoundaryWrite(IoError),
    ContentRead(IoError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::HeaderWrite(ref e) => write!(f, "Error writing headers: {}", e),
            Error::BoundaryWrite(ref e) => write!(f, "Error writing boundary: {}", e),
            Error::ContentRead(ref e) => write!(f, "Error reading content: {}", e),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::HeaderWrite(_) => "Error writing headers",
            Error::BoundaryWrite(_) => "Error writing boundary",
            Error::ContentRead(_) => "Error reading content",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::HeaderWrite(ref e) => Some(e),
            Error::BoundaryWrite(ref e) => Some(e),
            Error::ContentRead(ref e) => Some(e),
        }
    }
}
