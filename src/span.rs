#[derive(Clone)]
pub(crate) struct CidSpan;

impl<B> tower_http::trace::MakeSpan<B> for CidSpan {
    fn make_span(&mut self, _request: &axum::http::Request<B>) -> tracing::Span {
        let cid = crate::utils::create_correlation_id();
        let span = tracing::span!(tracing::Level::TRACE, "app", correlation_id = %cid.to_string());
        span
    }
}
