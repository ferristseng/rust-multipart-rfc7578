// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::error::Error;
use actix_http::body::{Body as ActixBody, BodySize, MessageBody};
use bytes_5::Bytes as Bytes5;
use common_multipart::client::multipart;
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

impl Into<ActixBody> for Body<'static> {
    fn into(self) -> ActixBody {
        ActixBody::Message(Box::new(self))
    }
}

impl<'a> MessageBody for Body<'a> {
    #[inline]
    fn size(&self) -> BodySize {
        BodySize::Stream
    }

    #[inline]
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Bytes5, actix_http::error::Error>>> {
        let pinned = Pin::into_inner(self);
        let Body(ref mut inner) = pinned;

        match Stream::poll_next(Pin::new(inner), cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                let compat_bytes = Bytes5::copy_from_slice(bytes.as_ref());

                Poll::Ready(Some(Ok(compat_bytes)))
            }
            Poll::Ready(Some(Err(err))) => {
                Poll::Ready(Some(Err(Error::MultipartError(err).into())))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
