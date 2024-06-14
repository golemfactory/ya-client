use structopt::StructOpt;
use url::Url;

use crate::identity::IDENTITY_URL_ENV_VAR;
use crate::{activity, identity, market, net, payment, web::WebClient, web::WebInterface};
use std::convert::TryFrom;

use crate::activity::ACTIVITY_URL_ENV_VAR;
use crate::market::MARKET_URL_ENV_VAR;
use crate::net::NET_URL_ENV_VAR;
use crate::payment::PAYMENT_URL_ENV_VAR;
use crate::web::{DEFAULT_YAGNA_API_URL, YAGNA_API_URL_ENV_VAR};

const YAGNA_APPKEY_ENV_VAR: &str = "YAGNA_APPKEY";

pub trait ApiClient: Clone {
    type Market: WebInterface;
    type Activity: WebInterface;
    type Payment: WebInterface;
    type Net: WebInterface;
    type Identity: WebInterface;
}

#[derive(Clone)]
pub struct Api<T: ApiClient> {
    pub market: T::Market,
    pub activity: T::Activity,
    pub payment: T::Payment,
    pub net: T::Net,
    pub identity: T::Identity,
}

pub type RequestorApi = Api<Requestor>;
pub type ProviderApi = Api<Provider>;

#[derive(Clone)]
pub struct Requestor;
#[derive(Clone)]
pub struct Provider;

impl ApiClient for Requestor {
    type Market = market::MarketRequestorApi;
    type Activity = activity::ActivityRequestorApi;
    type Payment = payment::PaymentApi;
    type Net = net::NetApi;
    type Identity = identity::IdentityApi;
}

impl ApiClient for Provider {
    type Market = market::MarketProviderApi;
    type Activity = activity::ActivityProviderApi;
    type Payment = payment::PaymentApi;
    type Net = net::NetApi;
    type Identity = identity::IdentityApi;
}

#[derive(StructOpt, Clone)]
#[structopt(rename_all = "kebab-case")]
pub struct ApiOpts {
    /// Yagna service application key (for HTTP Bearer authorization)
    #[structopt(long, env = YAGNA_APPKEY_ENV_VAR, hide_env_values = true)]
    app_key: String,

    /// default prefix URL for all APIs
    #[structopt(
        long,
        env = YAGNA_API_URL_ENV_VAR,
        hide_env_values = true,
        default_value = DEFAULT_YAGNA_API_URL
    )]
    api_url: Url,

    /// Market API URL
    #[structopt(long, env = MARKET_URL_ENV_VAR, hide_env_values = true)]
    market_url: Option<Url>,

    /// Activity API URL
    #[structopt(long, env = ACTIVITY_URL_ENV_VAR, hide_env_values = true)]
    activity_url: Option<Url>,

    /// Payment API URL
    #[structopt(long, env = PAYMENT_URL_ENV_VAR, hide_env_values = true)]
    payment_url: Option<Url>,

    /// Net API URL
    #[structopt(long, env = NET_URL_ENV_VAR, hide_env_values = true)]
    net_url: Option<Url>,

    /// Identity API URL
    #[structopt(long, env = IDENTITY_URL_ENV_VAR, hide_env_values = true)]
    identity_url: Option<Url>,
}

impl<T: ApiClient> TryFrom<&ApiOpts> for Api<T> {
    type Error = crate::Error;

    fn try_from(cli: &ApiOpts) -> Result<Self, Self::Error> {
        let client = WebClient::builder()
            .api_url(cli.api_url.clone())
            .auth_token(&cli.app_key)
            .build();

        Ok(Self {
            market: client.interface_at(cli.market_url.clone())?,
            activity: client.interface_at(cli.activity_url.clone())?,
            payment: client.interface_at(cli.payment_url.clone())?,
            net: client.interface_at(cli.net_url.clone())?,
            identity: client.interface_at(cli.identity_url.clone())?,
        })
    }
}
