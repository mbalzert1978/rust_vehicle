pub fn create_correlation_id() -> uuid::Uuid {
    uuid::Uuid::now_v7()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_correlation_id_when_called_should_return_a_uuid() {
        assert!(
            uuid::Uuid::parse_str(&create_correlation_id().to_string()).is_ok(),
            "FAIL: Could not parse uuid."
        );
    }
}
