mod config;
mod db;
mod handlers;

use axum::{Router, extract::State, routing::get};
use sqlx::migrate::Migrator;
use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
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
    let app_state = AppState { db: pool.clone() };
    let app = Router::new()
        .route("/health", get(handlers::health))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Cleanup after server shuts down
    tracing::info!("Server shutdown complete, closing database connections...");
    pool.close().await;
    tracing::info!("Database connections closed. Shutdown complete.");

    Ok(())
}

/// Waits for SIGTERM (Unix) or SIGINT (Ctrl+C) signals
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::info!("Received Ctrl+C (SIGINT), initiating graceful shutdown");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
        tracing::info!("Received SIGTERM, initiating graceful shutdown");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutdown signal received, waiting for pending requests to complete...");
}