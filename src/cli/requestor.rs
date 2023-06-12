use super::{Api, ApiClient};
use crate::{activity, market, net, payment};
pub type RequestorApi = Api<Requestor>;

#[derive(Clone)]
pub struct Requestor;

impl ApiClient for Requestor {
    type Market = market::MarketRequestorApi;
    type Activity = activity::ActivityRequestorApi;
    type Payment = payment::PaymentApi;
    type Net = net::NetVpnApi;
}
