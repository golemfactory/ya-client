//! Web utils
use crate::{Error, Result};
use awc::http::header::qitem;
use awc::{
    error::PayloadError,
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    ClientRequest, ClientResponse, SendClientRequest,
};
use bytes::{Buf, Bytes, BytesMut};
use futures::stream::Peekable;
use futures::{Stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use std::cmp::max;
use std::convert::TryFrom;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{env, rc::Rc, str::FromStr, time::Duration};
use url::{form_urlencoded, Url};

pub const YAGNA_API_URL_ENV_VAR: &str = "YAGNA_API_URL";
pub const DEFAULT_YAGNA_API_URL: &str = "http://127.0.0.1:7465";
const MAX_BODY_SIZE: usize = 10 * 1024 * 1024;

pub fn rest_api_url() -> Url {
    let api_url = env::var(YAGNA_API_URL_ENV_VAR).unwrap_or(DEFAULT_YAGNA_API_URL.into());
    api_url
        .parse()
        .expect(&format!("invalid API URL: {}", api_url))
}

#[derive(Clone, Debug)]
pub enum WebAuth {
    Bearer(String),
}

/// Convenient wrapper for the [`awc::Client`](
/// https://docs.rs/awc/1.0/awc/struct.Client.html) with builder.
#[derive(Clone)]
pub struct WebClient {
    base_url: Rc<Url>,
    awc: awc::Client,
}

pub trait WebInterface {
    const API_URL_ENV_VAR: &'static str;
    const API_SUFFIX: &'static str;

    fn rebase_service_url(base_url: Rc<Url>) -> Result<Rc<Url>> {
        if let Some(url) = std::env::var(Self::API_URL_ENV_VAR).ok() {
            return Ok(Url::from_str(&url)?.into());
        }
        Ok(base_url.join(Self::API_SUFFIX)?.into())
    }

    fn from_client(client: WebClient) -> Self;
}

pub struct WebRequest<T> {
    inner_request: T,
    url: String,
}

impl WebClient {
    pub fn builder() -> WebClientBuilder {
        WebClientBuilder::default()
    }

    pub fn with_token(token: &str) -> WebClient {
        WebClientBuilder::default().auth_token(token).build()
    }

    /// constructs endpoint url in form of `<base_url>/<suffix>`.
    ///
    /// suffix should not have leading slash ie. `offer` not `/offer`
    fn url<T: AsRef<str>>(&self, suffix: T) -> Result<url::Url> {
        Ok(self.base_url.join(suffix.as_ref())?)
    }

    pub fn request(&self, method: Method, url: &str) -> WebRequest<ClientRequest> {
        let url = self.url(url).unwrap().to_string();
        log::debug!("doing {} on {}", method, url);
        WebRequest {
            inner_request: self.awc.request(method, &url),
            url,
        }
    }

    pub async fn event_stream(&self, url: &str) -> Result<impl Stream<Item = Result<Event>>> {
        let url = self.url(url).unwrap().to_string();
        log::debug!("event stream at {}", url);
        let request = self
            .awc
            .request(Method::GET, &url)
            .set(awc::http::header::Accept(vec![qitem(
                mime::TEXT_EVENT_STREAM,
            )]));
        let stream = request
            .send()
            .await?
            .into_stream()
            .map_err(Error::from)
            .event_stream();
        Ok(stream)
    }

    pub fn get(&self, url: &str) -> WebRequest<ClientRequest> {
        self.request(Method::GET, url)
    }

    pub fn post(&self, url: &str) -> WebRequest<ClientRequest> {
        self.request(Method::POST, url)
    }

    pub fn put(&self, url: &str) -> WebRequest<ClientRequest> {
        self.request(Method::PUT, url)
    }

    pub fn delete(&self, url: &str) -> WebRequest<ClientRequest> {
        self.request(Method::DELETE, url)
    }

    pub fn interface<T: WebInterface>(&self) -> Result<T> {
        self.interface_at(None)
    }

    pub fn interface_at<T: WebInterface>(&self, base_url: impl Into<Option<Url>>) -> Result<T> {
        let base_url = match base_url.into() {
            Some(url) => url.into(),
            None => T::rebase_service_url(self.base_url.clone())?,
        };

        let awc = self.awc.clone();
        Ok(T::from_client(WebClient { base_url, awc }))
    }
}

impl WebRequest<ClientRequest> {
    pub fn send_json<T: Serialize + std::fmt::Debug>(
        self,
        value: &T,
    ) -> WebRequest<SendClientRequest> {
        log::trace!("sending payload: {:?}", value);
        WebRequest {
            inner_request: self.inner_request.send_json(value),
            url: self.url,
        }
    }

    pub fn send_bytes(self, bytes: Vec<u8>) -> WebRequest<SendClientRequest> {
        let url = self.url;
        let inner_request = self
            .inner_request
            .content_type("application/octet-stream")
            .send_body(bytes);
        WebRequest { url, inner_request }
    }

    pub fn send(self) -> WebRequest<SendClientRequest> {
        WebRequest {
            inner_request: self.inner_request.send(),
            url: self.url,
        }
    }
}

async fn filter_http_status<T>(
    mut response: ClientResponse<T>,
    url: String,
) -> Result<ClientResponse<T>>
where
    T: Stream<Item = std::result::Result<Bytes, PayloadError>> + Unpin,
{
    log::trace!("{:?}", response.headers());
    if response.status().is_success() {
        Ok(response)
    } else {
        if response
            .headers()
            .get("content-type")
            .map(|v| v.as_bytes() == b"application/json")
            .unwrap_or_default()
        {
            Err((response.status(), url, response.json().await).into())
        } else {
            match response.body().await {
                Ok(ref bytes) => {
                    let message = String::from_utf8_lossy(&bytes);
                    Err(Error::HttpStatusCode {
                        code: response.status(),
                        url,
                        msg: message.to_string(),
                        bt: Default::default(),
                    })
                }
                Err(_e) => Err(Error::HttpStatusCode {
                    code: response.status(),
                    url,
                    msg: response.status().as_str().to_string(),
                    bt: Default::default(),
                }),
            }
        }
    }
}

impl WebRequest<SendClientRequest> {
    pub async fn bytes(self) -> Result<Vec<u8>> {
        let url = self.url.clone();
        let response = self
            .inner_request
            .await
            .map_err(|e| Error::from((e, url.clone())))?;

        let mut response = filter_http_status(response, url).await?;
        let raw_body = response.body().await?;
        Ok(raw_body.to_vec())
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        let url = self.url.clone();
        let response = self
            .inner_request
            .await
            .map_err(|e| Error::from((e, url.clone())))?;

        let mut response = filter_http_status(response, url).await?;

        // allow empty body and no content (204) to pass smoothly
        if StatusCode::NO_CONTENT == response.status()
            || Some("0")
                == response
                    .headers()
                    .get(header::CONTENT_LENGTH)
                    .and_then(|h| h.to_str().ok())
        {
            return Ok(serde_json::from_str(&format!(
                "\"[ EMPTY BODY (http: {}) ]\"",
                response.status()
            ))?);
        }
        let raw_body = response.body().limit(MAX_BODY_SIZE).await?;
        let body = std::str::from_utf8(&raw_body)?;
        log::debug!(
            "WebRequest.json(). url={}, resp={}",
            self.url,
            body.split_at(512.min(body.len())).0
        );
        Ok(serde_json::from_str(body)?)
    }
}

// this is used internally to translate from HTTP Timeout into default result
// (empty vec most of the time)
pub(crate) fn default_on_timeout<T: Default>(err: Error) -> Result<T> {
    match err {
        Error::TimeoutError { msg, url, .. } => {
            log::trace!("timeout getting url {}: {}", url, msg);
            Ok(Default::default())
        }
        _ => Err(err),
    }
}

#[derive(Clone, Debug)]
pub struct WebClientBuilder {
    pub(crate) api_url: Option<Url>,
    pub(crate) auth: Option<WebAuth>,
    pub(crate) headers: HeaderMap,
    pub(crate) timeout: Option<Duration>,
}

impl WebClientBuilder {
    pub fn auth_token(mut self, token: &str) -> Self {
        self.auth = Some(WebAuth::Bearer(token.to_string()));
        self
    }

    pub fn api_url(mut self, url: Url) -> Self {
        self.api_url = Some(url);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn header(mut self, name: String, value: String) -> Result<Self> {
        let name = HeaderName::from_str(name.as_str())?;
        let value = HeaderValue::from_str(value.as_str())?;

        self.headers.insert(name, value);
        Ok(self)
    }

    pub fn build(self) -> WebClient {
        let mut builder = awc::Client::build();

        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        } else {
            builder = builder.disable_timeout();
        }
        if let Some(auth) = &self.auth {
            builder = match auth {
                WebAuth::Bearer(token) => builder.bearer_auth(token),
            }
        }
        for (key, value) in self.headers.iter() {
            builder = builder.header(key.clone(), value.clone());
        }

        WebClient {
            base_url: Rc::new(self.api_url.unwrap_or_else(|| rest_api_url())),
            awc: builder.finish(),
        }
    }
}

impl Default for WebClientBuilder {
    fn default() -> Self {
        WebClientBuilder {
            api_url: None,
            auth: None,
            headers: HeaderMap::new(),
            timeout: None,
        }
    }
}

/// Builder for the query part of the URLs.
pub struct QueryParamsBuilder<'a> {
    serializer: form_urlencoded::Serializer<'a, String>,
}

impl<'a> QueryParamsBuilder<'a> {
    pub fn new() -> Self {
        let serializer = form_urlencoded::Serializer::new("".into());
        QueryParamsBuilder { serializer }
    }

    pub fn put<N: ToString, V: ToString>(mut self, name: N, value: Option<V>) -> Self {
        if let Some(v) = value {
            self.serializer
                .append_pair(&name.to_string(), &v.to_string());
        };
        self
    }

    pub fn build(mut self) -> String {
        self.serializer.finish()
    }
}

#[derive(Debug)]
pub struct Event {
    pub id: Option<u64>,
    pub event: String,
    pub data: String,
}

impl TryFrom<String> for Event {
    type Error = Error;

    fn try_from(string: String) -> Result<Self> {
        let mut id = None;
        let mut event = String::new();
        let mut data = Vec::<String>::new();

        for line in string.split('\n') {
            let split = line.splitn(2, ":").collect::<Vec<_>>();
            if split.len() < 2 {
                continue;
            }

            let value = split[1].trim_start();
            match split[0] {
                "event" => event = value.into(),
                "data" => data.push(value.into()),
                "id" => {
                    id = match value.parse::<u64>() {
                        Ok(id) => Some(id),
                        _ => None,
                    }
                }
                _ => (),
            }
        }
        if event.is_empty() {
            return Err(Error::EventStreamError("Missing event entry".into()));
        }
        let data = data.join("\n");
        Ok(Event { id, event, data })
    }
}

pub trait EventStreamExt<S, E>
where
    S: Stream<Item = std::result::Result<Bytes, E>> + Unpin + 'static,
    E: Into<Error>,
{
    fn event_stream(self) -> EventStream<S, E>;
}

impl<S, E> EventStreamExt<S, E> for S
where
    S: Stream<Item = std::result::Result<Bytes, E>> + Unpin + 'static,
    E: Into<Error>,
{
    fn event_stream(self) -> EventStream<S, E> {
        EventStream::new(self)
    }
}

pub struct EventStream<S, E>
where
    S: Stream<Item = std::result::Result<Bytes, E>> + Unpin + 'static,
{
    inner: Peekable<S>,
    buffer: BytesMut,
}

impl<S, E> EventStream<S, E>
where
    S: Stream<Item = std::result::Result<Bytes, E>> + Unpin + 'static,
    E: Into<Error>,
{
    pub fn new(stream: S) -> Self {
        EventStream {
            inner: stream.peekable(),
            buffer: BytesMut::new(),
        }
    }

    fn next_event(&mut self, start_idx: usize) -> Option<Result<Event>> {
        let idx = max(0, start_idx as i64 - 1) as usize;
        if let Some(idx) = Self::find(self.buffer.bytes(), b"\n\n", idx) {
            let bytes = self.buffer.split_to(idx);
            return String::from_utf8(bytes.to_vec())
                .map(Event::try_from)
                .map_err(Error::from)
                .ok();
        }
        None
    }

    fn find(source: &[u8], find: &[u8], start_idx: usize) -> Option<usize> {
        let mut find_idx = 0;
        for (i, b) in source.iter().enumerate().skip(start_idx) {
            if *b == find[find_idx] {
                find_idx += 1;
                if find_idx == find.len() {
                    return Some(i);
                }
            } else {
                find_idx = 0;
            }
        }
        None
    }
}

impl<S, E> Stream for EventStream<S, E>
where
    S: Stream<Item = std::result::Result<Bytes, E>> + Unpin + 'static,
    E: Into<Error>,
{
    type Item = std::result::Result<Event, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if let Some(result) = this.next_event(0) {
            return Poll::Ready(Some(result));
        }

        match Pin::new(&mut this.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(bytes))) => {
                let idx = this.buffer.len();
                this.buffer.extend(bytes.into_iter());

                if let Some(result) = this.next_event(idx) {
                    Poll::Ready(Some(result))
                } else {
                    if let Poll::Ready(_) = Pin::new(&mut this.inner).poll_peek(cx) {
                        cx.waker().wake_by_ref();
                    }
                    Poll::Pending
                }
            }
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Macro to facilitate URL formatting for REST API async bindings
macro_rules! url_format {
    {
        $path:expr $(,$var:ident)* $(,#[query] $varq:ident)* $(,)?
    } => {{
        let mut url = format!( $path $(, $var=$var)* );
        let query = crate::web::QueryParamsBuilder::new()
            $( .put( stringify!($varq), $varq ) )*
            .build();
        if query.len() > 1 {
            url = format!("{}?{}", url, query)
        }
        url
    }};
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use bytes::Bytes;
    use crate::web::EventStream;
    use futures::{StreamExt, FutureExt, Stream};
    use crate::Error;

    #[test]
    fn static_url() {
        assert_eq!(url_format!("foo"), "foo");
    }

    #[test]
    fn single_placeholder_url() {
        let bar = "qux";
        assert_eq!(url_format!("foo/{}", bar), "foo/qux");
    }

    #[test]
    fn single_var_url() {
        let bar = "qux";
        assert_eq!(url_format!("foo/{bar}", bar), "foo/qux");
    }

    // compilation error when wrong var name given
    //    #[test]
    //    fn wrong_single_var_url() {
    //        let bar="qux";
    //        assert_eq!(url_format!("foo/{baz}", bar), "foo/{}");
    //    }

    #[test]
    fn multi_var_url() {
        let bar = "qux";
        let baz = "quz";
        assert_eq!(
            url_format!("foo/{bar}/fuu/{baz}", bar, baz),
            "foo/qux/fuu/quz"
        );
    }

    #[test]
    fn empty_query_url() {
        let bar = Option::<String>::None;
        assert_eq!(url_format!("foo", #[query] bar), "foo");
    }

    #[test]
    #[rustfmt::skip]
    fn single_query_url() {
        let bar= Some("qux");
        assert_eq!(url_format!("foo", #[query] bar), "foo?bar=qux");
    }

    #[test]
    fn mix_query_url() {
        let bar = Option::<String>::None;
        let baz = Some("quz");
        assert_eq!(url_format!("foo", #[query] bar, #[query] baz), "foo?baz=quz");
    }

    #[test]
    fn multi_query_url() {
        let bar = Some("qux");
        let baz = Some("quz");
        assert_eq!(url_format!("foo", #[query] bar, #[query] baz), "foo?bar=qux&baz=quz");
    }

    #[test]
    fn multi_var_and_query_url() {
        let bar = "baara";
        let baz = 0;
        let qar = Some(true);
        let qaz = Some(3);
        assert_eq!(
            url_format!(
                "foo/{bar}/fuu/{baz}",
                bar,
                baz,
                #[query] qar,
                #[query] qaz
            ),
            "foo/baara/fuu/0?qar=true&qaz=3"
        );
    }

    async fn verify_stream<S, F>(f: F) -> anyhow::Result<()>
    where
        S: Stream<Item = std::result::Result<Bytes, Error>> + Unpin + 'static,
        F: Fn(&'static str) -> EventStream<S, Error>,
    {
        let src = r#"
:ping
event: stdout
data: some
data: output
id: 1

:ping

event: stderr
data:
id: 2

event: stdout
data: 0
id

"#;
        let stream = f(src);
        let events = stream.collect::<Vec<_>>().await;

        assert_eq!(events.len(), 4);
        let mut iter = events.into_iter();

        let event = iter.next().unwrap()?;
        assert_eq!(event.event, "stdout".to_string());
        assert_eq!(event.data, "some\noutput".to_string());
        assert_eq!(event.id, Some(1));

        assert!(iter.next().unwrap().is_err());

        let event = iter.next().unwrap()?;
        assert_eq!(event.event, "stderr".to_string());
        assert_eq!(event.data, "".to_string());
        assert_eq!(event.id, Some(2));

        let event = iter.next().unwrap()?;
        assert_eq!(event.event, "stdout".to_string());
        assert_eq!(event.data, "0".to_string());
        assert_eq!(event.id, None);

        Ok(())
    }

    #[actix_rt::test]
    async fn event_stream() {
        verify_stream(|s| {
            let stream = futures::stream::once(async move { Ok::<_, Error>(Bytes::from(s.to_string().into_bytes()))}.boxed_local());
            EventStream::new(stream)
        }).await.unwrap();

        verify_stream(|s| {
            let stream = futures::stream::iter(s.as_bytes()).chunks(5).map(|v| {
                Ok::<_, Error>(Bytes::from(v.iter().map(|b| **b).collect::<Vec<_>>()))
            });
            EventStream::new(stream)
        }).await.unwrap();

        verify_stream(|s| {
            let stream = futures::stream::iter(s.as_bytes()).chunks(1).map(|v| {
                Ok::<_, Error>(Bytes::from(v.iter().map(|b| **b).collect::<Vec<_>>()))
            });
            EventStream::new(stream)
        }).await.unwrap();
    }
}
