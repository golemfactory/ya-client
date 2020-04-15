use structopt::StructOpt;
use url::Url;

use crate::{web::WebClient, web::WebInterface, market, activity, payment};
use std::convert::TryFrom;


const ACTIVITY_URL_ENV_VAR:&str = activity::ActivityRequestorApi::API_URL_ENV_VAR;
const MARKET_URL_ENV_VAR:&str = market::MarketRequestorApi::API_URL_ENV_VAR;
const PAYMENT_URL_ENV_VAR:&str = payment::requestor::RequestorApi::API_URL_ENV_VAR;

const YAGNA_APPKEY_ENV_VAR: &str = "YAGNA_APPKEY";

pub trait ApiClient {
    type Market: WebInterface;
    type Activity: WebInterface;
    type Payment: WebInterface;
}

pub struct Api<T: ApiClient> {
    pub market: T::Market,
    pub activity: T::Activity,
    pub payment: T::Payment,
}

pub type RequestorApi = Api<Requestor>;
pub type ProviderApi = Api<Provider>;

pub struct Requestor;
pub struct Provider;

impl ApiClient for Requestor {
    type Market = market::MarketRequestorApi;
    type Activity = activity::ActivityRequestorApi;
    type Payment = payment::requestor::RequestorApi;
}

impl ApiClient for Provider {
    type Market = market::MarketProviderApi;
    type Activity = activity::ActivityProviderApi;
    type Payment = payment::provider::ProviderApi;
}


#[derive(StructOpt)]
pub struct ApiOpts {
    /// Yagna daemon application key
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

impl<T: ApiClient> TryFrom<ApiOpts> for Api<T> {
    type Error = crate::Error;

    fn try_from(cli: ApiOpts) -> Result<Self, Self::Error> {
        let client = WebClient::with_token(&cli.app_key)?;

        Ok(Self {
            market: client.interface_at(cli.market_url)?,
            activity: client.interface_at(cli.activity_url)?,
            payment: client.interface_at(cli.payment_url)?,
        })
    }
}
