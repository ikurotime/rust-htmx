use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{Router,routing::{get, post} };
mod endpoints;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("hello, web server!");   
    let assets_path = std::env::current_dir().unwrap();

    let app = Router::new()
        .route("/", get(endpoints::get_index))
        .route("/page-two", get(endpoints::get_page_two))
        .route("/create", post(endpoints::post_data)).nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("router initialized, now listening on port {}", 3000);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

