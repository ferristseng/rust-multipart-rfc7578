// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use bytes::Bytes;
use common_multipart::client::multipart;
use error::Error;
use futures::{Poll, Stream};

/// Wraps a
/// [`common_multipart::client::multipart::Body`] and makes it compatible with Actix.
///
pub struct Body<'a>(multipart::Body<'a>);

impl<'a> From<multipart::Form<'a>> for Body<'a> {
    #[inline]
    fn from(form: multipart::Form<'a>) -> Body<'a> {
        Body(multipart::Body::from(form))
    }
}

impl<'a> Stream for Body<'a> {
    type Item = Bytes;

    type Error = Error;

    #[inline]
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let Body(ref mut inner) = self;

        inner.poll().map_err(|e| Error::MultipartError(e))
    }
}
