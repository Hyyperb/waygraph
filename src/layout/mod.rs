use glam::Vec2;

use crate::{desktop::DesktopItem, renderer::Rendering};

pub mod simplegridlayout;

pub struct DummyLayout {}
impl Layout for DummyLayout {}
impl Rendering for DummyLayout {}

pub trait Layout: Rendering {
    fn arrange_items(&self, state: &mut Vec<DesktopItem>) {}
    fn get_index_of_item_under_position(
        &self,
        desktop_items: &Vec<DesktopItem>,
        x: f32,
        y: f32,
    ) -> Option<usize> {
        None
    }
}

#[derive(Debug)]
pub struct StaticArea {
    pos: Vec2,
    size: Vec2,
}

impl StaticArea {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> StaticArea {
        StaticArea {
            pos: Vec2 { x, y },
            size: Vec2 { x: w, y: h },
        }
    }
}
