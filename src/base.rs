pub use ggez::*;
pub use ggez::event::{self, EventHandler, KeyCode, KeyMods};
pub use ggez::graphics::{self, Mesh};
pub use ggez::nalgebra as na;

pub struct Pos {
    pub x : f32,
    pub y : f32,
}

pub enum Direction {
    Left, Right, Up, Down
}
// Lol
