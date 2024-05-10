use axum::Json;
use serde_json::json;

pub(crate) fn create_correlation_id() -> uuid::Uuid {
    uuid::Uuid::now_v7()
}

pub(crate) fn serializer<T: Sized + serde::Serialize>(
    value: &T,
    status: axum::http::StatusCode,
) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    let body = Json(json!(&value));
    (status, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize)]
    struct TestStruct {
        data: i32,
        value: String,
    }

    fn get_test_struct() -> TestStruct {
        TestStruct {
            data: 42,
            value: "foo".to_string(),
        }
    }

    #[tokio::test]
    async fn create_correlation_id_when_called_should_return_a_uuid() {
        assert!(
            uuid::Uuid::parse_str(&create_correlation_id().to_string()).is_ok(),
            "FAIL: Could not parse uuid."
        );
    }

    #[tokio::test]
    async fn serializer_with_null_value_should_return_status_code_and_empty_json() {
        let null_value: Option<i32> = None;
        let (status, body) = serializer(&null_value, axum::http::StatusCode::OK);
        assert_eq!(status, axum::http::StatusCode::OK);
        assert_eq!(body.0.to_string(), "null");
    }

    #[tokio::test]
    async fn serializer_with_null_value_should_return_status_code_and_empty_json_for_not_found() {
        let null_value: Option<i32> = None;
        let (status, body) = serializer(&null_value, axum::http::StatusCode::NOT_FOUND);
        assert_eq!(status, axum::http::StatusCode::NOT_FOUND);
        assert_eq!(body.0.to_string(), "null");
    }

    #[tokio::test]
    async fn serializer_with_null_value_should_return_status_code_and_empty_json_for_bad_request() {
        let null_value: Option<i32> = None;
        let (status, body) = serializer(&null_value, axum::http::StatusCode::BAD_REQUEST);
        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert_eq!(body.0.to_string(), "null");
    }

    #[tokio::test]
    async fn serializer_with_null_value_should_return_status_code_and_empty_json_for_internal_server_error(
    ) {
        let null_value: Option<i32> = None;
        let (status, body) = serializer(&null_value, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(status, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body.0.to_string(), "null");
    }

    #[tokio::test]
    async fn serializer_with_null_value_should_return_status_code_and_empty_json_for_unauthorized()
    {
        let null_value: Option<i32> = None;
        let (status, body) = serializer(&null_value, axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(status, axum::http::StatusCode::UNAUTHORIZED);
        assert_eq!(body.0.to_string(), "null");
    }

    #[tokio::test]
    async fn serializer_with_valid_value_should_return_status_code_and_serialized_json() {
        let (status, body) = serializer(&get_test_struct(), axum::http::StatusCode::OK);
        assert_eq!(status, axum::http::StatusCode::OK);
        assert_eq!(body.0.to_string(), r#"{"data":42,"value":"foo"}"#);
    }
}
