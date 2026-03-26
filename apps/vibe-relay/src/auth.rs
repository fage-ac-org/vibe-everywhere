use axum::{
    extract::{Request, State},
    http::Method,
    middleware::Next,
    response::Response,
};
use url::form_urlencoded;

use crate::{ApiError, AppState};

pub(crate) async fn require_relay_auth(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    if request.method() == Method::OPTIONS {
        return Ok(next.run(request).await);
    }

    let Some(expected_token) = state.config.access_token.as_deref() else {
        return Ok(next.run(request).await);
    };

    if request_access_token(&request).as_deref() != Some(expected_token) {
        return Err(ApiError::unauthorized(
            "auth_required",
            "Missing or invalid access token",
        ));
    }

    Ok(next.run(request).await)
}

pub(crate) fn request_access_token(request: &Request) -> Option<String> {
    if let Some(header_value) = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        let token = header_value
            .strip_prefix("Bearer ")
            .or_else(|| header_value.strip_prefix("bearer "))
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if let Some(token) = token {
            return Some(token.to_string());
        }
    }

    request.uri().query().and_then(query_access_token)
}

pub(crate) fn query_access_token(query: &str) -> Option<String> {
    form_urlencoded::parse(query.as_bytes())
        .find(|(key, _)| key == "access_token")
        .map(|(_, value)| value.into_owned())
        .filter(|value| !value.is_empty())
}
