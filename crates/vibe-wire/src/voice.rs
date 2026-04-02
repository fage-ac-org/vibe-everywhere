use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// Happy-compatible deny reasons for voice token requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoiceTokenDeniedReason {
    #[serde(rename = "voice_limit_reached")]
    VoiceLimitReached,
    #[serde(rename = "subscription_required")]
    SubscriptionRequired,
}

/// Voice token response payload for allowed requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceTokenAllowed {
    /// Issued ElevenLabs-compatible token.
    pub token: String,
    /// Agent identifier used for billing and attribution.
    pub agent_id: String,
    /// ElevenLabs user identifier associated with the token.
    pub eleven_user_id: String,
    /// Current usage in seconds.
    pub used_seconds: u64,
    /// Subscription or quota limit in seconds.
    pub limit_seconds: u64,
}

impl Serialize for VoiceTokenAllowed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wire<'a> {
            allowed: bool,
            token: &'a str,
            #[serde(rename = "agentId")]
            agent_id: &'a str,
            #[serde(rename = "elevenUserId")]
            eleven_user_id: &'a str,
            #[serde(rename = "usedSeconds")]
            used_seconds: u64,
            #[serde(rename = "limitSeconds")]
            limit_seconds: u64,
        }

        Wire {
            allowed: true,
            token: &self.token,
            agent_id: &self.agent_id,
            eleven_user_id: &self.eleven_user_id,
            used_seconds: self.used_seconds,
            limit_seconds: self.limit_seconds,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for VoiceTokenAllowed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            allowed: bool,
            token: String,
            #[serde(rename = "agentId")]
            agent_id: String,
            #[serde(rename = "elevenUserId")]
            eleven_user_id: String,
            #[serde(rename = "usedSeconds")]
            used_seconds: u64,
            #[serde(rename = "limitSeconds")]
            limit_seconds: u64,
        }

        let wire = Wire::deserialize(deserializer)?;
        if !wire.allowed {
            return Err(de::Error::custom("expected allowed=true"));
        }

        Ok(Self {
            token: wire.token,
            agent_id: wire.agent_id,
            eleven_user_id: wire.eleven_user_id,
            used_seconds: wire.used_seconds,
            limit_seconds: wire.limit_seconds,
        })
    }
}

/// Voice token response payload for denied requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceTokenDenied {
    /// Stable deny reason constrained to the Happy value set.
    pub reason: VoiceTokenDeniedReason,
    /// Current usage in seconds.
    pub used_seconds: u64,
    /// Subscription or quota limit in seconds.
    pub limit_seconds: u64,
    /// Agent identifier associated with the denied request.
    pub agent_id: String,
}

impl Serialize for VoiceTokenDenied {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Wire<'a> {
            allowed: bool,
            reason: VoiceTokenDeniedReason,
            #[serde(rename = "usedSeconds")]
            used_seconds: u64,
            #[serde(rename = "limitSeconds")]
            limit_seconds: u64,
            #[serde(rename = "agentId")]
            agent_id: &'a str,
        }

        Wire {
            allowed: false,
            reason: self.reason,
            used_seconds: self.used_seconds,
            limit_seconds: self.limit_seconds,
            agent_id: &self.agent_id,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for VoiceTokenDenied {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            allowed: bool,
            reason: VoiceTokenDeniedReason,
            #[serde(rename = "usedSeconds")]
            used_seconds: u64,
            #[serde(rename = "limitSeconds")]
            limit_seconds: u64,
            #[serde(rename = "agentId")]
            agent_id: String,
        }

        let wire = Wire::deserialize(deserializer)?;
        if wire.allowed {
            return Err(de::Error::custom("expected allowed=false"));
        }

        Ok(Self {
            reason: wire.reason,
            used_seconds: wire.used_seconds,
            limit_seconds: wire.limit_seconds,
            agent_id: wire.agent_id,
        })
    }
}

/// Top-level voice token union discriminated by `allowed`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceTokenResponse {
    Allowed(VoiceTokenAllowed),
    Denied(VoiceTokenDenied),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{VoiceTokenAllowed, VoiceTokenDenied, VoiceTokenDeniedReason, VoiceTokenResponse};

    #[test]
    fn allow_response_round_trip() {
        let response = VoiceTokenAllowed {
            token: "token".into(),
            agent_id: "agent-1".into(),
            eleven_user_id: "eleven-1".into(),
            used_seconds: 12,
            limit_seconds: 42,
        };

        let value = serde_json::to_value(&response).unwrap();
        assert_eq!(
            value,
            json!({
                "allowed": true,
                "token": "token",
                "agentId": "agent-1",
                "elevenUserId": "eleven-1",
                "usedSeconds": 12,
                "limitSeconds": 42,
            })
        );

        let parsed: VoiceTokenAllowed = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, response);
    }

    #[test]
    fn deny_response_round_trip() {
        let response = VoiceTokenDenied {
            reason: VoiceTokenDeniedReason::SubscriptionRequired,
            used_seconds: 12,
            limit_seconds: 42,
            agent_id: "agent-1".into(),
        };

        let value = serde_json::to_value(&response).unwrap();
        assert_eq!(
            value,
            json!({
                "allowed": false,
                "reason": "subscription_required",
                "usedSeconds": 12,
                "limitSeconds": 42,
                "agentId": "agent-1",
            })
        );

        let parsed: VoiceTokenDenied = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, response);
    }

    #[test]
    fn response_union_parses_both_variants() {
        let allowed = serde_json::from_value::<VoiceTokenResponse>(json!({
            "allowed": true,
            "token": "token",
            "agentId": "agent-1",
            "elevenUserId": "eleven-1",
            "usedSeconds": 12,
            "limitSeconds": 42,
        }))
        .unwrap();
        let denied = serde_json::from_value::<VoiceTokenResponse>(json!({
            "allowed": false,
            "reason": "voice_limit_reached",
            "usedSeconds": 12,
            "limitSeconds": 42,
            "agentId": "agent-1",
        }))
        .unwrap();

        assert!(matches!(allowed, VoiceTokenResponse::Allowed(_)));
        assert!(matches!(denied, VoiceTokenResponse::Denied(_)));
    }

    #[test]
    fn invalid_reason_is_rejected() {
        let error = serde_json::from_value::<VoiceTokenDenied>(json!({
            "allowed": false,
            "reason": "not-real",
            "usedSeconds": 12,
            "limitSeconds": 42,
            "agentId": "agent-1",
        }))
        .unwrap_err();

        assert!(error.to_string().contains("unknown variant"));
    }

    #[test]
    fn invalid_discriminators_are_rejected() {
        let invalid_values = [
            json!({
                "token": "token",
                "agentId": "agent-1",
                "elevenUserId": "eleven-1",
                "usedSeconds": 12,
                "limitSeconds": 42,
            }),
            json!({
                "allowed": "true",
                "token": "token",
                "agentId": "agent-1",
                "elevenUserId": "eleven-1",
                "usedSeconds": 12,
                "limitSeconds": 42,
            }),
            json!({
                "allowed": true,
                "reason": "voice_limit_reached",
                "usedSeconds": 12,
                "limitSeconds": 42,
                "agentId": "agent-1",
            }),
            json!({
                "allowed": false,
                "token": "token",
                "agentId": "agent-1",
                "elevenUserId": "eleven-1",
                "usedSeconds": 12,
                "limitSeconds": 42,
            }),
        ];

        for invalid_value in invalid_values {
            assert!(serde_json::from_value::<VoiceTokenResponse>(invalid_value).is_err());
        }
    }
}
