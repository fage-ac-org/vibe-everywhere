use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::{Map, Value};

use crate::message_meta::MessageMeta;

/// Canonical legacy user payload content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserMessageContent {
    /// User-authored plain-text input.
    pub text: String,
}

impl UserMessageContent {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Serialize for UserMessageContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(rename = "type")]
            kind: &'static str,
            text: &'a str,
        }

        Wire {
            kind: "text",
            text: &self.text,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UserMessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(rename = "type")]
            kind: String,
            text: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        if wire.kind != "text" {
            return Err(de::Error::custom("expected type \"text\""));
        }

        Ok(Self { text: wire.text })
    }
}

/// Fixed role discriminator for legacy user messages.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserMessage {
    #[serde(rename = "role")]
    pub role: UserMessageRole,
    /// Legacy user content, restricted to text payloads.
    pub content: UserMessageContent,
    /// Client-local optimistic identifier used before the durable message id exists.
    #[serde(
        rename = "localKey",
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_key: Option<String>,
    /// Optional message metadata shared with session-protocol wrappers.
    #[serde(
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<MessageMeta>,
}

impl UserMessage {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            role: UserMessageRole::User,
            content: UserMessageContent::new(text),
            local_key: None,
            meta: None,
        }
    }
}

/// Literal `user` role for legacy user messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserMessageRole {
    #[serde(rename = "user")]
    User,
}

/// Legacy agent content wrapper that preserves unknown keys while requiring `type`.
#[derive(Clone, Debug, PartialEq)]
pub struct AgentMessageContent {
    inner: Map<String, Value>,
}

impl AgentMessageContent {
    pub fn new(message_type: impl Into<String>) -> Self {
        let mut inner = Map::new();
        inner.insert("type".into(), Value::String(message_type.into()));
        Self { inner }
    }

    pub fn from_map(inner: Map<String, Value>) -> Result<Self, AgentMessageContentError> {
        validate_agent_content_map(&inner)?;
        Ok(Self { inner })
    }

    pub fn from_value(value: Value) -> Result<Self, AgentMessageContentError> {
        match value {
            Value::Object(inner) => Self::from_map(inner),
            _ => Err(AgentMessageContentError::NotAnObject),
        }
    }

    pub fn message_type(&self) -> &str {
        self.inner
            .get("type")
            .and_then(Value::as_str)
            .expect("validated agent content always contains a string type")
    }

    pub fn as_map(&self) -> &Map<String, Value> {
        &self.inner
    }

    pub fn into_map(self) -> Map<String, Value> {
        self.inner
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }
}

impl Serialize for AgentMessageContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AgentMessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = Map::<String, Value>::deserialize(deserializer)?;
        Self::from_map(inner).map_err(de::Error::custom)
    }
}

/// Literal `agent` role for legacy agent messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentMessageRole {
    #[serde(rename = "agent")]
    Agent,
}

/// Legacy agent message payload retained for imported-app compatibility.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentMessage {
    /// Fixed role discriminator for agent-emitted legacy messages.
    pub role: AgentMessageRole,
    /// Pass-through agent payload with a required `type` field.
    pub content: AgentMessageContent,
    /// Optional metadata used by Happy clients for rendering and runtime hints.
    #[serde(
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub meta: Option<MessageMeta>,
}

impl AgentMessage {
    pub fn new(content: AgentMessageContent) -> Self {
        Self {
            role: AgentMessageRole::Agent,
            content,
            meta: None,
        }
    }
}

/// Top-level legacy decrypted message union discriminated by `role`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LegacyMessageContent {
    User(UserMessage),
    Agent(AgentMessage),
}

/// Validation errors for pass-through legacy agent content.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AgentMessageContentError {
    NotAnObject,
    MissingType,
    NonStringType,
}

impl fmt::Display for AgentMessageContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnObject => write!(f, "agent content must be a JSON object"),
            Self::MissingType => write!(f, "agent content must contain a \"type\" field"),
            Self::NonStringType => write!(f, "agent content field \"type\" must be a string"),
        }
    }
}

impl std::error::Error for AgentMessageContentError {}

fn validate_agent_content_map(inner: &Map<String, Value>) -> Result<(), AgentMessageContentError> {
    match inner.get("type") {
        Some(Value::String(_)) => Ok(()),
        Some(_) => Err(AgentMessageContentError::NonStringType),
        None => Err(AgentMessageContentError::MissingType),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value, json};

    use crate::message_meta::MessageMeta;

    use super::{
        AgentMessage, AgentMessageContent, AgentMessageContentError, LegacyMessageContent,
        UserMessage,
    };

    #[test]
    fn user_message_round_trip() {
        let message = UserMessage {
            local_key: Some("local-1".into()),
            meta: Some(MessageMeta {
                sent_from: Some("mobile".into()),
                ..MessageMeta::default()
            }),
            ..UserMessage::new("fix this test")
        };

        let value = serde_json::to_value(&message).unwrap();
        assert_eq!(
            value,
            json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": "fix this test",
                },
                "localKey": "local-1",
                "meta": {
                    "sentFrom": "mobile",
                }
            })
        );

        let parsed: UserMessage = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, message);
    }

    #[test]
    fn agent_message_passthrough_round_trip() {
        let mut map = Map::new();
        map.insert("type".into(), Value::String("output".into()));
        map.insert(
            "data".into(),
            json!({
                "type": "message",
                "message": "done",
            }),
        );
        map.insert("unknown".into(), json!({ "nested": true }));

        let message = AgentMessage {
            content: AgentMessageContent::from_map(map).unwrap(),
            meta: Some(MessageMeta {
                sent_from: Some("cli".into()),
                ..MessageMeta::default()
            }),
            ..AgentMessage::new(AgentMessageContent::new("ignored"))
        };

        let value = serde_json::to_value(&message).unwrap();
        assert_eq!(value["role"], "agent");
        assert_eq!(value["content"]["type"], "output");
        assert_eq!(value["content"]["unknown"], json!({ "nested": true }));

        let parsed: AgentMessage = serde_json::from_value(value).unwrap();
        assert_eq!(parsed.content.message_type(), "output");
        assert_eq!(
            parsed.content.get("unknown").unwrap(),
            &json!({ "nested": true })
        );
    }

    #[test]
    fn discriminated_union_parse_tests() {
        let user = serde_json::from_value::<LegacyMessageContent>(json!({
            "role": "user",
            "content": {
                "type": "text",
                "text": "hello",
            }
        }))
        .unwrap();
        let agent = serde_json::from_value::<LegacyMessageContent>(json!({
            "role": "agent",
            "content": {
                "type": "event",
                "data": {
                    "type": "ready",
                }
            }
        }))
        .unwrap();

        assert!(matches!(user, LegacyMessageContent::User(_)));
        assert!(matches!(agent, LegacyMessageContent::Agent(_)));
    }

    #[test]
    fn invalid_user_content_is_rejected() {
        let error = serde_json::from_value::<UserMessage>(json!({
            "role": "user",
            "content": {
                "type": "image",
                "text": "hello",
            }
        }))
        .unwrap_err();

        assert!(error.to_string().contains("expected type \"text\""));
    }

    #[test]
    fn invalid_null_optional_only_fields_are_rejected() {
        assert!(
            serde_json::from_value::<UserMessage>(json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": "hello",
                },
                "localKey": null,
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<UserMessage>(json!({
                "role": "user",
                "content": {
                    "type": "text",
                    "text": "hello",
                },
                "meta": null,
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<AgentMessage>(json!({
                "role": "agent",
                "content": {
                    "type": "output",
                },
                "meta": null,
            }))
            .is_err()
        );
    }

    #[test]
    fn invalid_agent_content_is_rejected() {
        let error = AgentMessageContent::from_value(json!({
            "data": {
                "type": "ready"
            }
        }))
        .unwrap_err();

        assert_eq!(error, AgentMessageContentError::MissingType);
    }
}
