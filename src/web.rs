//! Web utils
use awc::{
    error::PayloadError,
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    ClientRequest, ClientResponse, SendClientRequest,
};
use bytes::Bytes;
use futures::Stream;
use serde::{de::DeserializeOwned, Serialize};
use std::{env, rc::Rc, str::FromStr, time::Duration};
use url::{form_urlencoded, Url};

use crate::{Error, Result};

pub const YAGNA_API_URL_ENV_VAR: &str = "YAGNA_API_URL";
pub const DEFAULT_YAGNA_API_URL: &str = "http://127.0.0.1:7465";

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
        Err((response.status(), url, response.json().await).into())
    }
}

impl WebRequest<SendClientRequest> {
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
        let raw_body = response.body().await?;
        let body = std::str::from_utf8(&raw_body)?;
        log::debug!("WebRequest.json(). url={}, resp={}", self.url, body);

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
}
