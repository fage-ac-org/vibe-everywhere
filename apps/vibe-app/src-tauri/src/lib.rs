use vibe_core::{AppConfig, default_app_config};

#[tauri::command]
fn app_config() -> AppConfig {
    let default_relay_base_url = std::env::var("VIBE_PUBLIC_RELAY_BASE_URL")
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(default_relay_base_url);
    let requires_auth = std::env::var("VIBE_RELAY_ACCESS_TOKEN")
        .ok()
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false);
    default_app_config(default_relay_base_url, requires_auth)
}

fn default_relay_base_url() -> String {
    if cfg!(target_os = "android") || cfg!(target_os = "ios") {
        String::new()
    } else {
        "http://127.0.0.1:8787".to_string()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![app_config])
        .run(tauri::generate_context!())
        .expect("failed to run tauri app");
}
