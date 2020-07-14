use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn new(x: i32, y: i32) -> Self {
        Vector2 { x, y }
    }
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

impl Renderable {
    pub fn new(glyph: u16, fg: RGB, bg: RGB) -> Self {
        Renderable { glyph, fg, bg }
    }
}
