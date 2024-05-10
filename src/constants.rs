pub(crate) const STARTUP: &str = "Starting Application...";
pub(crate) const TIMEOUT_SECONDS: u64 = 5;

pub(crate) struct Prefix;

impl Prefix {
    pub(crate) fn get() -> &'static str {
        "/api/v1"
    }
}
