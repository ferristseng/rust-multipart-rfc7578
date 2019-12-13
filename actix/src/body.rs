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
use futures::{
    task::{Context, Poll},
    Stream,
};
use std::pin::Pin;

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
    type Item = Result<Bytes, Error>;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let Body(ref mut inner) = Pin::into_inner(self);

        match Pin::new(inner).poll_next(cx) {
            Poll::Ready(Some(Ok(t))) => Poll::Ready(Some(Ok(t))),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(Error::MultipartError(e)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
