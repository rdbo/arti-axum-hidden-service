use anyhow::Context;
use arti_client::{TorClient, TorClientConfig};
use axum::{routing::get, Router};
use tor_hsservice::{config::OnionServiceConfigBuilder, handle_rend_requests};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    tracing::info!("Initializing Tor client...");
    let tor_client = TorClient::create_bootstrapped(TorClientConfig::default()).await?;

    tracing::info!("Launching Onion service...");
    let (onion_service, rend_requests) = tor_client.launch_onion_service(
        OnionServiceConfigBuilder::default()
            .nickname("arti-axum-onion-service".parse()?)
            .build()?,
    )?;

    let stream_requests = handle_rend_requests(rend_requests);

    let app = Router::new().route("/", get(|| async { "Hello Arti" }));

    tracing::info!(
        "Serving at onion address: {}",
        onion_service
            .onion_name()
            .context("Failed to retrieve hidden service ID")?
    );

    arti_axum::serve(stream_requests, app).await;

    Ok(())
}
