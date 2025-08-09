use anyhow::Result;
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use omnivore_core::{CrawlConfig, CrawlStats};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

#[derive(Default)]
struct Query;

#[Object]
impl Query {
    async fn health(&self) -> &str {
        "OK"
    }

    async fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }
}

type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
struct AppState {
    crawler_stats: Arc<RwLock<Option<CrawlStats>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info,tower_http=debug")
        .init();

    let schema = Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish();

    let state = AppState {
        crawler_stats: Arc::new(RwLock::new(None)),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/crawl", post(start_crawl))
        .route("/api/stats", get(get_stats))
        .route("/graphql", post(graphql_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
        .layer(axum::extract::Extension(schema));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    
    tracing::info!("🚀 Omnivore API server running at http://0.0.0.0:3000");
    tracing::info!("📊 GraphQL playground available at http://0.0.0.0:3000/graphql");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "Omnivore API",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health": "/health",
            "crawl": "/api/crawl",
            "stats": "/api/stats",
            "graphql": "/graphql"
        }
    }))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

#[derive(Deserialize)]
struct CrawlRequest {
    url: String,
    max_depth: Option<u32>,
    max_workers: Option<usize>,
}

#[derive(Serialize)]
struct CrawlResponse {
    id: String,
    status: String,
    message: String,
}

async fn start_crawl(
    State(state): State<AppState>,
    Json(payload): Json<CrawlRequest>,
) -> impl IntoResponse {
    let crawl_id = uuid::Uuid::new_v4().to_string();
    let url_clone = payload.url.clone();

    tokio::spawn(async move {
        let config = CrawlConfig {
            max_depth: payload.max_depth.unwrap_or(5),
            max_workers: payload.max_workers.unwrap_or(10),
            ..Default::default()
        };

        match url::Url::parse(&url_clone) {
            Ok(url) => {
                match omnivore_core::crawler::Crawler::new(config).await {
                    Ok(crawler) => {
                        let crawler = Arc::new(crawler);
                        let _ = crawler.add_seed(url).await;
                        let crawler_clone = crawler.clone();
                        tokio::spawn(async move {
                            let _ = crawler_clone.start().await;
                            let stats = crawler_clone.get_stats().await;
                            let mut stats_lock = state.crawler_stats.write().await;
                            *stats_lock = Some(stats);
                        });
                    }
                    Err(e) => {
                        tracing::error!("Failed to create crawler: {}", e);
                    }
                }
            }
            Err(e) => {
                tracing::error!("Invalid URL: {}", e);
            }
        }
    });

    Json(CrawlResponse {
        id: crawl_id,
        status: "started".to_string(),
        message: format!("Crawl started for URL: {}", payload.url),
    })
}

async fn get_stats(State(state): State<AppState>) -> impl IntoResponse {
    let stats = state.crawler_stats.read().await;
    
    match &*stats {
        Some(s) => Json(serde_json::json!({
            "status": "completed",
            "stats": s
        })),
        None => Json(serde_json::json!({
            "status": "no_data",
            "message": "No crawl statistics available"
        })),
    }
}

async fn graphql_handler(
    schema: axum::extract::Extension<ApiSchema>,
    req: async_graphql_axum::GraphQLRequest,
) -> async_graphql_axum::GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}