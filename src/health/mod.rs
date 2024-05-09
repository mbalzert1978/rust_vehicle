mod constants;
pub mod router;
mod schemas;
mod services;

pub struct Tag;

impl Tag {
    pub fn get() -> &'static str {
        "/health"
    }
}
