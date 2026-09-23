use crate::AppData;

pub trait Rendering {
    fn render(&self, state: &AppData) {}
}
