pub mod router;
mod schemas;
mod services;

pub(crate) struct Tag;

impl Tag {
    pub(crate) fn get() -> &'static str {
        "/vehicles"
    }
}
