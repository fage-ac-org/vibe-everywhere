use serde::{Deserialize, Serialize};

use crate::{
    legacy_protocol::{AgentMessage, UserMessage},
    message_meta::MessageMeta,
    session_protocol::SessionEnvelope,
};

/// Fixed content discriminator for encrypted session message bodies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionMessageContentType {
    #[serde(rename = "encrypted")]
    Encrypted,
}

/// Encrypted session message payload stored by the server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionMessageContent {
    /// Base64-encoded encrypted content.
    #[serde(rename = "c")]
    pub ciphertext: String,
    /// Fixed content discriminator.
    #[serde(rename = "t")]
    pub kind: SessionMessageContentType,
}

impl SessionMessageContent {
    pub fn new(ciphertext: impl Into<String>) -> Self {
        Self {
            ciphertext: ciphertext.into(),
            kind: SessionMessageContentType::Encrypted,
        }
    }
}

/// Durable session message container shared across server, app, agent, and CLI.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionMessage {
    /// Durable message identifier.
    pub id: String,
    /// Session-local ordering counter.
    pub seq: u64,
    /// Optional client-local optimistic id; explicit `null` is preserved when present.
    #[serde(
        rename = "localId",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_id: Option<Option<String>>,
    /// Encrypted message content wrapper.
    pub content: SessionMessageContent,
    /// Creation timestamp in milliseconds.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    /// Last update timestamp in milliseconds.
    #[serde(rename = "updatedAt")]
    pub updated_at: u64,
}

/// Fixed outer role for session-protocol decrypted payloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionProtocolMessageRole {
    #[serde(rename = "session")]
    Session,
}

/// Decrypted session-protocol message wrapper with `role: "session"`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionProtocolMessage {
    /// Fixed discriminator used by the top-level message union.
    pub role: SessionProtocolMessageRole,
    /// Canonical session envelope payload.
    pub content: SessionEnvelope,
    /// Optional shared message metadata.
    #[serde(
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<MessageMeta>,
}

impl SessionProtocolMessage {
    pub fn new(content: SessionEnvelope) -> Self {
        Self {
            role: SessionProtocolMessageRole::Session,
            content,
            meta: None,
        }
    }
}

/// Top-level decrypted message family covering legacy and session-protocol payloads.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    User(UserMessage),
    Agent(AgentMessage),
    Session(SessionProtocolMessage),
}

/// Versioned encrypted field wrapper for required string values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedEncryptedValue {
    /// Monotonic version number for the encrypted field.
    pub version: u64,
    /// Base64-encoded encrypted value.
    pub value: String,
}

/// Versioned encrypted field wrapper for nullable string values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedNullableEncryptedValue {
    /// Monotonic version number for the encrypted field.
    pub version: u64,
    /// Base64-encoded encrypted value or explicit `null` when the field is cleared.
    pub value: Option<String>,
}

/// Versioned encrypted field wrapper dedicated to machine records.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionedMachineEncryptedValue {
    /// Monotonic version number for the encrypted field.
    pub version: u64,
    /// Base64-encoded encrypted value.
    pub value: String,
}

/// Fixed discriminator for new-message durable updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateNewMessageType {
    #[serde(rename = "new-message")]
    NewMessage,
}

/// Durable update body carrying a new encrypted session message.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateNewMessageBody {
    /// Fixed update discriminator.
    #[serde(rename = "t")]
    pub kind: UpdateNewMessageType,
    /// Owning session identifier for `message`.
    #[serde(rename = "sid")]
    pub session_id: String,
    /// Newly stored session message.
    pub message: SessionMessage,
}

impl UpdateNewMessageBody {
    pub fn new(session_id: impl Into<String>, message: SessionMessage) -> Self {
        Self {
            kind: UpdateNewMessageType::NewMessage,
            session_id: session_id.into(),
            message,
        }
    }
}

/// Fixed discriminator for session metadata/state updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateSessionType {
    #[serde(rename = "update-session")]
    UpdateSession,
}

/// Durable update body carrying encrypted session-level fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateSessionBody {
    /// Fixed update discriminator.
    #[serde(rename = "t")]
    pub kind: UpdateSessionType,
    /// Session identifier being updated.
    pub id: String,
    /// Updated encrypted session metadata; explicit `null` is preserved when present.
    #[serde(
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<Option<VersionedEncryptedValue>>,
    /// Updated encrypted agent state; explicit `null` is preserved when present.
    #[serde(
        rename = "agentState",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_state: Option<Option<VersionedNullableEncryptedValue>>,
}

impl UpdateSessionBody {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            kind: UpdateSessionType::UpdateSession,
            id: id.into(),
            metadata: None,
            agent_state: None,
        }
    }
}

/// Fixed discriminator for machine metadata/state updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateMachineType {
    #[serde(rename = "update-machine")]
    UpdateMachine,
}

/// Durable update body carrying encrypted machine-level fields.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateMachineBody {
    /// Fixed update discriminator.
    #[serde(rename = "t")]
    pub kind: UpdateMachineType,
    /// Machine identifier being updated.
    #[serde(rename = "machineId")]
    pub machine_id: String,
    /// Updated encrypted machine metadata; explicit `null` is preserved when present.
    #[serde(
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata: Option<Option<VersionedMachineEncryptedValue>>,
    /// Updated encrypted daemon state; explicit `null` is preserved when present.
    #[serde(
        rename = "daemonState",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub daemon_state: Option<Option<VersionedMachineEncryptedValue>>,
    /// Optional activity flag emitted for presence-only updates.
    #[serde(
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub active: Option<bool>,
    /// Optional activity timestamp emitted for presence-only updates.
    #[serde(
        rename = "activeAt",
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_at: Option<u64>,
}

impl UpdateMachineBody {
    pub fn new(machine_id: impl Into<String>) -> Self {
        Self {
            kind: UpdateMachineType::UpdateMachine,
            machine_id: machine_id.into(),
            metadata: None,
            daemon_state: None,
            active: None,
            active_at: None,
        }
    }
}

/// Durable update body union discriminated by `t`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CoreUpdateBody {
    NewMessage(UpdateNewMessageBody),
    UpdateSession(UpdateSessionBody),
    UpdateMachine(UpdateMachineBody),
}

/// Minimal durable socket/container wrapper shared across Happy-compatible consumers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreUpdateContainer {
    /// Durable update identifier.
    pub id: String,
    /// Global durable update sequence number.
    pub seq: u64,
    /// Update payload discriminated by `t`.
    pub body: CoreUpdateBody,
    /// Creation timestamp in milliseconds.
    #[serde(rename = "createdAt")]
    pub created_at: u64,
}

pub type ApiMessage = SessionMessage;
pub type ApiUpdateNewMessage = UpdateNewMessageBody;
pub type ApiUpdateSessionState = UpdateSessionBody;
pub type ApiUpdateMachineState = UpdateMachineBody;
pub type UpdateBody = UpdateNewMessageBody;
pub type Update = CoreUpdateContainer;

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        legacy_protocol::{AgentMessage, LegacyMessageContent, UserMessage},
        message_meta::MessageMeta,
        session_protocol::{SessionEvent, SessionRole, SessionTextEvent},
    };

    use super::{
        ApiUpdateMachineState, ApiUpdateNewMessage, ApiUpdateSessionState, CoreUpdateBody,
        CoreUpdateContainer, MessageContent, SessionMessage, SessionMessageContent,
        SessionProtocolMessage, SessionProtocolMessageRole, UpdateMachineBody,
        UpdateNewMessageBody, UpdateSessionBody, VersionedEncryptedValue,
        VersionedMachineEncryptedValue, VersionedNullableEncryptedValue,
    };

    fn sample_session_message() -> SessionMessage {
        SessionMessage {
            id: "msg-1".into(),
            seq: 10,
            local_id: Some(None),
            content: SessionMessageContent::new("ZmFrZS1lbmNyeXB0ZWQ="),
            created_at: 123,
            updated_at: 124,
        }
    }

    #[test]
    fn parses_new_message_update() {
        let parsed = serde_json::from_value::<ApiUpdateNewMessage>(json!({
            "t": "new-message",
            "sid": "session-1",
            "message": {
                "id": "msg-1",
                "seq": 10,
                "localId": null,
                "content": {
                    "t": "encrypted",
                    "c": "ZmFrZS1lbmNyeXB0ZWQ=",
                },
                "createdAt": 123,
                "updatedAt": 124,
            }
        }))
        .unwrap();

        assert_eq!(parsed.session_id, "session-1");
        assert_eq!(parsed.message.local_id, Some(None));
    }

    #[test]
    fn parses_update_session_with_nullable_agent_state() {
        let parsed = serde_json::from_value::<ApiUpdateSessionState>(json!({
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
        }))
        .unwrap();

        assert_eq!(
            parsed.metadata,
            Some(Some(VersionedEncryptedValue {
                version: 2,
                value: "abc".into(),
            }))
        );
        assert_eq!(
            parsed.agent_state,
            Some(Some(VersionedNullableEncryptedValue {
                version: 3,
                value: None,
            }))
        );
    }

    #[test]
    fn parses_update_machine_with_optional_activity_fields() {
        let parsed = serde_json::from_value::<ApiUpdateMachineState>(json!({
            "t": "update-machine",
            "machineId": "machine-1",
            "metadata": {
                "version": 1,
                "value": "abc",
            },
            "daemonState": {
                "version": 2,
                "value": "def",
            },
            "active": true,
            "activeAt": 12345,
        }))
        .unwrap();

        assert_eq!(
            parsed.metadata,
            Some(Some(VersionedMachineEncryptedValue {
                version: 1,
                value: "abc".into(),
            }))
        );
        assert_eq!(parsed.active, Some(true));
        assert_eq!(parsed.active_at, Some(12345));
    }

    #[test]
    fn parses_container_updates_for_all_variants() {
        let examples = [
            json!({
                "id": "upd-1",
                "seq": 1,
                "body": {
                    "t": "new-message",
                    "sid": "session-1",
                    "message": {
                        "id": "msg-1",
                        "seq": 1,
                        "localId": null,
                        "content": { "t": "encrypted", "c": "x" },
                        "createdAt": 1,
                        "updatedAt": 1,
                    }
                },
                "createdAt": 1,
            }),
            json!({
                "id": "upd-2",
                "seq": 2,
                "body": {
                    "t": "update-session",
                    "id": "session-1",
                    "metadata": null,
                    "agentState": {
                        "version": 1,
                        "value": null,
                    }
                },
                "createdAt": 2,
            }),
            json!({
                "id": "upd-3",
                "seq": 3,
                "body": {
                    "t": "update-machine",
                    "machineId": "machine-1",
                    "metadata": null,
                    "daemonState": null,
                },
                "createdAt": 3,
            }),
        ];

        for sample in examples {
            let parsed = serde_json::from_value::<CoreUpdateContainer>(sample).unwrap();
            match parsed.body {
                CoreUpdateBody::NewMessage(_)
                | CoreUpdateBody::UpdateSession(_)
                | CoreUpdateBody::UpdateMachine(_) => {}
            }
        }
    }

    #[test]
    fn parses_legacy_decrypted_payloads() {
        let user = serde_json::from_value::<UserMessage>(json!({
            "role": "user",
            "content": {
                "type": "text",
                "text": "fix this test",
            },
            "meta": {
                "sentFrom": "mobile",
            }
        }))
        .unwrap();
        let agent = serde_json::from_value::<AgentMessage>(json!({
            "role": "agent",
            "content": {
                "type": "output",
                "data": {
                    "type": "message",
                    "message": "done",
                }
            },
            "meta": {
                "sentFrom": "cli",
            }
        }))
        .unwrap();

        assert_eq!(user.meta.unwrap().sent_from.as_deref(), Some("mobile"));
        assert_eq!(agent.content.message_type(), "output");
    }

    #[test]
    fn parses_legacy_and_modern_message_unions() {
        let modern = SessionProtocolMessage {
            role: SessionProtocolMessageRole::Session,
            content: crate::session_protocol::create_envelope(
                SessionRole::Agent,
                SessionEvent::from(SessionTextEvent::new("hello from session protocol")),
                crate::session_protocol::CreateEnvelopeOptions {
                    id: Some("msg-2".into()),
                    time: Some(2000),
                    turn: Some("turn-2".into()),
                    subagent: None,
                },
            )
            .unwrap(),
            meta: Some(MessageMeta {
                sent_from: Some("cli".into()),
                ..MessageMeta::default()
            }),
        };

        let modern_value = serde_json::to_value(&modern).unwrap();
        let user = serde_json::from_value::<MessageContent>(json!({
            "role": "user",
            "content": {
                "type": "text",
                "text": "hello from user",
            }
        }))
        .unwrap();
        let agent = serde_json::from_value::<MessageContent>(json!({
            "role": "agent",
            "content": {
                "type": "output",
                "data": {
                    "type": "message",
                    "message": "hello from agent",
                }
            }
        }))
        .unwrap();
        let modern_parsed = serde_json::from_value::<MessageContent>(modern_value).unwrap();

        assert!(matches!(user, MessageContent::User(_)));
        assert!(matches!(agent, MessageContent::Agent(_)));
        assert!(matches!(modern_parsed, MessageContent::Session(_)));
    }

    #[test]
    fn nullish_fields_preserve_missing_vs_null() {
        let message_with_missing_local_id = SessionMessage {
            local_id: None,
            ..sample_session_message()
        };
        let update_with_missing_metadata = UpdateSessionBody {
            metadata: None,
            agent_state: Some(None),
            ..UpdateSessionBody::new("session-1")
        };
        let machine_with_missing_daemon_state = UpdateMachineBody {
            metadata: Some(None),
            daemon_state: None,
            ..UpdateMachineBody::new("machine-1")
        };

        let message_value = serde_json::to_value(&message_with_missing_local_id).unwrap();
        let update_value = serde_json::to_value(&update_with_missing_metadata).unwrap();
        let machine_value = serde_json::to_value(&machine_with_missing_daemon_state).unwrap();

        assert!(message_value.get("localId").is_none());
        assert!(update_value.get("metadata").is_none());
        assert_eq!(update_value["agentState"], serde_json::Value::Null);
        assert_eq!(machine_value["metadata"], serde_json::Value::Null);
        assert!(machine_value.get("daemonState").is_none());
    }

    #[test]
    fn invalid_discriminators_are_rejected() {
        assert!(
            serde_json::from_value::<MessageContent>(json!({
                "role": "bogus",
                "content": {}
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<CoreUpdateBody>(json!({
                "t": "not-real",
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<LegacyMessageContent>(json!({
                "role": "session",
                "content": {}
            }))
            .is_err()
        );
    }

    #[test]
    fn invalid_null_optional_only_fields_are_rejected() {
        assert!(
            serde_json::from_value::<SessionProtocolMessage>(json!({
                "role": "session",
                "content": {
                    "id": "msg-1",
                    "time": 1,
                    "role": "agent",
                    "ev": {
                        "t": "text",
                        "text": "hello",
                    }
                },
                "meta": null,
            }))
            .is_err()
        );

        let invalid_machine_updates = [
            json!({
                "t": "update-machine",
                "machineId": "machine-1",
                "active": null,
            }),
            json!({
                "t": "update-machine",
                "machineId": "machine-1",
                "activeAt": null,
            }),
        ];

        for invalid_machine_update in invalid_machine_updates {
            assert!(serde_json::from_value::<UpdateMachineBody>(invalid_machine_update).is_err());
        }
    }

    #[test]
    fn round_trips_public_message_types() {
        let message = sample_session_message();
        let update = UpdateNewMessageBody::new("session-1", message.clone());
        let container = CoreUpdateContainer {
            id: "upd-1".into(),
            seq: 1,
            body: CoreUpdateBody::NewMessage(update),
            created_at: 10,
        };

        let value = serde_json::to_value(&container).unwrap();
        let parsed: CoreUpdateContainer = serde_json::from_value(value).unwrap();

        assert_eq!(parsed, container);
    }
}
