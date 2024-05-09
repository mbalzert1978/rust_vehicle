pub const STARTING: &str = "Starting Application...";

pub struct Prefix;

impl Prefix {
    pub fn get() -> &'static str {
        "/api/v1"
    }
}
