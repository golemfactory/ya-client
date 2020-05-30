use serde_json;
use std::{env, thread, time::Duration};
use structopt::StructOpt;
use url::Url;

use ya_client::{
    market::{MarketProviderApi, MarketRequestorApi},
    model::market::{
        proposal::State, AgreementProposal, Demand, Offer, Proposal, ProviderEvent, RequestorEvent,
    },
    web::{WebClient, WebInterface},
    Error, Result,
};

#[derive(StructOpt)]
#[structopt(name = "Market", about = "Market service properties")]
struct Options {
    #[structopt(short, long, default_value = <MarketRequestorApi as WebInterface>::API_URL_ENV_VAR )]
    url: Url,
    #[structopt(long)]
    app_key: Option<String>,
    #[structopt(long, default_value = "info")]
    log_level: String,
}

async fn check_provider_subscriptions(
    client: &MarketProviderApi,
    expected_cnt: usize,
) -> Result<()> {
    let provider_subscriptions = client.get_offers().await?;

    println!(
        "  <=PROVIDER | {} active subscriptions",
        provider_subscriptions.len(),
    );
    assert_eq!(provider_subscriptions.len(), expected_cnt);
    Ok(())
}

async fn check_requestor_subscriptions(
    client: &MarketRequestorApi,
    expected_cnt: usize,
) -> Result<()> {
    let requestor_subscriptions = client.get_demands().await?;

    println!(
        "REQUESTOR=>  | {} active subscriptions",
        requestor_subscriptions.len(),
    );
    assert_eq!(requestor_subscriptions.len(), expected_cnt);
    Ok(())
}

/// compares to given proposals
fn cmp_proposals(p1: &Proposal, p2: &Proposal) {
    if p1 == p2 {
        println!("  <=     =>  | Fetched Offer proposal. Equals this from event");
    } else {
        log::error!(
            "fetched proposal does not equal event {:#?} != {:#?}",
            p1,
            p2
        );
    }
}

//////////////
// PROVIDER //
//////////////
async fn provider_interact(client: MarketProviderApi) -> Result<()> {
    // provider - publish offer
    let offer = Offer::new(serde_json::json!({"zima":"już"}), "(&(lato=nie))".into());
    let provider_subscription_id = client.subscribe(&offer).await?;
    println!(
        "  <=PROVIDER | subscription id: {}",
        provider_subscription_id
    );

    check_provider_subscriptions(&client, 1).await?;

    // provider - get events
    'prov_events: loop {
        let mut provider_events = vec![];
        while provider_events.is_empty() {
            provider_events = client
                .collect(&provider_subscription_id, Some(1.0), Some(2))
                .await?;
            println!("  <=PROVIDER | waiting for events");
            thread::sleep(Duration::from_millis(3000))
        }
        println!("  <=PROVIDER | Yay! Got event(s): {:#?}", provider_events);

        for event in &provider_events {
            match &event {
                // demand proposal received --> respond with an counter offer
                ProviderEvent::ProposalEvent { proposal, .. } => {
                    if proposal.prev_proposal_id.is_none() && proposal.state()? == &State::Draft {
                        log::error!("Draft Proposal but wo prev id: {:#?}", proposal)
                    }

                    let proposal_id = proposal.proposal_id()?;

                    // this is not needed in regular flow; just to illustrate possibility
                    let demand_proposal = client
                        .get_proposal(&provider_subscription_id, &proposal_id)
                        .await?;
                    cmp_proposals(&demand_proposal, proposal);

                    println!("  <=PROVIDER | Huha! Got Demand Proposal. Accepting...");
                    let bespoke_proposal = proposal.counter_offer(offer.clone())?;
                    let new_prop_id = client
                        .counter_proposal(&bespoke_proposal, &provider_subscription_id)
                        .await?;
                    println!(
                        "  <=PROVIDER | Responded with Counter proposal: {}",
                        new_prop_id
                    );
                }
                // provider - agreement proposal received --> approve it
                ProviderEvent::AgreementEvent { agreement, .. } => {
                    let agreement_id = &agreement.agreement_id;
                    println!(
                        "  <=PROVIDER | Wooha! Got new Agreement event {}. Approving...",
                        agreement_id
                    );

                    let status = client.approve_agreement(agreement_id, None).await?;
                    // one can also call:
                    // let res = client.reject_agreement(agreement_id).await?;
                    println!("  <=PROVIDER | Agreement {} by Requestor!", status);

                    println!("  <=PROVIDER | I'm done for now! Bye...");
                    break 'prov_events;
                }
                ProviderEvent::PropertyQueryEvent { .. } => {
                    println!("Unsupported PropertyQueryEvent.");
                }
            }
        }
    }

    println!("  <=PROVIDER | Unsubscribing...");
    let res = client.unsubscribe(&provider_subscription_id).await?;
    println!("  <=PROVIDER | Unsubscribed: {}", res);
    check_provider_subscriptions(&client, 0).await
}

//\\\\\\\\\\\//
// REQUESTOR //
//\\\\\\\\\\\//
async fn requestor_interact(client: MarketRequestorApi) -> Result<()> {
    thread::sleep(Duration::from_millis(300));
    // requestor - publish demand
    let demand = Demand::new(serde_json::json!({"lato":"nie"}), "(&(zima=już))".into());
    let requestor_subscription_id = client.subscribe(&demand).await?;
    println!(
        "REQUESTOR=>  | subscription id: {}",
        requestor_subscription_id
    );

    check_requestor_subscriptions(&client, 1).await?;

    // requestor - get events
    'req_events: loop {
        let mut requestor_events = vec![];
        while requestor_events.is_empty() {
            requestor_events = client
                .collect(&requestor_subscription_id, Some(1.0), Some(2))
                .await?;
            println!("REQUESTOR=>  | waiting for events");
            thread::sleep(Duration::from_millis(3000))
        }
        println!("REQUESTOR=>  | Yay! Got event(s): {:#?}", requestor_events);

        // requestor - support first event
        for event in &requestor_events {
            match &event {
                RequestorEvent::ProposalEvent { proposal, .. } => {
                    let proposal_id = proposal.proposal_id()?;

                    // this is not needed in regular flow; just to illustrate possibility
                    let offer_proposal = client
                        .get_proposal(&requestor_subscription_id, proposal.proposal_id()?)
                        .await?;
                    cmp_proposals(&offer_proposal, proposal);

                    match proposal.state()? {
                        State::Initial => {
                            if proposal.prev_proposal_id.is_some() {
                                log::error!("Initial Proposal but with prev id: {:#?}", proposal);
                            }
                            println!("REQUESTOR=>  | Negotiating proposal...");
                            let bespoke_proposal = proposal.counter_demand(demand.clone())?;
                            let new_proposal_id = client
                                .counter_proposal(&bespoke_proposal, &requestor_subscription_id)
                                .await?;
                            println!(
                                "REQUESTOR=>  | Responded with counter proposal (id: {})",
                                new_proposal_id
                            );
                        }
                        State::Draft => {
                            println!("REQUESTOR=>  | Got draft proposal. Creating agreement...");
                            let new_agreement_id = proposal_id.clone();
                            let agreement =
                                AgreementProposal::new(new_agreement_id, chrono::Utc::now());
                            let id = client.create_agreement(&agreement).await?;
                            println!(
                                "REQUESTOR=>  | agreement created {}: \n{:#?}\nConfirming...",
                                id, &agreement
                            );
                            let res = client.confirm_agreement(&agreement.proposal_id).await?;
                            println!(
                                "REQUESTOR=>  | agreement {} confirmed: {}",
                                &agreement.proposal_id, res
                            );

                            println!("REQUESTOR=>  | Waiting for Agreement approval...");
                            match client.wait_for_approval(&agreement.proposal_id, None).await {
                                Err(Error::TimeoutError { .. }) => {
                                    println!(
                                        "REQUESTOR=>  | Timeout waiting for Agreement approval..."
                                    );
                                    Ok("".into())
                                }
                                Ok(status) => {
                                    println!("REQUESTOR=>  | Agreement {} by Provider!", status);
                                    Ok(status)
                                }
                                e => e,
                            }?;

                            println!("REQUESTOR=>  | I'm done for now! Bye...");
                            break 'req_events;
                        }
                        _ => log::error!("unsupported offer proposal state: {:#?}", proposal),
                    }
                }
                RequestorEvent::PropertyQueryEvent { .. } => {
                    log::error!("Unsupported PropertyQueryEvent.");
                }
            }
        }
    }

    println!("REQUESTOR=>  | Unsunscribing...");
    let res = client.unsubscribe(&requestor_subscription_id).await?;
    println!("REQUESTOR=>  | Unsubscribed: {}", res);
    check_requestor_subscriptions(&client, 0).await
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let options = Options::from_args();
    println!("\nrun this example with RUST_LOG=debug to see REST calls\n");
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or(options.log_level),
    );
    env_logger::init();

    let mut client_builder = WebClient::builder();
    if let Some(app_key) = options.app_key {
        client_builder = client_builder.auth_token(&app_key);
    }
    let client = client_builder.build();

    futures::try_join!(
        provider_interact(client.interface_at(options.url.clone())?),
        requestor_interact(client.interface_at(options.url)?)
    )?;

    Ok(())
}
