// Copyright 2017 rust-multipart-rfc7578 Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use crate::{
    boundary::{BoundaryGenerator, RandomAsciiGenerator},
    error::Error,
};
use bytes::{BufMut, BytesMut};
use futures_core::Stream;
use futures_util::io::{AllowStdIo, AsyncRead, Cursor};
use http::{
    self,
    header::{self, HeaderName},
    request::{Builder, Request},
};
use mime::{self, Mime};
use std::{
    fmt::Display,
    fs::File,
    io::{self, Read},
    iter::Peekable,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
    vec::IntoIter,
};

static CONTENT_DISPOSITION: HeaderName = header::CONTENT_DISPOSITION;
static CONTENT_TYPE: HeaderName = header::CONTENT_TYPE;

/// Async streamable Multipart body.
///
pub struct Body<'a> {
    /// The amount of data to write with each chunk.
    ///
    buf: BytesMut,

    /// The active reader.
    ///
    current: Option<Box<dyn 'a + AsyncRead + Send + Sync + Unpin>>,

    /// The parts as an iterator. When the iterator stops
    /// yielding, the body is fully written.
    ///
    parts: Peekable<IntoIter<Part<'a>>>,

    /// The multipart boundary.
    ///
    boundary: String,
}

impl<'a> Body<'a> {
    /// Writes a CLRF.
    ///
    fn write_crlf(&mut self) {
        self.buf.put_slice(&[b'\r', b'\n']);
    }

    /// Implements section 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    ///
    fn write_boundary(&mut self) {
        self.buf.put_slice(&[b'-', b'-']);
        self.buf.put_slice(self.boundary.as_bytes());
    }

    /// Writes the last form boundary.
    ///
    /// [See](https://tools.ietf.org/html/rfc2046#section-5.1).
    ///
    fn write_final_boundary(&mut self) {
        self.write_boundary();
        self.buf.put_slice(&[b'-', b'-']);
    }

    /// Writes the Content-Disposition, and Content-Type headers.
    ///
    fn write_headers(&mut self, part: &Part) {
        self.write_crlf();
        self.buf.put_slice(CONTENT_TYPE.as_ref());
        self.buf.put_slice(b": ");
        self.buf.put_slice(part.content_type.as_bytes());
        self.write_crlf();
        self.buf.put_slice(CONTENT_DISPOSITION.as_ref());
        self.buf.put_slice(b": ");
        self.buf.put_slice(part.content_disposition.as_bytes());
        self.write_crlf();
        self.write_crlf();
    }
}

impl<'a> Stream for Body<'a> {
    type Item = Result<BytesMut, Error>;

    /// Iterate over each form part, and write it out.
    ///
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let body = self.get_mut();

        match body.current {
            None => {
                if let Some(part) = body.parts.next() {
                    body.write_boundary();
                    body.write_headers(&part);

                    let read: Box<dyn AsyncRead + Send + Sync + Unpin> = match part.inner {
                        Inner::Read(read, _) => Box::new(AllowStdIo::new(read)),
                        Inner::AsyncRead(read) => read,
                        Inner::Text(s) => Box::new(Cursor::new(s)),
                    };

                    body.current = Some(read);

                    cx.waker().wake_by_ref();

                    Poll::Ready(Some(Ok(body.buf.split())))
                } else {
                    // No current part, and no parts left means there is nothing
                    // left to write.
                    //
                    Poll::Ready(None)
                }
            }
            Some(ref mut read) => {
                // Reserve some space to read the next part
                body.buf.reserve(256);
                let len_before = body.buf.len();

                // Init the remaining capacity to 0, and get a mut slice to it
                body.buf.resize(body.buf.capacity(), 0);
                let slice = &mut body.buf.as_mut()[len_before..];

                match Pin::new(read).poll_read(cx, slice) {
                    Poll::Pending => {
                        body.buf.truncate(len_before);
                        Poll::Pending
                    }
                    // Read some data.
                    Poll::Ready(Ok(bytes_read)) => {
                        body.buf.truncate(len_before + bytes_read);

                        if bytes_read == 0 {
                            // EOF: No data left to read. Get ready to move onto write the next part.
                            body.current = None;
                            body.write_crlf();
                            if body.parts.peek().is_none() {
                                // If there is no next part, write the final boundary
                                body.write_final_boundary();
                                body.write_crlf();
                            }
                        }

                        Poll::Ready(Some(Ok(body.buf.split())))
                    }
                    // Error reading from underlying stream.
                    Poll::Ready(Err(e)) => {
                        body.buf.truncate(len_before);
                        Poll::Ready(Some(Err(Error::ContentRead(e))))
                    }
                }
            }
        }
    }
}

/// Implements the multipart/form-data media type as described by
/// RFC 7578.
///
/// [See](https://tools.ietf.org/html/rfc7578#section-1).
///
pub struct Form<'a> {
    parts: Vec<Part<'a>>,

    /// The auto-generated boundary as described by 4.1.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.1).
    ///
    boundary: String,
}

impl<'a> Default for Form<'a> {
    /// Creates a new form with the default boundary generator.
    ///
    #[inline]
    fn default() -> Form<'a> {
        Form::new::<RandomAsciiGenerator>()
    }
}

impl<'a> Form<'a> {
    /// Creates a new form with the specified boundary generator function.
    ///
    /// # Examples
    ///
    /// ```
    /// # use common_multipart_rfc7578::client::multipart::{
    /// #     self,
    /// #     BoundaryGenerator
    /// # };
    /// #
    /// struct TestGenerator;
    ///
    /// impl BoundaryGenerator for TestGenerator {
    ///     fn generate_boundary() -> String {
    ///         "test".to_string()
    ///     }
    /// }
    ///
    /// let form = multipart::Form::new::<TestGenerator>();
    /// ```
    ///
    #[inline]
    pub fn new<G>() -> Form<'a>
    where
        G: BoundaryGenerator,
    {
        Form {
            parts: vec![],
            boundary: G::generate_boundary(),
        }
    }

    /// Adds a text part to the Form.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    ///
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_text("text", "Hello World!");
    /// form.add_text("more", String::from("Hello Universe!"));
    /// ```
    ///
    pub fn add_text<N, T>(&mut self, name: N, text: T)
    where
        N: Display,
        T: Into<String>,
    {
        self.parts.push(Part::new::<_, String>(
            Inner::Text(text.into()),
            name,
            None,
            None,
        ))
    }

    /// Adds a readable part to the Form.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_reader("input", bytes);
    /// ```
    ///
    pub fn add_reader<F, R>(&mut self, name: F, read: R)
    where
        F: Display,
        R: 'a + Read + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::Read(read, None),
            name,
            None,
            None,
        ));
    }

    /// Adds a readable part to the Form.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use futures_util::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_async_reader("input", bytes);
    /// ```
    ///
    pub fn add_async_reader<F, R>(&mut self, name: F, read: R)
    where
        F: Display,
        R: 'a + AsyncRead + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::AsyncRead(read),
            name,
            None,
            None,
        ));
    }

    /// Adds a file, and attempts to derive the mime type.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    ///
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_file("file", file!()).expect("file to exist");
    /// ```
    ///
    pub fn add_file<P, F>(&mut self, name: F, path: P) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: Display,
    {
        self._add_file(name, path, None)
    }

    /// Adds a file with the specified mime type to the form.
    /// If the mime type isn't specified, a mime type will try to
    /// be derived.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    ///
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_file_with_mime("data", "test.csv", mime::TEXT_CSV);
    /// ```
    ///
    pub fn add_file_with_mime<P, F>(&mut self, name: F, path: P, mime: Mime) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: Display,
    {
        self._add_file(name, path, Some(mime))
    }

    /// Internal method for adding a file part to the form.
    ///
    fn _add_file<P, F>(&mut self, name: F, path: P, mime: Option<Mime>) -> io::Result<()>
    where
        P: AsRef<Path>,
        F: Display,
    {
        let f = File::open(&path)?;
        let mime = mime.or_else(|| mime_guess::from_path(&path).first());

        let len = match f.metadata() {
            // If the path is not a file, it can't be uploaded because there
            // is no content.
            //
            Ok(ref meta) if !meta.is_file() => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected a file not directory",
            )),

            // If there is some metadata on the file, try to derive some
            // header values.
            //
            Ok(ref meta) => Ok(Some(meta.len())),

            // The file metadata could not be accessed. This MIGHT not be an
            // error, if the file could be opened.
            //
            Err(e) => Err(e),
        }?;

        let read = Box::new(f);

        self.parts.push(Part::new(
            Inner::Read(read, len),
            name,
            mime,
            Some(path.as_ref().as_os_str().to_string_lossy()),
        ));

        Ok(())
    }

    /// Adds a readable part to the Form as a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_reader_file("input", bytes, "filename.txt");
    /// ```
    ///
    pub fn add_reader_file<F, G, R>(&mut self, name: F, read: R, filename: G)
    where
        F: Display,
        G: Into<String>,
        R: 'a + Read + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::Read(read, None),
            name,
            None,
            Some(filename.into()),
        ));
    }

    /// Adds a readable part to the Form as a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use futures_util::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_async_reader_file("input", bytes, "filename.txt");
    /// ```
    ///
    pub fn add_async_reader_file<F, G, R>(&mut self, name: F, read: R, filename: G)
    where
        F: Display,
        G: Into<String>,
        R: 'a + AsyncRead + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::AsyncRead(read),
            name,
            None,
            Some(filename.into()),
        ));
    }

    /// Adds a readable part to the Form as a file with a specified mime.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use std::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_reader_file_with_mime("input", bytes, "filename.txt", mime::TEXT_PLAIN);
    /// ```
    ///
    pub fn add_reader_file_with_mime<F, G, R>(&mut self, name: F, read: R, filename: G, mime: Mime)
    where
        F: Display,
        G: Into<String>,
        R: 'a + Read + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::Read(read, None),
            name,
            Some(mime),
            Some(filename.into()),
        ));
    }

    /// Adds a readable part to the Form as a file with a specified mime.
    ///
    /// # Examples
    ///
    /// ```
    /// use common_multipart_rfc7578::client::multipart;
    /// use futures_util::io::Cursor;
    ///
    /// let bytes = Cursor::new("Hello World!");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_async_reader_file_with_mime("input", bytes, "filename.txt", mime::TEXT_PLAIN);
    /// ```
    ///
    pub fn add_async_reader_file_with_mime<F, G, R>(
        &mut self,
        name: F,
        read: R,
        filename: G,
        mime: Mime,
    ) where
        F: Display,
        G: Into<String>,
        R: 'a + AsyncRead + Send + Sync + Unpin,
    {
        let read = Box::new(read);

        self.parts.push(Part::new::<_, String>(
            Inner::AsyncRead(read),
            name,
            Some(mime),
            Some(filename.into()),
        ));
    }

    /// Updates a request instance with the multipart Content-Type header
    /// and the payload data.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper::{Method, Request};
    /// use hyper_multipart_rfc7578::client::multipart;
    ///
    /// let mut req_builder = Request::post("http://localhost:80/upload");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_text("text", "Hello World!");
    /// let req = form.set_body::<multipart::Body>(req_builder).unwrap();
    /// ```
    ///
    pub fn set_body<B>(self, req: Builder) -> Result<Request<B>, http::Error>
    where
        B: From<Body<'a>>,
    {
        self.set_body_convert::<B, B>(req)
    }

    /// Updates a request instance with the multipart Content-Type header
    /// and the payload data.
    ///
    /// Allows converting body into an intermediate type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper::{Body, Method, Request};
    /// use hyper_multipart_rfc7578::client::multipart;
    ///
    /// let mut req_builder = Request::post("http://localhost:80/upload");
    /// let mut form = multipart::Form::default();
    ///
    /// form.add_text("text", "Hello World!");
    /// let req = form.set_body_convert::<hyper::Body, multipart::Body>(req_builder).unwrap();
    /// ```
    ///
    pub fn set_body_convert<B, I>(self, req: Builder) -> Result<Request<B>, http::Error>
    where
        I: From<Body<'a>> + Into<B>,
    {
        req.header(&CONTENT_TYPE, self.content_type().as_str())
            .body(I::from(Body::from(self)).into())
    }

    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", &self.boundary)
    }
}

impl<'a> From<Form<'a>> for Body<'a> {
    /// Turns a `Form` into a multipart `Body`.
    ///
    fn from(form: Form<'a>) -> Self {
        Body {
            buf: BytesMut::with_capacity(2048),
            current: None,
            parts: form.parts.into_iter().peekable(),
            boundary: form.boundary,
        }
    }
}

/// One part of a body delimited by a boundary line.
///
/// [See RFC2046 5.1](https://tools.ietf.org/html/rfc2046#section-5.1).
///
pub struct Part<'a> {
    inner: Inner<'a>,

    /// Each part can include a Content-Type header field. If this
    /// is not specified, it defaults to "text/plain", or
    /// "application/octet-stream" for file data.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.4)
    ///
    content_type: String,

    /// Each part must contain a Content-Disposition header field.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.2).
    ///
    content_disposition: String,
}

impl<'a> Part<'a> {
    /// Internal method to build a new Part instance. Sets the disposition type,
    /// content-type, and the disposition parameters for name, and optionally
    /// for filename.
    ///
    /// Per [4.3](https://tools.ietf.org/html/rfc7578#section-4.3), if multiple
    /// files need to be specified for one form field, they can all be specified
    /// with the same name parameter.
    ///
    fn new<N, F>(inner: Inner<'a>, name: N, mime: Option<Mime>, filename: Option<F>) -> Part<'a>
    where
        N: Display,
        F: Display,
    {
        // `name` disposition parameter is required. It should correspond to the
        // name of a form field.
        //
        // [See 4.2](https://tools.ietf.org/html/rfc7578#section-4.2)
        //
        let mut disposition_params = vec![format!("name=\"{}\"", name)];

        // `filename` can be supplied for files, but is totally optional.
        //
        // [See 4.2](https://tools.ietf.org/html/rfc7578#section-4.2)
        //
        if let Some(filename) = filename {
            disposition_params.push(format!("filename=\"{}\"", filename));
        }

        let content_type = format!("{}", mime.unwrap_or_else(|| inner.default_content_type()));

        Part {
            inner,
            content_type,
            content_disposition: format!("form-data; {}", disposition_params.join("; ")),
        }
    }
}

enum Inner<'a> {
    /// The `Read` and `AsyncRead` variants captures multiple cases.
    ///
    ///   * The first is it supports uploading a file, which is explicitly
    ///     described in RFC 7578.
    ///
    ///   * The second (which is not described by RFC 7578), is it can handle
    ///     arbitrary input streams (for example, a server response).
    ///     Any arbitrary input stream is automatically considered a file,
    ///     and assigned the corresponding content type if not explicitly
    ///     specified.
    ///
    Read(Box<dyn 'a + Read + Send + Sync + Unpin>, Option<u64>),

    AsyncRead(Box<dyn 'a + AsyncRead + Send + Sync + Unpin>),

    /// The `String` variant handles "text/plain" form data payloads.
    ///
    Text(String),
}

impl<'a> Inner<'a> {
    /// Returns the default Content-Type header value as described in section 4.4.
    ///
    /// [See](https://tools.ietf.org/html/rfc7578#section-4.4)
    ///
    fn default_content_type(&self) -> Mime {
        match *self {
            Inner::Read(_, _) | Inner::AsyncRead(_) => mime::APPLICATION_OCTET_STREAM,
            Inner::Text(_) => mime::TEXT_PLAIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Body, Form};
    use crate::error::Error;
    use bytes::BytesMut;
    use futures_util::TryStreamExt;
    use std::{
        io::Cursor,
        path::{Path, PathBuf},
    };

    async fn form_output(form: Form<'_>) -> String {
        let result: Result<BytesMut, Error> = Body::from(form).try_concat().await;

        assert!(result.is_ok());

        let bytes = result.unwrap();
        let data = std::str::from_utf8(bytes.as_ref()).unwrap();

        data.into()
    }

    fn test_file_path() -> PathBuf {
        // common/src/data/test.txt
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("data")
            .join("test.txt")
    }

    #[tokio::test]
    async fn add_text_returns_expected_result() {
        let mut form = Form::default();

        form.add_text("test", "Hello World!");

        let data = form_output(form).await;

        assert!(data.contains("Hello World!"));
    }

    #[tokio::test]
    async fn add_reader_returns_expected_result() {
        let bytes = Cursor::new("Hello World!");
        let mut form = Form::default();

        form.add_reader("input", bytes);

        let data = form_output(form).await;

        assert!(data.contains("Hello World!"));
    }

    #[tokio::test]
    async fn add_file_returns_expected_result() {
        let mut form = Form::default();

        assert!(form.add_file("test_file.txt", test_file_path()).is_ok());

        let data = form_output(form).await;

        assert!(data.contains("This is a test file!"));
        assert!(data.contains("text/plain"));
    }

    #[tokio::test]
    async fn add_file_with_mime_returns_expected_result() {
        let mut form = Form::default();

        assert!(form
            .add_file_with_mime("test_file.txt", test_file_path(), mime::TEXT_CSV)
            .is_ok());

        let data = form_output(form).await;

        assert!(data.contains("This is a test file!"));
        assert!(data.contains("text/csv"));
    }

    struct FixedBoundary;
    impl crate::boundary::BoundaryGenerator for FixedBoundary {
        fn generate_boundary() -> String {
            "boundary".to_owned()
        }
    }

    #[tokio::test]
    async fn test_form_body_stream() {
        let mut form = Form::new::<FixedBoundary>();
        // Text fields
        form.add_text("name1", "value1");
        form.add_text("name2", "value2");

        // Reader field
        form.add_reader("input", Cursor::new("Hello World!"));

        let result: BytesMut = Body::from(form).try_concat().await.unwrap();

        assert_eq!(
            result.as_ref(),
            [
                b"--boundary\r\n".as_ref(),
                b"content-type: text/plain\r\n".as_ref(),
                b"content-disposition: form-data; name=\"name1\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"value1\r\n".as_ref(),
                b"--boundary\r\n".as_ref(),
                b"content-type: text/plain\r\n".as_ref(),
                b"content-disposition: form-data; name=\"name2\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"value2\r\n".as_ref(),
                b"--boundary\r\n".as_ref(),
                b"content-type: application/octet-stream\r\n".as_ref(),
                b"content-disposition: form-data; name=\"input\"\r\n".as_ref(),
                b"\r\n".as_ref(),
                b"Hello World!\r\n".as_ref(),
                b"--boundary--\r\n".as_ref(),
            ]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<u8>>()
        );
    }

    #[tokio::test]
    async fn test_content_type_header_format() {
        use http::Request;

        let mut form = Form::new::<FixedBoundary>();
        // Text fields
        form.add_text("name1", "value1");
        form.add_text("name2", "value2");

        let builder = Request::builder();
        let body = form.set_body::<Body>(builder).unwrap();

        assert_eq!(
            body.headers().get("Content-Type").unwrap().as_bytes(),
            b"multipart/form-data; boundary=boundary",
        )
    }
}
