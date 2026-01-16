use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header, Request},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use log::{error, info, warn};
use serde::Serialize;
use tauri::Manager;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    settings::overlays::{
        lap_times::LapTimesOverlaySettings,
        main::MainOverlaySettings,
        proximity::ProximityOverlaySettings,
        relative::RelativeOverlaySettings,
        standings::StandingsOverlaySettings,
        subtimer::SubTimerOverlaySettings,
        telemetry::TelemetryOverlaySettings,
        telemetry_reference::TelemetryReferenceOverlaySettings,
        timer::TimerOverlaySettings,
        track_map::TrackMapOverlaySettings,
    },
    util::settings_helper::get_settings,
    websocket::WebSocketServer,
};

static VR_SERVER_PORT: OnceLock<u16> = OnceLock::new();

// Default Vite dev server URL
const VITE_DEV_SERVER: &str = "http://localhost:5173";

#[derive(Clone)]
pub struct VrServerState {
    pub app_handle: Arc<RwLock<Option<tauri::AppHandle>>>,
    pub static_dir: PathBuf,
    pub dev_mode: bool,
    pub vite_url: String,
}

#[derive(Serialize)]
struct PortsResponse {
    ws_port: Option<u16>,
    http_port: Option<u16>,
}

#[derive(Serialize)]
struct AllSettingsResponse {
    main: Option<MainOverlaySettings>,
    timer: Option<TimerOverlaySettings>,
    subtimer: Option<SubTimerOverlaySettings>,
    standings: Option<StandingsOverlaySettings>,
    lap_times: Option<LapTimesOverlaySettings>,
    proximity: Option<ProximityOverlaySettings>,
    relative: Option<RelativeOverlaySettings>,
    telemetry: Option<TelemetryOverlaySettings>,
    telemetry_reference: Option<TelemetryReferenceOverlaySettings>,
    track_map: Option<TrackMapOverlaySettings>,
}

pub fn get_vr_server_port() -> Option<u16> {
    VR_SERVER_PORT.get().copied()
}

pub async fn run_vr_server(addr: &str, static_dir: PathBuf, dev_mode: bool) {
    let state = VrServerState {
        app_handle: Arc::new(RwLock::new(None)),
        static_dir,
        dev_mode,
        vite_url: VITE_DEV_SERVER.to_string(),
    };

    if dev_mode {
        info!("VR server running in development mode, proxying to {}", VITE_DEV_SERVER);
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/settings", get(get_all_settings))
        .route("/api/settings/{overlay}", get(get_overlay_settings))
        .fallback(get(serve_request))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind VR server");
    let port = listener.local_addr().unwrap().port();
    VR_SERVER_PORT.set(port).expect("Failed to set VR server port");

    info!("VR HTTP server listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to run VR server");
}

pub fn set_app_handle(app_handle: tauri::AppHandle) {
    // This will be called after the server starts
    // We use a separate mechanism to set the app handle
    if let Some(state) = APP_STATE.get() {
        let state = state.clone();
        tokio::spawn(async move {
            let mut handle = state.app_handle.write().await;
            *handle = Some(app_handle);
        });
    }
}

static APP_STATE: OnceLock<VrServerState> = OnceLock::new();

pub async fn run_vr_server_with_state(addr: &str, static_dir: PathBuf, app_handle: tauri::AppHandle, dev_mode: bool) {
    let state = VrServerState {
        app_handle: Arc::new(RwLock::new(Some(app_handle))),
        static_dir,
        dev_mode,
        vite_url: VITE_DEV_SERVER.to_string(),
    };

    let _ = APP_STATE.set(state.clone());

    if dev_mode {
        info!("VR server running in development mode, proxying to {}", VITE_DEV_SERVER);
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/settings", get(get_all_settings))
        .route("/api/settings/{overlay}", get(get_overlay_settings))
        .fallback(get(serve_request))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind VR server");
    let port = listener.local_addr().unwrap().port();
    VR_SERVER_PORT.set(port).expect("Failed to set VR server port");

    info!("VR HTTP server listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed to run VR server");
}

async fn get_ports() -> Json<PortsResponse> {
    Json(PortsResponse {
        ws_port: WebSocketServer::get_port(),
        http_port: get_vr_server_port(),
    })
}

async fn get_all_settings(State(state): State<VrServerState>) -> impl IntoResponse {
    let app_handle = state.app_handle.read().await;

    if let Some(app) = app_handle.as_ref() {
        let response = AllSettingsResponse {
            main: Some(get_settings(app.clone(), "main")),
            timer: Some(get_settings(app.clone(), "timer")),
            subtimer: Some(get_settings(app.clone(), "subtimer")),
            standings: Some(get_settings(app.clone(), "standings")),
            lap_times: Some(get_settings(app.clone(), "lap_times")),
            proximity: Some(get_settings(app.clone(), "proximity")),
            relative: Some(get_settings(app.clone(), "relative")),
            telemetry: Some(get_settings(app.clone(), "telemetry")),
            telemetry_reference: Some(get_settings(app.clone(), "telemetry_reference")),
            track_map: Some(get_settings(app.clone(), "track_map")),
        };
        Json(response).into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "App not ready").into_response()
    }
}

async fn get_overlay_settings(
    State(state): State<VrServerState>,
    Path(overlay): Path<String>,
) -> impl IntoResponse {
    let app_handle = state.app_handle.read().await;

    if let Some(app) = app_handle.as_ref() {
        match overlay.as_str() {
            "main" => Json(get_settings::<MainOverlaySettings>(app.clone(), "main")).into_response(),
            "timer" => Json(get_settings::<TimerOverlaySettings>(app.clone(), "timer")).into_response(),
            "subtimer" => Json(get_settings::<SubTimerOverlaySettings>(app.clone(), "subtimer")).into_response(),
            "standings" => Json(get_settings::<StandingsOverlaySettings>(app.clone(), "standings")).into_response(),
            "lap_times" => Json(get_settings::<LapTimesOverlaySettings>(app.clone(), "lap_times")).into_response(),
            "proximity" => Json(get_settings::<ProximityOverlaySettings>(app.clone(), "proximity")).into_response(),
            "relative" => Json(get_settings::<RelativeOverlaySettings>(app.clone(), "relative")).into_response(),
            "telemetry" => Json(get_settings::<TelemetryOverlaySettings>(app.clone(), "telemetry")).into_response(),
            "telemetry_reference" => Json(get_settings::<TelemetryReferenceOverlaySettings>(app.clone(), "telemetry_reference")).into_response(),
            "track_map" => Json(get_settings::<TrackMapOverlaySettings>(app.clone(), "track_map")).into_response(),
            _ => (StatusCode::NOT_FOUND, "Overlay not found").into_response(),
        }
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "App not ready").into_response()
    }
}

/// Unified request handler that either proxies to Vite (dev mode) or serves static files (production)
async fn serve_request(
    State(state): State<VrServerState>,
    uri: axum::http::Uri,
) -> impl IntoResponse {
    if state.dev_mode {
        // In dev mode, proxy to Vite dev server
        proxy_to_vite(&state.vite_url, uri.path()).await
    } else {
        // In production, serve static files
        serve_static_file(&state.static_dir, uri.path()).await
    }
}

/// Proxy request to Vite dev server
async fn proxy_to_vite(vite_url: &str, path: &str) -> Response {
    let url = format!("{}{}", vite_url, path);

    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            error!("Failed to create HTTP client: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create HTTP client").into_response();
        }
    };

    match client.get(&url).send().await {
        Ok(response) => {
            let status = StatusCode::from_u16(response.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            // Get content-type from response
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();

            match response.bytes().await {
                Ok(body) => {
                    (
                        status,
                        [(header::CONTENT_TYPE, content_type)],
                        body.to_vec()
                    ).into_response()
                }
                Err(e) => {
                    error!("Failed to read response body from Vite: {}", e);
                    (StatusCode::BAD_GATEWAY, "Failed to read response from dev server").into_response()
                }
            }
        }
        Err(e) => {
            warn!("Failed to proxy request to Vite dev server at {}: {}", url, e);
            (
                StatusCode::BAD_GATEWAY,
                format!("Vite dev server not available. Make sure 'pnpm dev' is running. Error: {}", e)
            ).into_response()
        }
    }
}

/// Serve static files from the build directory (production mode)
async fn serve_static_file(static_dir: &PathBuf, path: &str) -> Response {
    let path = path.trim_start_matches('/');

    // Try to serve the file directly
    if let Ok(response) = read_file(static_dir, path).await {
        return response;
    }

    // If not found and it looks like a route (no file extension), try serving index.html
    if !path.contains('.') {
        // Try path/index.html first
        let index_path = if path.is_empty() {
            "index.html".to_string()
        } else {
            format!("{}/index.html", path)
        };

        if let Ok(response) = read_file(static_dir, &index_path).await {
            return response;
        }

        // For SPA routes like /vr, try vr.html
        let html_path = format!("{}.html", path);
        if let Ok(response) = read_file(static_dir, &html_path).await {
            return response;
        }
    }

    (StatusCode::NOT_FOUND, "Not found").into_response()
}

/// Read a file from the static directory
async fn read_file(static_dir: &PathBuf, path: &str) -> Result<Response, StatusCode> {
    let file_path = static_dir.join(path);

    // Security: prevent directory traversal
    let canonical_static = static_dir.canonicalize().unwrap_or_else(|_| static_dir.clone());
    let canonical_file = match file_path.canonicalize() {
        Ok(p) => p,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    if !canonical_file.starts_with(&canonical_static) {
        return Err(StatusCode::FORBIDDEN);
    }

    match tokio::fs::read(&file_path).await {
        Ok(contents) => {
            let mime = mime_guess::from_path(&file_path)
                .first_or_octet_stream()
                .to_string();

            Ok((
                [(header::CONTENT_TYPE, mime)],
                contents
            ).into_response())
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
