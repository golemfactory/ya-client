use std::{env, str::FromStr};

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use structopt::StructOpt;
use url::Url;
use ya_client::{
    payment::PaymentApi,
    web::{WebClient, WebInterface},
    Result,
};
use ya_client_model::payment::allocation::PaymentPlatformEnum;
use ya_client_model::payment::{Allocation, AllocationUpdate, NewAllocation};

#[derive(Clone, StructOpt)]
#[structopt(name = "Market", about = "Market service properties")]
struct Options {
    #[structopt(short, long, default_value = <PaymentApi as WebInterface>::API_URL_ENV_VAR )]
    url: Url,
    #[structopt(long)]
    app_key: String,
    #[structopt(long, default_value = "info")]
    log_level: String,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Clone, StructOpt)]
enum Command {
    List,
    Get {
        #[structopt(long)]
        id: String,
    },
    Amend {
        #[structopt(long)]
        id: String,
        #[structopt(long)]
        budget: String,
    },
    Create {
        #[structopt(long)]
        budget: String,
        #[structopt(long, default_value = "erc20-goerli-tglm")]
        platform: String,
    },
}

fn print_allocations<'a>(allocations: impl IntoIterator<Item = &'a Allocation>) {
    let s = " ".repeat(10);
    println!("{s} allocation - id {s}| total amount");
    println!("{}+{}", "-".repeat(17 + 2 * 10), "-".repeat(14));
    for allocation in allocations {
        println!("{} | {}", allocation.allocation_id, allocation.total_amount);
    }
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let options = Options::from_args();
    println!("\nrun this example with RUST_LOG=debug to see REST calls\n");
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or(options.log_level.clone()),
    );
    env_logger::init();

    let client: PaymentApi = WebClient::with_token(&options.app_key).interface_at(options.url)?;

    match options.command {
        Command::List => {
            let allocations = client
                .get_allocations(Option::<DateTime<Utc>>::None, None)
                .await?;
            print_allocations(&allocations);
        }
        Command::Get { id } => {
            let allocation = client.get_allocation(&id).await?;
            println!("{:#?}", allocation);
        }
        Command::Amend { id, budget } => {
            let new_allocation = AllocationUpdate {
                total_amount: Some(BigDecimal::from_str(&budget).unwrap()),
                timeout: None,
            };

            let allocation = client.amend_allocation(&id, &new_allocation).await?;
            println!("{:#?}", allocation);
        }
        Command::Create { budget, platform } => {
            let allocation = client
                .create_allocation(&NewAllocation {
                    total_amount: BigDecimal::from_str(&budget).unwrap(),
                    make_deposit: true,
                    address: None,
                    payment_platform: Some(PaymentPlatformEnum::PaymentPlatformName(platform)),
                    timeout: None,
                })
                .await?;
            println!("{:#?}", allocation);
        }
    }

    Ok(())
}
