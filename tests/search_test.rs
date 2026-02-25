use reqwest::StatusCode;
use synapse_core::db::pool_manager::PoolManager;
use synapse_core::handlers::search::search_transactions;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tokio::net::TcpListener;

async fn setup_test_app() -> (String, impl std::any::Any) {
    let container = Postgres::default().start().await.unwrap();
    let host_port = container.get_host_port_ipv4(5432).await.unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        host_port
    );
    let pool_manager = PoolManager::new(&database_url, None).await.unwrap();

    let app = axum::Router::new()
        .route(
            "/transactions/search",
            axum::routing::get(search_transactions),
        )
        .with_state(pool_manager);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let std_listener = listener.into_std().unwrap();

    tokio::spawn(async move {
        axum::Server::from_tcp(std_listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    (format!("http://{}", addr), container)
}

#[tokio::test]
async fn test_search_endpoint_not_implemented() {
    let (base_url, _container) = setup_test_app().await;
    let response = reqwest::get(format!("{}/transactions/search", base_url))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
}

#[tokio::test]
async fn test_search_endpoint_accepts_query_params() {
    let (base_url, _container) = setup_test_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/transactions/search", base_url))
        .query(&[
            ("status", "completed"),
            ("asset_code", "USD"),
            ("limit", "10"),
        ])
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
}
