use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, OnceLock},
};

use axum::{
    Router,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Json, Response},
    routing::get,
};
use log::{error, info};
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

#[derive(Clone)]
pub struct VrServerState {
    pub app_handle: Arc<RwLock<Option<tauri::AppHandle>>>,
    pub static_dir: PathBuf,
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

pub async fn run_vr_server(addr: &str, static_dir: PathBuf) {
    let state = VrServerState {
        app_handle: Arc::new(RwLock::new(None)),
        static_dir,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/settings", get(get_all_settings))
        .route("/api/settings/{overlay}", get(get_overlay_settings))
        .route("/vr", get(serve_vr_page))
        .route("/vr/", get(serve_vr_page))
        .route("/vr/{overlay}", get(serve_vr_overlay_page))
        .fallback(get(serve_static))
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

pub async fn run_vr_server_with_state(addr: &str, static_dir: PathBuf, app_handle: tauri::AppHandle) {
    let state = VrServerState {
        app_handle: Arc::new(RwLock::new(Some(app_handle))),
        static_dir,
    };

    let _ = APP_STATE.set(state.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ports", get(get_ports))
        .route("/api/settings", get(get_all_settings))
        .route("/api/settings/{overlay}", get(get_overlay_settings))
        .route("/vr", get(serve_vr_page))
        .route("/vr/", get(serve_vr_page))
        .route("/vr/{overlay}", get(serve_vr_overlay_page))
        .fallback(get(serve_static))
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

async fn serve_vr_page(State(state): State<VrServerState>) -> impl IntoResponse {
    match serve_file(&state.static_dir, "vr.html").await {
        Ok(response) => response,
        Err(_) => (StatusCode::NOT_FOUND, "VR page not found").into_response(),
    }
}

async fn serve_vr_overlay_page(
    State(state): State<VrServerState>,
    Path(overlay): Path<String>,
) -> impl IntoResponse {
    // Serve the specific overlay page for VR
    let path = format!("overlay/{}/index.html", overlay);
    match serve_file(&state.static_dir, &path).await {
        Ok(response) => response,
        Err(_) => (StatusCode::NOT_FOUND, "Overlay not found").into_response(),
    }
}

async fn serve_static(
    State(state): State<VrServerState>,
    uri: axum::http::Uri,
) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    // Try to serve the file directly
    if let Ok(response) = serve_file(&state.static_dir, path).await {
        return response;
    }

    // If not found and it looks like a route, try serving index.html
    if !path.contains('.') {
        let index_path = format!("{}/index.html", path);
        if let Ok(response) = serve_file(&state.static_dir, &index_path).await {
            return response;
        }
    }

    (StatusCode::NOT_FOUND, "Not found").into_response()
}

async fn serve_file(static_dir: &PathBuf, path: &str) -> Result<Response, StatusCode> {
    let file_path = static_dir.join(path);

    // Security: prevent directory traversal
    if !file_path.starts_with(static_dir) {
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
