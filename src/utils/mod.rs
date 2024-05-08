use crate::prelude::*;

pub fn create_correlation_id() -> uuid::Uuid {
    uuid::Uuid::now_v7()
}

pub fn extract_cid(req: &axum::http::Request<axum::body::Body>) -> Result<uuid::Uuid> {
    let cid = try_extract::<uuid::Uuid>(req)?;
    Ok(*cid)
}

pub fn try_extract<T: Send + Sync + 'static>(
    req: &axum::http::Request<axum::body::Body>,
) -> Result<&T> {
    let err = Error::generic(format!("Missing extension: {}", std::any::type_name::<T>()));
    let value = req.extensions().get::<T>().ok_or(err)?;
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_correlation_id_when_called_should_return_a_uuid() {
        assert!(uuid::Uuid::parse_str(&create_correlation_id().to_string()).is_ok());
    }

    #[tokio::test]
    async fn extract_cid_when_called_with_request_that_holds_a_cid_should_return_the_cid() {
        let mut req = axum::http::Request::default();
        req.extensions_mut().insert(create_correlation_id());

        let result = extract_cid(&req);

        assert!(result.is_ok());
        assert!(uuid::Uuid::parse_str(&result.unwrap().to_string()).is_ok());
    }

    #[tokio::test]
    async fn extract_cid_when_called_with_request_that_does_not_hold_a_cid_should_return_error() {
        let result = extract_cid(&axum::http::Request::default());

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            Error::generic(format!(
                "Missing extension: {}",
                std::any::type_name::<uuid::Uuid>()
            ))
        );
    }
}
