use structopt::StructOpt;
use url::Url;

use crate::{activity, market, payment, web::WebClient, web::WebInterface};
use std::convert::TryFrom;

use crate::activity::ACTIVITY_URL_ENV_VAR;
use crate::market::MARKET_URL_ENV_VAR;
use crate::payment::PAYMENT_URL_ENV_VAR;

const YAGNA_APPKEY_ENV_VAR: &str = "YAGNA_APPKEY";

pub trait ApiClient: Clone {
    type Market: WebInterface;
    type Activity: WebInterface;
    type Payment: WebInterface;
}

#[derive(Clone)]
pub struct Api<T: ApiClient> {
    pub market: T::Market,
    pub activity: T::Activity,
    pub payment: T::Payment,
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
    type Payment = payment::PaymentRequestorApi;
}

impl ApiClient for Provider {
    type Market = market::MarketProviderApi;
    type Activity = activity::ActivityProviderApi;
    type Payment = payment::PaymentProviderApi;
}

#[derive(StructOpt, Clone)]
pub struct ApiOpts {
    /// Yagna service application key (for HTTP Bearer authorization)
    #[structopt(long = "app-key", env = YAGNA_APPKEY_ENV_VAR, hide_env_values = true)]
    app_key: String,

    /// Market API URL
    #[structopt(long = "market-url", env = MARKET_URL_ENV_VAR, hide_env_values = true)]
    market_url: Option<Url>,

    /// Activity API URL
    #[structopt(long = "activity-url", env = ACTIVITY_URL_ENV_VAR, hide_env_values = true)]
    activity_url: Option<Url>,

    /// Payment API URL
    #[structopt(long = "payment-url", env = PAYMENT_URL_ENV_VAR, hide_env_values = true)]
    payment_url: Option<Url>,
}

impl<T: ApiClient> TryFrom<&ApiOpts> for Api<T> {
    type Error = crate::Error;

    fn try_from(cli: &ApiOpts) -> Result<Self, Self::Error> {
        let client = WebClient::with_token(&cli.app_key)?;

        Ok(Self {
            market: client.interface_at(cli.market_url.clone())?,
            activity: client.interface_at(cli.activity_url.clone())?,
            payment: client.interface_at(cli.payment_url.clone())?,
        })
    }
}
