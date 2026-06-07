use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;
mod models;
mod router;
mod services;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env())
    .unwrap_or_else(|_| "metamask_mission_control=debug, tower_http=debug".into())
    .with(tracing_subscriber::fmt::layer())
    .init();

    let cfg = config::Config::from_env()?;
    let app_state = state::AppState::new(cfg).await?;
    let app = router::build(app_state);


    let addr = "0.0.0.0:8080";
    tracing::info!("Mission Control online → http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
    





}