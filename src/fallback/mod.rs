use crate::prelude::*;

pub async fn fallback(
    headers: axum::http::HeaderMap,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
) -> impl axum::response::IntoResponse {
    tracing::warn!("Fallback reached::Headers::{:?}", headers);
    Error::not_found(cid)
}

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::*;

    #[tokio::test]
    async fn fallback_when_reached_should_return_not_found() {
        let headers = axum::http::HeaderMap::new();
        let cid = axum::Extension(crate::utils::create_correlation_id());

        let result = fallback(headers, cid).await.into_response();

        assert_eq!(result.status(), axum::http::StatusCode::NOT_FOUND);
    }
}
