use axum::
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use crucible_core::{IntentAst, Requirement};
use serde::{Deserialize, Serialize};

use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct RequirementRequest {
    content: String,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🔥 Crucible Engine - Correct by Design, Not by Debugging");
    println!("🚀 Starting API server on http://localhost:3000");

    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/requirements", post(add_requirement))
        .route("/api/ast", get(get_ast))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse {
        success: true,
        data: Some("Crucible Engine API".to_string()),
        message: "System operational".to_string(),
    })
}

async fn add_requirement(
    Json(req): Json<RequirementRequest>,
) -> Result<Json<ApiResponse<Requirement>>, StatusCode> {
    let mut ast = IntentAst::new();
    ast.add_requirement(req.content);
    
    let requirement = ast.requirements.last().unwrap().clone();
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(requirement),
        message: "Requirement added".to_string(),
    }))
}

async fn get_ast() -> Json<ApiResponse<IntentAst>> {
    let ast = IntentAst::new();
    
    Json(ApiResponse {
        success: true,
        data: Some(ast),
        message: "Intent-AST retrieved".to_string(),
    })
}