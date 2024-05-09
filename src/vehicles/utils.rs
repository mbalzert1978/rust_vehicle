pub(crate) fn serializer<T: Sized + serde::Serialize>(
    value: &T,
    status: axum::http::StatusCode,
) -> (axum::http::StatusCode, String) {
    let body = serde_json::to_string(&value).unwrap_or_default();
    (status, body)
}
