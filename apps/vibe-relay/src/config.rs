use std::path::PathBuf;
use vibe_core::{ActorIdentity, StorageKind};

#[derive(Clone, Debug)]
pub(crate) struct RelayConfig {
    pub(crate) public_base_url: String,
    pub(crate) access_token: Option<String>,
    pub(crate) enrollment_token: Option<String>,
    pub(crate) state_file: PathBuf,
    pub(crate) task_bridge_port: u16,
    pub(crate) overlay_bridge_connect_timeout_ms: u64,
    pub(crate) overlay_bridge_start_timeout_ms: u64,
    pub(crate) overlay_bridge_recovery_cooldown_ms: u64,
    pub(crate) overlay_bridge_probe_interval_ms: u64,
    pub(crate) storage_kind: StorageKind,
}

impl RelayConfig {
    pub(crate) fn from_env(bind_host: &str, bind_port: &str) -> Self {
        let public_base_url = resolve_public_base_url(bind_host, bind_port);
        Self {
            public_base_url: public_base_url.clone(),
            access_token: std::env::var("VIBE_RELAY_ACCESS_TOKEN")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            enrollment_token: std::env::var("VIBE_RELAY_ENROLLMENT_TOKEN")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            state_file: resolve_state_file(),
            task_bridge_port: resolve_task_bridge_port(),
            overlay_bridge_connect_timeout_ms: resolve_duration_ms(
                "VIBE_OVERLAY_BRIDGE_CONNECT_TIMEOUT_MS",
                1_500,
            ),
            overlay_bridge_start_timeout_ms: resolve_duration_ms(
                "VIBE_OVERLAY_BRIDGE_START_TIMEOUT_MS",
                3_000,
            ),
            overlay_bridge_recovery_cooldown_ms: resolve_duration_ms(
                "VIBE_OVERLAY_BRIDGE_RECOVERY_COOLDOWN_MS",
                1_500,
            ),
            overlay_bridge_probe_interval_ms: resolve_duration_ms(
                "VIBE_OVERLAY_BRIDGE_PROBE_INTERVAL_MS",
                3_000,
            ),
            storage_kind: resolve_storage_kind(),
        }
    }

    pub(crate) fn default_actor(&self) -> ActorIdentity {
        ActorIdentity::personal_owner()
    }
}

fn resolve_public_base_url(bind_host: &str, bind_port: &str) -> String {
    if let Some(base_url) = std::env::var("VIBE_PUBLIC_RELAY_BASE_URL")
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
    {
        return base_url;
    }

    default_public_base_url(bind_host, bind_port, allow_local_loopback_fallback())
}

fn resolve_task_bridge_port() -> u16 {
    std::env::var("VIBE_AGENT_TASK_BRIDGE_PORT")
        .ok()
        .and_then(|value| value.trim().parse::<u16>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(crate::DEFAULT_TASK_BRIDGE_PORT)
}

fn resolve_duration_ms(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

fn resolve_state_file() -> PathBuf {
    if let Some(path) = std::env::var("VIBE_RELAY_STATE_FILE")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        return PathBuf::from(path);
    }

    default_state_file()
}

fn default_state_file() -> PathBuf {
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let state_file = base.join(".vibe-everywhere").join("relay-state.json");
    if state_file.exists() {
        return state_file;
    }

    let legacy_state_file = base.join(".vibe-remote").join("relay-state.json");
    if legacy_state_file.exists() {
        return legacy_state_file;
    }

    state_file
}

fn resolve_storage_kind() -> StorageKind {
    match std::env::var("VIBE_RELAY_STORAGE_KIND")
        .ok()
        .map(|value| value.trim().to_lowercase())
        .as_deref()
    {
        Some("memory") => StorageKind::Memory,
        Some("external") => StorageKind::External,
        _ => StorageKind::File,
    }
}

fn allow_local_loopback_fallback() -> bool {
    cfg!(debug_assertions)
}

fn default_public_base_url(bind_host: &str, bind_port: &str, allow_local_fallback: bool) -> String {
    if is_wildcard_host(bind_host) {
        if allow_local_fallback {
            return format!("http://127.0.0.1:{bind_port}");
        }

        return String::new();
    }

    format!("http://{bind_host}:{bind_port}")
}

fn is_wildcard_host(value: &str) -> bool {
    matches!(value.trim(), "0.0.0.0" | "::")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_bind_host_requires_explicit_public_origin_outside_dev_mode() {
        assert_eq!(default_public_base_url("0.0.0.0", "8787", false), "");
    }

    #[test]
    fn wildcard_bind_host_keeps_local_dev_fallback_in_dev_mode() {
        assert_eq!(
            default_public_base_url("0.0.0.0", "8787", true),
            "http://127.0.0.1:8787"
        );
    }
}
