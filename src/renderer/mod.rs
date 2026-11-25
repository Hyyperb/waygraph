use crate::AppData;

pub mod quad;

pub trait Rendering {
    fn render(&self, state: &AppData) {}
}
