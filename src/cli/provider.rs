use super::{Api, ApiClient};
use crate::{activity, market, p2p, payment};

#[derive(Clone)]
pub struct Provider;

pub type ProviderApi = Api<Provider>;

impl ApiClient for Provider {
    type Market = market::MarketProviderApi;
    type Activity = activity::ActivityProviderApi;
    type Payment = payment::PaymentApi;
    type Net = p2p::NetApi;
}
