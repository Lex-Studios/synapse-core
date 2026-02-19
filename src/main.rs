mod config;
mod db;
mod handlers;
mod middleware;

use axum::{Router, routing::{get, post}};
use middleware::ip_filter::IpFilterLayer;
use sqlx::migrate::Migrator; // for Migrator
use std::net::SocketAddr; // for SocketAddr
use std::path::Path; // for Path
use tokio::net::TcpListener; // for TcpListener
use tracing_subscriber::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt}; // for .with() on registry

#[derive(Clone)] // <-- Add Clone
pub struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::Config::from_env()?;

    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database pool
    let pool = db::create_pool(&config).await?;

    // Run migrations
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    tracing::info!("Database migrations completed");

    // Build router with state
    let app_state = AppState { db: pool };
    let callback_routes = Router::new()
        .route("/transaction", post(handlers::callback_transaction))
        .layer(IpFilterLayer::new(
            config.allowed_ips.clone(),
            config.trusted_proxy_depth,
        ));

    let app = Router::new()
        .route("/health", get(handlers::health))
        .nest("/callback", callback_routes)
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;

    Ok(())
}

