use super::*;

pub enum Obstacles {
    Rectangle(Rectangle),
    Polygon(Polygon), // List of points
    Circle(Circle)
}

impl Obstacles {
    pub fn update(&mut self) -> GameResult<()> {
        Ok(())
    }

    pub fn renderable(&self, ctx: &mut Context) -> GameResult<Mesh> {
        match self {
            Obstacles::Rectangle(r) => Mesh::new_rectangle(
                ctx,
                r.mode,
                r.into(),
                r.color
            ),
            _ => unimplemented!()
        }
    }
}

impl Into<graphics::Rect> for &Rectangle {
    pub fn into(self) -> graphics::Rect {
        graphics::Rect::new(self.x, self.y, self.w, self.h)
    }
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,

    pub mode: graphics::DrawMode,
    pub color: graphics::Color,
}

impl Rectangle {
    pub fn new(x : f32, y: f32, w: f32, h: f32) -> Rectangle {
        Rectangle { x, y, w, h, mode: graphics::DrawMode::fill(), color: graphics::WHITE }
    }
}

pub struct Polygon {
    pub points: Box<[Pos]>,
}

pub struct Circle {
    pub center: Pos,
    pub radius: f32
}

