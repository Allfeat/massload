//! HTTP Server for massload API.
//!
//! Provides REST endpoints for CSV upload and transformation.
//! Blockchain submission is handled directly by the frontend via @allfeat/client SDK.
//!
//! # API Endpoints
//!
//! | Method | Path              | Description                          |
//! |--------|-------------------|--------------------------------------|
//! | GET    | `/health`         | Health check                         |
//! | POST   | `/api/upload`     | Upload CSV for transformation        |
//! | GET    | `/api/logs`       | SSE stream for real-time logs        |

use axum::{
    extract::Multipart,
    http::{header, Method, StatusCode},
    response::{Json, Sse, sse::Event},
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use serde_json::{json, Value};
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::cors::CorsLayer;

use super::types::{error_response, UploadResponse};
use super::logs::LOG_BROADCASTER;
use crate::transform::pipeline::{transform_bytes, TransformOptions};

/// Start the HTTP server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // CORS permissif pour le développement
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT])
        .expose_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/api/upload", post(upload_csv))
        .route("/api/logs", get(sse_logs))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Massload server running on http://localhost:{}", port);
    println!("   POST /api/upload - Upload CSV file");
    println!("   GET  /api/logs   - SSE log stream");
    println!("   GET  /health     - Health check");
    println!();
    println!("📝 Blockchain submission via frontend SDK (@allfeat/client)");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "massload",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "upload": "POST /api/upload",
            "logs": "GET /api/logs (SSE)"
        }
    }))
}

/// SSE endpoint for real-time log streaming
async fn sse_logs() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = LOG_BROADCASTER.subscribe();
    
    let stream = BroadcastStream::new(rx)
        .filter_map(|result| {
            match result {
                Ok(entry) => {
                    let json = serde_json::to_string(&entry).ok()?;
                    Some(Ok(Event::default().data(json)))
                }
                Err(_) => None,
            }
        });
    
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive")
    )
}

/// Upload CSV endpoint
async fn upload_csv(mut multipart: Multipart) -> Result<Json<UploadResponse>, (StatusCode, Json<Value>)> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, Json(error_response(&format!("Multipart error: {}", e))))
    })? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            file_name = field.file_name().map(|s| s.to_string());
            file_data = Some(field.bytes().await.map_err(|e| {
                (StatusCode::BAD_REQUEST, Json(error_response(&format!("Read error: {}", e))))
            })?.to_vec());
        }
    }

    let bytes = file_data.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(error_response("No file provided")))
    })?;

    println!("\n{}", "=".repeat(70));
    println!("📄 NEW UPLOAD: {} ({} bytes)", 
        file_name.as_deref().unwrap_or("unknown"), 
        bytes.len()
    );
    println!("{}\n", "=".repeat(70));

    let options = TransformOptions::default();
    
    let result = transform_bytes(&bytes, options).await.map_err(|e| {
        eprintln!("❌ Transform error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response(&e.to_string())))
    })?;

    println!("\n{}", "=".repeat(70));
    println!("📊 SUMMARY");
    println!("{}", "=".repeat(70));
    println!("   Flat records:   {}", result.flat.len());
    println!("   Grouped works:  {}", result.grouped.len());
    println!("   Valid:          {}", result.valid_count);
    println!("   Invalid:        {}", result.invalid_count);
    if let Some(ref tid) = result.template_id {
        println!("   Template ID:    {}", tid);
    }
    println!("{}\n", "=".repeat(70));

    let response = UploadResponse::from(result);
    
    Ok(Json(response))
}
