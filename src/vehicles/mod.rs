pub mod router;
mod schemas;
mod services;
mod utils;

pub struct Tag;

impl Tag {
    pub fn get() -> &'static str {
        "/vehicles"
    }
}
