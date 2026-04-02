//! Happy-compatible wire fixtures shared across `vibe-wire` tests and downstream crates.

use serde::Serialize;
use serde_json::{Value, json};

const SESSION_START_SUBAGENT: &str = "a1234567890bcdefghijklmn";
const SESSION_STOP_SUBAGENT: &str = "b1234567890bcdefghijklmn";

/// Named JSON sample used to verify cross-crate wire compatibility.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JsonFixture {
    /// Stable fixture identifier for test names and diagnostics.
    pub name: &'static str,
    /// Canonical Happy-compatible JSON payload.
    pub value: Value,
}

/// Named JSON fixture file exported for non-Rust downstream compatibility checks.
#[derive(Clone, Debug, PartialEq)]
pub struct CompatibilityVectorFile {
    /// Stable relative file name used under `crates/vibe-wire/fixtures/`.
    pub file_name: &'static str,
    /// Canonical fixtures written into the file.
    pub fixtures: Vec<JsonFixture>,
}

impl CompatibilityVectorFile {
    fn new(file_name: &'static str, fixtures: Vec<JsonFixture>) -> Self {
        Self {
            file_name,
            fixtures,
        }
    }

    /// Renders the canonical pretty-printed JSON file content.
    pub fn render_json(&self) -> String {
        let mut rendered =
            serde_json::to_string_pretty(&self.fixtures).expect("fixture vectors should serialize");
        rendered.push('\n');
        rendered
    }
}

/// Canonical JSON vector files published for non-Rust consumers.
pub fn compatibility_vector_files() -> Vec<CompatibilityVectorFile> {
    vec![
        CompatibilityVectorFile::new("message-meta.json", message_meta_fixtures()),
        CompatibilityVectorFile::new("legacy-messages.json", legacy_message_fixtures()),
        CompatibilityVectorFile::new("session-envelopes.json", session_envelope_fixtures()),
        CompatibilityVectorFile::new("message-content.json", message_content_fixtures()),
        CompatibilityVectorFile::new("update-containers.json", update_container_fixtures()),
        CompatibilityVectorFile::new("voice-responses.json", voice_response_fixtures()),
        CompatibilityVectorFile::new(
            "session-invalid-envelopes.json",
            invalid_session_envelope_fixtures(),
        ),
    ]
}

/// Canonical `MessageMeta` samples covering sparse and nullable fields.
pub fn message_meta_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "message-meta-full",
            value: json!({
                "sentFrom": "mobile",
                "permissionMode": "bypassPermissions",
                "model": "gpt-5",
                "fallbackModel": null,
                "customSystemPrompt": "system",
                "appendSystemPrompt": null,
                "allowedTools": ["shell", "git"],
                "disallowedTools": null,
                "displayText": "Visible",
            }),
        },
        JsonFixture {
            name: "message-meta-sparse",
            value: json!({
                "sentFrom": "cli",
            }),
        },
        JsonFixture {
            name: "message-meta-custom-permission-mode",
            value: json!({
                "permissionMode": "team-custom-mode",
                "displayText": "Visible",
            }),
        },
    ]
}

/// Canonical legacy decrypted message samples for user and agent roles.
pub fn legacy_message_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "legacy-user-message",
            value: json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": "fix this test",
                },
                "localKey": "local-1",
                "meta": {
                    "sentFrom": "mobile",
                }
            }),
        },
        JsonFixture {
            name: "legacy-agent-message",
            value: json!({
                "role": "agent",
                "content": {
                    "type": "output",
                    "data": {
                        "type": "message",
                        "message": "done",
                    },
                    "unknown": {
                        "nested": true,
                    }
                },
                "meta": {
                    "sentFrom": "cli",
                }
            }),
        },
    ]
}

/// Canonical session envelope samples covering every supported event variant.
pub fn session_envelope_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "session-text-envelope",
            value: json!({
                "id": "msg-text",
                "time": 1,
                "role": "user",
                "ev": {
                    "t": "text",
                    "text": "hello",
                }
            }),
        },
        JsonFixture {
            name: "session-thinking-envelope",
            value: json!({
                "id": "msg-thinking",
                "time": 2,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "text",
                    "text": "thinking",
                    "thinking": true,
                }
            }),
        },
        JsonFixture {
            name: "session-service-envelope",
            value: json!({
                "id": "msg-service",
                "time": 3,
                "role": "agent",
                "ev": {
                    "t": "service",
                    "text": "**Service:** restarting MCP bridge",
                }
            }),
        },
        JsonFixture {
            name: "session-tool-call-start-envelope",
            value: json!({
                "id": "msg-tool-start",
                "time": 4,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "tool-call-start",
                    "call": "call-1",
                    "name": "CodexBash",
                    "title": "Run `ls`",
                    "description": "Run `ls -la` in the repo root",
                    "args": {
                        "command": "ls -la",
                    }
                }
            }),
        },
        JsonFixture {
            name: "session-tool-call-end-envelope",
            value: json!({
                "id": "msg-tool-end",
                "time": 5,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "tool-call-end",
                    "call": "call-1",
                }
            }),
        },
        JsonFixture {
            name: "session-file-envelope",
            value: json!({
                "id": "msg-file",
                "time": 6,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "file",
                    "ref": "upload-1",
                    "name": "report.txt",
                    "size": 1024,
                    "mimeType": "text/plain",
                }
            }),
        },
        JsonFixture {
            name: "session-image-envelope",
            value: json!({
                "id": "msg-image",
                "time": 7,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "file",
                    "ref": "upload-2",
                    "name": "image.png",
                    "size": 2048,
                    "mimeType": "image/png",
                    "image": {
                        "width": 100,
                        "height": 80,
                        "thumbhash": "abc",
                    }
                }
            }),
        },
        JsonFixture {
            name: "session-turn-start-envelope",
            value: json!({
                "id": "msg-turn-start",
                "time": 8,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "turn-start",
                }
            }),
        },
        JsonFixture {
            name: "session-start-envelope",
            value: json!({
                "id": "msg-start",
                "time": 9,
                "role": "agent",
                "subagent": SESSION_START_SUBAGENT,
                "ev": {
                    "t": "start",
                    "title": "Research agent",
                }
            }),
        },
        JsonFixture {
            name: "session-turn-end-envelope",
            value: json!({
                "id": "msg-turn-end",
                "time": 10,
                "role": "agent",
                "turn": "turn-1",
                "ev": {
                    "t": "turn-end",
                    "status": "completed",
                }
            }),
        },
        JsonFixture {
            name: "session-stop-envelope",
            value: json!({
                "id": "msg-stop",
                "time": 11,
                "role": "agent",
                "subagent": SESSION_STOP_SUBAGENT,
                "ev": {
                    "t": "stop",
                }
            }),
        },
    ]
}

/// Canonical decrypted `MessageContent` samples spanning legacy and session-protocol payloads.
pub fn message_content_fixtures() -> Vec<JsonFixture> {
    let legacy = legacy_message_fixtures();

    vec![
        JsonFixture {
            name: "message-content-legacy-user",
            value: legacy[0].value.clone(),
        },
        JsonFixture {
            name: "message-content-legacy-agent",
            value: legacy[1].value.clone(),
        },
        JsonFixture {
            name: "message-content-session-protocol",
            value: session_protocol_message_fixture_value(),
        },
    ]
}

/// Canonical durable update container samples spanning every Happy-exported update body family.
pub fn update_container_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "update-container-new-message",
            value: json!({
                "id": "upd-1",
                "seq": 1,
                "body": {
                    "t": "new-message",
                    "sid": "session-1",
                    "message": encrypted_session_message_fixture_value(
                        "msg-1",
                        10,
                        Value::Null,
                        123,
                        124,
                        "ZmFrZS1lbmNyeXB0ZWQ="
                    ),
                },
                "createdAt": 1,
            }),
        },
        JsonFixture {
            name: "update-container-session-missing-metadata",
            value: json!({
                "id": "upd-2",
                "seq": 2,
                "body": {
                    "t": "update-session",
                    "id": "session-1",
                    "agentState": null,
                },
                "createdAt": 2,
            }),
        },
        JsonFixture {
            name: "update-container-session-versioned-fields",
            value: json!({
                "id": "upd-3",
                "seq": 3,
                "body": {
                    "t": "update-session",
                    "id": "session-1",
                    "metadata": {
                        "version": 2,
                        "value": "abc",
                    },
                    "agentState": {
                        "version": 3,
                        "value": null,
                    }
                },
                "createdAt": 3,
            }),
        },
        JsonFixture {
            name: "update-container-machine-null-metadata",
            value: json!({
                "id": "upd-4",
                "seq": 4,
                "body": {
                    "t": "update-machine",
                    "machineId": "machine-1",
                    "metadata": null,
                },
                "createdAt": 4,
            }),
        },
        JsonFixture {
            name: "update-container-machine-presence-only",
            value: json!({
                "id": "upd-5",
                "seq": 5,
                "body": {
                    "t": "update-machine",
                    "machineId": "machine-2",
                    "active": true,
                    "activeAt": 12345,
                },
                "createdAt": 5,
            }),
        },
    ]
}

/// Canonical voice allow and deny response samples.
pub fn voice_response_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "voice-token-allowed",
            value: json!({
                "allowed": true,
                "token": "token",
                "agentId": "agent-1",
                "elevenUserId": "eleven-1",
                "usedSeconds": 12,
                "limitSeconds": 42,
            }),
        },
        JsonFixture {
            name: "voice-token-denied",
            value: json!({
                "allowed": false,
                "reason": "voice_limit_reached",
                "usedSeconds": 12,
                "limitSeconds": 42,
                "agentId": "agent-1",
            }),
        },
    ]
}

/// Canonical invalid session envelopes used to verify Happy role and subagent constraints.
pub fn invalid_session_envelope_fixtures() -> Vec<JsonFixture> {
    vec![
        JsonFixture {
            name: "session-invalid-service-user-role",
            value: json!({
                "id": "msg-invalid-service",
                "time": 1,
                "role": "user",
                "ev": {
                    "t": "service",
                    "text": "internal event",
                }
            }),
        },
        JsonFixture {
            name: "session-invalid-start-user-role",
            value: json!({
                "id": "msg-invalid-start",
                "time": 1,
                "role": "user",
                "ev": {
                    "t": "start",
                    "title": "Research agent",
                }
            }),
        },
        JsonFixture {
            name: "session-invalid-stop-user-role",
            value: json!({
                "id": "msg-invalid-stop",
                "time": 1,
                "role": "user",
                "ev": {
                    "t": "stop",
                }
            }),
        },
        JsonFixture {
            name: "session-invalid-subagent",
            value: json!({
                "id": "msg-invalid-subagent",
                "time": 1,
                "role": "agent",
                "turn": "turn-1",
                "subagent": "provider-tool-id",
                "ev": {
                    "t": "text",
                    "text": "hello",
                }
            }),
        },
    ]
}

fn session_protocol_message_fixture_value() -> Value {
    json!({
        "role": "session",
        "content": {
            "id": "msg-session",
            "time": 12,
            "role": "agent",
            "turn": "turn-2",
            "ev": {
                "t": "text",
                "text": "hello from session protocol",
            }
        },
        "meta": {
            "sentFrom": "cli",
        }
    })
}

fn encrypted_session_message_fixture_value(
    id: &str,
    seq: u64,
    local_id: Value,
    created_at: u64,
    updated_at: u64,
    ciphertext: &str,
) -> Value {
    json!({
        "id": id,
        "seq": seq,
        "localId": local_id,
        "content": {
            "t": "encrypted",
            "c": ciphertext,
        },
        "createdAt": created_at,
        "updatedAt": updated_at,
    })
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::{
        CoreUpdateContainer, LegacyMessageContent, MessageContent, MessageMeta, SessionEnvelope,
        VoiceTokenResponse, compatibility_vector_files, invalid_session_envelope_fixtures,
        legacy_message_fixtures, message_content_fixtures, message_meta_fixtures,
        session_envelope_fixtures, update_container_fixtures, voice_response_fixtures,
    };

    #[test]
    fn metadata_fixtures_round_trip() {
        for fixture in message_meta_fixtures() {
            let parsed: MessageMeta = serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn legacy_fixtures_round_trip() {
        for fixture in legacy_message_fixtures() {
            let parsed: LegacyMessageContent =
                serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn session_fixtures_round_trip() {
        for fixture in session_envelope_fixtures() {
            let parsed: SessionEnvelope = serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn message_content_fixtures_round_trip() {
        for fixture in message_content_fixtures() {
            let parsed: MessageContent = serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn update_container_fixtures_round_trip() {
        for fixture in update_container_fixtures() {
            let parsed: CoreUpdateContainer =
                serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn invalid_session_fixtures_are_rejected() {
        for fixture in invalid_session_envelope_fixtures() {
            let error =
                serde_json::from_value::<SessionEnvelope>(fixture.value.clone()).unwrap_err();
            assert!(
                error
                    .to_string()
                    .contains(expected_invalid_session_error(fixture.name)),
                "{}",
                fixture.name
            );
        }
    }

    #[test]
    fn voice_fixtures_round_trip() {
        for fixture in voice_response_fixtures() {
            let parsed: VoiceTokenResponse = serde_json::from_value(fixture.value.clone()).unwrap();
            let value = serde_json::to_value(parsed).unwrap();
            assert_eq!(value, fixture.value, "{}", fixture.name);
        }
    }

    #[test]
    fn committed_compatibility_vectors_match_generated_output() {
        let fixtures_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures");

        for vector_file in compatibility_vector_files() {
            let path = fixtures_dir.join(vector_file.file_name);
            let on_disk = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            assert_eq!(
                on_disk,
                vector_file.render_json(),
                "{}",
                vector_file.file_name
            );
        }
    }

    fn expected_invalid_session_error(name: &str) -> &'static str {
        match name {
            "session-invalid-service-user-role" => "service events must use role",
            "session-invalid-start-user-role" => "start events must use role",
            "session-invalid-stop-user-role" => "stop events must use role",
            "session-invalid-subagent" => "subagent must be a cuid2 value",
            _ => panic!("unknown invalid fixture {name}"),
        }
    }
}
