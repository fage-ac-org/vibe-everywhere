use serde::{Deserialize, Serialize};

/// Shared optional metadata attached to legacy and session-protocol messages.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageMeta {
    /// Runtime or surface identifier that emitted the message.
    #[serde(
        rename = "sentFrom",
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub sent_from: Option<String>,
    /// Requested permission handling mode key for the message.
    #[serde(
        rename = "permissionMode",
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_mode: Option<String>,
    /// Requested primary model; `Some(None)` preserves an explicit JSON `null`.
    #[serde(
        rename = "model",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub model: Option<Option<String>>,
    /// Requested fallback model; `Some(None)` preserves an explicit JSON `null`.
    #[serde(
        rename = "fallbackModel",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_model: Option<Option<String>>,
    /// Full system prompt override; `Some(None)` preserves an explicit JSON `null`.
    #[serde(
        rename = "customSystemPrompt",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_system_prompt: Option<Option<String>>,
    /// Prompt text appended after the base system prompt.
    #[serde(
        rename = "appendSystemPrompt",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub append_system_prompt: Option<Option<String>>,
    /// Explicit allow-list of tool identifiers.
    #[serde(
        rename = "allowedTools",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_tools: Option<Option<Vec<String>>>,
    /// Explicit deny-list of tool identifiers.
    #[serde(
        rename = "disallowedTools",
        default,
        with = "crate::serde_helpers::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub disallowed_tools: Option<Option<Vec<String>>>,
    /// Rendering-friendly text used by Happy clients for summaries or labels.
    #[serde(
        rename = "displayText",
        default,
        with = "crate::serde_helpers::strict_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::MessageMeta;

    #[test]
    fn full_object_round_trip() {
        let meta = MessageMeta {
            sent_from: Some("mobile".into()),
            permission_mode: Some("bypassPermissions".into()),
            model: Some(Some("gpt-5".into())),
            fallback_model: Some(None),
            custom_system_prompt: Some(Some("system".into())),
            append_system_prompt: Some(None),
            allowed_tools: Some(Some(vec!["shell".into(), "git".into()])),
            disallowed_tools: Some(None),
            display_text: Some("Visible".into()),
        };

        let value = serde_json::to_value(&meta).unwrap();
        assert_eq!(
            value,
            json!({
                "sentFrom": "mobile",
                "permissionMode": "bypassPermissions",
                "model": "gpt-5",
                "fallbackModel": null,
                "customSystemPrompt": "system",
                "appendSystemPrompt": null,
                "allowedTools": ["shell", "git"],
                "disallowedTools": null,
                "displayText": "Visible",
            })
        );

        let parsed: MessageMeta = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn sparse_object_round_trip() {
        let meta = MessageMeta {
            sent_from: Some("cli".into()),
            ..MessageMeta::default()
        };

        let value = serde_json::to_value(&meta).unwrap();
        assert_eq!(value, json!({ "sentFrom": "cli" }));

        let parsed: MessageMeta = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn known_permission_mode_values_round_trip() {
        let cases = [
            "default",
            "acceptEdits",
            "bypassPermissions",
            "plan",
            "read-only",
            "safe-yolo",
            "yolo",
        ];

        for mode in cases {
            let meta = MessageMeta {
                permission_mode: Some(mode.into()),
                ..MessageMeta::default()
            };
            let value = serde_json::to_value(&meta).unwrap();
            assert_eq!(value, json!({ "permissionMode": mode }));

            let parsed: MessageMeta = serde_json::from_value(value).unwrap();
            assert_eq!(parsed, meta);
        }
    }

    #[test]
    fn custom_permission_mode_value_round_trips() {
        let meta = MessageMeta {
            permission_mode: Some("team-custom-mode".into()),
            ..MessageMeta::default()
        };

        let value = serde_json::to_value(&meta).unwrap();
        assert_eq!(value, json!({ "permissionMode": "team-custom-mode" }));

        let parsed: MessageMeta = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn invalid_non_string_permission_mode_is_rejected() {
        let error = serde_json::from_value::<MessageMeta>(json!({
            "permissionMode": 7,
        }))
        .unwrap_err();

        assert!(error.to_string().contains("string"));
    }

    #[test]
    fn invalid_null_optional_only_fields_are_rejected() {
        let invalid_values = [
            json!({ "sentFrom": null }),
            json!({ "permissionMode": null }),
            json!({ "displayText": null }),
        ];

        for invalid_value in invalid_values {
            assert!(serde_json::from_value::<MessageMeta>(invalid_value).is_err());
        }
    }
}
