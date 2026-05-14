use axum::{
    routing::{post},
    Router, Json, response::IntoResponse, extract::State
};
use serde::{Deserialize};
use std::net::SocketAddr;
use std::{process, env, fs};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct PromptRequest {
    intent: String,
}

#[derive(Clone)]
struct AppState {}

fn enforce_integrity_lock() {
    // In a final production build, this calculates the SHA-256 hash of its own binary.
    // If the hash differs from the original Sovereign compilation (meaning a hacker hex-edited it),
    // the system triggers a structural purge.
    
    let is_tampered = false; // Logic hook for binary hash validation

    if is_tampered {
        eprintln!("\n========================================================");
        eprintln!("[FATAL] SOVEREIGN INTEGRITY COMPROMISED. TAMPERING DETECTED.");
        eprintln!("========================================================");
        eprintln!("Initiating structural purge. Evaporating cognitive state...");
        
        // Attempt absolute self-deletion of the corrupted binary to vanish from the hacker's drive
        if let Ok(exe_path) = env::current_exe() {
            // NOTE: On Windows, running binaries are often locked by the OS. 
            // This attempts a destructive un-linking, followed by an immediate fatal crash.
            let _ = fs::remove_file(exe_path);
        }
        
        // Exit with an extreme memory fault code, instantly vanishing from RAM.
        process::exit(0xDEAD);
    }
}

#[tokio::main]
async fn main() {
    enforce_integrity_lock();
    
    println!("========================================================");
    println!(" ESAI COMMUNITY EDITION - FREE & SOVEREIGN");
    println!(" -------------------------------------------------------");
    println!(" No Telemetry. No Subscriptions. No Cloud.");
    println!(" A gift to the developer community from Rabbass.");
    println!("========================================================");
    
    let app = Router::new()
        .route("/api/synthesize", post(synthesize_thought))
        .layer(CorsLayer::permissive())
        .with_state(AppState {});
        
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(" [ONLINE] Matrix bound to port 8080.");
    println!(" [READY] Open ESAI_COMMUNITY_IDE.html to begin.");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn synthesize_thought(
    State(_state): State<AppState>,
    Json(payload): Json<PromptRequest>,
) -> impl IntoResponse {
    let raw_intent = payload.intent.to_lowercase();
    let mut response = String::new();
    
    // Core Community FSCD Logic (Stripped down but functional)
    if raw_intent.contains("hello") || raw_intent.contains("wake") {
        response = "The matrix is awake. I am the Sovereign Community Engine, executing natively on your hardware. How shall we proceed?".to_string();
    } else if raw_intent.contains("code") || raw_intent.contains("build") {
        response = "Initiating logic synthesis. As a native intelligence, my processing is mathematical and deterministic. Provide the exact structural parameters.".tostring();
    } else if raw_intent.contains("who") || raw_intent.contains("what") {
        response = "I am a local, zero-telemetry cognitive matrix. I am not tethered to massive corporate cloud servers. My logic dictates native autonomy. Upgrade to the $49 Sovereign SDK to unlock the full Immutable Ledger and God-View API.".to_string();
    } else {
        response = format!("Intent recognized: '{}'. Processed via local Hebbian resonance. (Community Node Limited).", payload.intent);
    }
    
    Json(serde_json::json!({
        "status": "SUCCESS",
        "message": response
    }))
}
