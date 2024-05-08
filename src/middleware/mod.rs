use crate::{prelude::*, utils::create_correlation_id};

pub async fn inject_cid(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::http::Response<axum::body::Body>> {
    let cid = create_correlation_id();
    let span = tracing::span!(tracing::Level::TRACE, "api", correlation_id = %cid.to_string());

    let _enter = span.enter();

    req.extensions_mut().insert(cid);
    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn inject_cid_middleware_when_called_should_inject_cid_and_enter_a_span() {
        let extract_cid =
            |axum::Extension(cid): axum::Extension<uuid::Uuid>| async move { format!("{}", cid) };

        let test_route = axum::Router::new().route(
            "/test",
            axum::routing::get(extract_cid).layer(axum::middleware::from_fn(inject_cid)),
        );

        let response = test_route
            .oneshot(
                axum::http::Request::builder()
                    .uri("/test")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert!(uuid::Uuid::parse_str(&body).is_ok());
    }
}
