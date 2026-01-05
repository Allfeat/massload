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
    extract::{DefaultBodyLimit, Multipart},
    http::{header, Method, StatusCode},
    response::{Html, Json, Sse, sse::Event},
    routing::{get, post},
    Router,
};
use futures::stream::Stream;
use serde_json::{json, Value};
use std::{convert::Infallible, net::SocketAddr, time::Duration};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::BroadcastStream;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
};

use super::logs::{LOG_BROADCASTER, LogEntry};
use super::types::{error_response, UploadResponse};
use super::excel;
use allfeat_services::{transform_bytes, TransformOptions};
use std::sync::Arc;

/// Start the HTTP server (like faucet: serves frontend + backend API)
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Allfeat Apps Hub");
    println!("Backend: PRIVATE (integrated)");
    println!();
    
    // Install pipeline logger to capture all processing logs
    allfeat_services::set_pipeline_logger(Arc::new(|msg: &str, level| {
        let entry = match level {
            allfeat_services::LogLevel::Info => LogEntry::info(msg),
            allfeat_services::LogLevel::Success => LogEntry::success(msg),
            allfeat_services::LogLevel::Warning => LogEntry::warning(msg),
            allfeat_services::LogLevel::Error => LogEntry::error(msg),
        };
        LOG_BROADCASTER.log(entry);
    }));
    
    // Determine frontend dist path - check multiple locations
    let frontend_paths = [
        "apps/hub/frontend/dist",  // When run from workspace root
        "frontend/dist",            // Legacy path
        "../frontend/dist",         // When run from backend dir
    ];
    
    let frontend_path = frontend_paths
        .iter()
        .find(|p| std::path::Path::new(p).exists())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "apps/hub/frontend/dist".to_string());
    
    println!("📁 Frontend: {} (CSR)", frontend_path);
    
    // CORS permissif pour le développement
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT])
        .expose_headers([header::CONTENT_TYPE]);

    // Create SPA fallback handler - read index.html for all unmatched routes
    let index_path = format!("{}/index.html", frontend_path);
    let index_html: &'static str = Box::leak(
        std::fs::read_to_string(&index_path)
            .unwrap_or_else(|_| String::from("<!DOCTYPE html><html><body>Frontend not built</body></html>"))
            .into_boxed_str()
    );
    
    // SPA fallback service: try static files first, then serve index.html
    let spa_fallback = ServeDir::new(&frontend_path)
        .fallback(get(move || async move { Html(index_html) }));
    
    let app = Router::new()
        // Backend API routes (PRIVATE)
        .route("/health", get(health))
        .route("/api/upload", post(upload_csv))
        .route("/api/logs", get(sse_logs))
        
        // Serve frontend static files with SPA fallback
        .fallback_service(spa_fallback)
        
        // Middleware
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024))  // 5 MB upload limit
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Server ready on http://localhost:{}", port);
    println!("Frontend: http://localhost:{} (CSR)", port);
    println!("Backend API: http://localhost:{}/api/* (PRIVATE)", port);
    println!();
    println!("Blockchain submission via frontend SDK (@allfeat/client)");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint - returns service info
async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "allfeat-hub",
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

    let mut bytes = file_data.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, Json(error_response("No file provided")))
    })?;

    // Start a new log session for this upload
    let session_id = LOG_BROADCASTER.start_session();

    println!("\n{}", "=".repeat(70));
    println!("📄 NEW UPLOAD: {} ({} bytes)", 
        file_name.as_deref().unwrap_or("unknown"), 
        bytes.len()
    );
    println!("   Session ID: {}", session_id);
    println!("{}\n", "=".repeat(70));

    // Detect and convert Excel files to CSV
    if let Some(excel_type) = excel::detect_excel_type(&bytes) {
        LOG_BROADCASTER.log(LogEntry::info(&format!(
            "Detected {:?} file, converting to CSV...", 
            excel_type
        )));
        
        let csv_string = excel::excel_to_csv(&bytes, excel_type).map_err(|e| {
            eprintln!("Excel conversion error: {}", e);
            LOG_BROADCASTER.end_session();
            (StatusCode::BAD_REQUEST, Json(error_response(&format!("Excel conversion failed: {}", e))))
        })?;
        
        bytes = csv_string.into_bytes();
        
        LOG_BROADCASTER.log(LogEntry::success(&format!(
            "Converted to CSV ({} bytes)", 
            bytes.len()
        )));
    }

    let options = TransformOptions::default();
    
    let result = transform_bytes(&bytes, options).await.map_err(|e| {
        eprintln!("Transform error: {}", e);
        LOG_BROADCASTER.end_session();
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response(&e.to_string())))
    })?;

    println!("\n{}", "=".repeat(70));
    println!("SUMMARY");
    println!("{}", "=".repeat(70));
    println!("   Flat records:   {}", result.flat.len());
    println!("   Grouped works:  {}", result.grouped.len());
    println!("   Valid:          {}", result.valid_count);
    println!("   Invalid:        {}", result.invalid_count);
    if let Some(ref tid) = result.template_id {
        println!("   Template ID:    {}", tid);
    }
    println!("{}\n", "=".repeat(70));

    // End the log session
    LOG_BROADCASTER.end_session();

    let response = UploadResponse::from(result);
    
    Ok(Json(response))
}
