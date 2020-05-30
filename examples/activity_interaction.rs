use std::env;

use ya_client::{
    activity::{ActivityProviderApi, ActivityRequestorControlApi, ActivityRequestorStateApi},
    model::activity::ExeScriptRequest,
    web::WebClient,
    Result,
};

async fn provider(client: &ActivityProviderApi, activity_id: &str) -> Result<()> {
    println!("[?] Events for activity {}", activity_id);
    let activity_events = client.get_activity_events(Some(60.), None).await.unwrap();
    println!("[<] Events: {:?}", activity_events);

    println!("[+] Activity state");
    let activity_state = client.get_activity_state(activity_id).await.unwrap();
    println!("[<] {:?}", activity_state);

    println!("[+] Activity usage");
    let activity_usage = client.get_activity_usage(activity_id).await.unwrap();
    println!("[<] {:?}", activity_usage);
    Ok(())
}

async fn requestor(client: &WebClient, agreement_id: &str) -> Result<()> {
    let activity_id = requestor_start(&client.interface()?, agreement_id).await?;
    requestor_exec(&client.interface()?, &activity_id).await?;
    requestor_state(&client.interface()?, &activity_id).await?;
    requestor_stop(&client.interface()?, &activity_id).await
}

async fn requestor_start(
    client: &ActivityRequestorControlApi,
    agreement_id: &str,
) -> Result<String> {
    println!("[+] Activity, agreement {}", agreement_id);
    let activity_id = client.create_activity(agreement_id).await?;
    println!("[<] Activity: {}", activity_id);

    Ok(activity_id)
}

async fn requestor_stop(client: &ActivityRequestorControlApi, activity_id: &str) -> Result<()> {
    println!("[-] Activity {}", activity_id);
    client.destroy_activity(&activity_id).await?;
    println!("[<] Destroyed");
    Ok(())
}

async fn requestor_exec(client: &ActivityRequestorControlApi, activity_id: &str) -> Result<()> {
    let exe_request = ExeScriptRequest::new("STOP".to_string());
    println!("[+] Batch exe script:{:?}", exe_request);
    let batch_id = client.exec(exe_request, &activity_id).await?;
    println!("[<] Batch id: {}", batch_id);

    println!("[?] Batch results for activity {}", activity_id);
    let results = client
        .get_exec_batch_results(&activity_id, &batch_id, Some(10.), None)
        .await?;
    println!("[<] Batch results: {:?}", results);
    Ok(())
}

async fn requestor_state(client: &ActivityRequestorStateApi, activity_id: &str) -> Result<()> {
    println!("[?] State for activity {}", activity_id);
    let state = client.get_state(activity_id).await?;
    println!("[<] State: {:?}", state);

    println!("[?] Usage vector for activity {}", activity_id);
    let usage = client.get_usage(activity_id).await?;
    println!("[<] Usage vector: {:?}", usage);

    println!("[?] Command state for activity {}", activity_id);
    let command_state = client.get_running_command(activity_id).await?;
    println!("[<] Command state: {:?}", command_state);
    Ok(())
}

async fn interact() -> Result<()> {
    let client = WebClient::builder().build();
    requestor(&client, "agreement_id").await?;
    provider(&client.interface()?, "activity_id").await
}

#[actix_rt::main]
async fn main() -> Result<()> {
    println!("\nrun this example with RUST_LOG=info to see REST calls\n");
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or("warn".into()));
    env_logger::init();

    interact().await
}
