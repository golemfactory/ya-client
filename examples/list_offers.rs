use anyhow::Result;
use ya_client::market::MarketRequestorApi;
use ya_client_model::market::scan::{NewScan, ScanType};

#[actix_rt::main]
async fn main() -> Result<()> {
    let client =
        ya_client::web::WebClient::with_token(&std::env::var("YAGNA_APPKEY").unwrap_or_default());
    let market: MarketRequestorApi = client.interface()?;

    let it = market
        .begin_scan(&NewScan {
            timeout: Some(30),
            scan_type: ScanType::Offer,
            constraints: Some("(golem.runtime.capabilities=!exp:gpu)".into()),
        })
        .await?;

    let mut idx = 0;
    loop {
        let offers = market.collect_scan(&it, None, None, None).await?;
        if offers.is_empty() {
            break;
        }

        for offer in offers {
            idx += 1;
            let m = offer.properties.as_object().unwrap();
            let name = m
                .get("golem.!exp.gap-35.v1.inf.gpu.model")
                .unwrap()
                .as_str()
                .unwrap();
            let rt_name = m.get("golem.runtime.name").unwrap().as_str().unwrap();
            eprintln!(
                "{} {} {:3} = {}\t{}",
                offer.offer_id, offer.provider_id, idx, name, rt_name
            );
        }
    }
    market.end_scan(&it).await?;

    Ok(())
}
