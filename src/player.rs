use super::*;

pub struct Player {
    pub pos      : Pos,
    pub vel      : Pos,
    pub max_vel  : Pos,
    pub mass     : f32,
    pub force    : f32,

    pub stand_on : GroundType,
    pub is_in    : Media,
    pub fcoeff   : f32,
    pub dcoeff   : f32,
    pub area     : f32,

    pub size     : Pos,
}

impl Player {
    pub fn init() -> Player {
        Player {
            pos: Pos { x: 100.0, y: 200.0 },
            vel: Pos { x: 0.0, y: 0.0 },
            max_vel : Pos { x: 5.0, y: 5.0 },
            force: 10.0,
            mass: 1.0,

            stand_on: GroundType::Normal,
            is_in: Media::Air,
            fcoeff: 0.1,
            dcoeff: 0.01,
            area: 1.0,

            size: Pos { x: 20.0, y: 20.0 }
        }
    }

    pub fn update(&mut self, keys: &[bool;4]) -> GameResult<()> {

        if keys[0] {
            self.vel.y -= self.force / self.mass;
        }
        if keys[1] {
            self.vel.x -= self.force / self.mass;
        }
        if keys[2] {
            self.vel.y += self.force / self.mass;
        }
        if keys[3] {
            self.vel.x += self.force / self.mass;
        }


        let f : f32 = (&self.stand_on).into();
        let ffactor = f * self.mass * self.fcoeff;
        let friction = Pos { x: ffactor * self.vel.x, y: ffactor * self.vel.y } ;

        let d : f32 = (&self.is_in).into();
        let dfactor = d * self.dcoeff * self.area / self.mass;
        let drag = Pos {
            x: dfactor * self.vel.x.abs() * self.vel.x, 
            y: dfactor * self.vel.y.abs() * self.vel.x
        };

        let between = |x : f32, min: f32, max: f32| { min.max(max.min(x)) };
        let sbet = |x: f32, npboundaries: f32| { between(x, -npboundaries, npboundaries) };

        self.vel.x = sbet(self.vel.x - drag.x - friction.x, self.max_vel.x);
        self.vel.y = sbet(self.vel.y - drag.y - friction.y, self.max_vel.y);

        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        Ok(())
    }

    pub fn renderable(&self, ctx: &mut Context) -> GameResult<Mesh> {
        let circle = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: self.pos.x, y: self.pos.y},
            10.0,
            0.1,
            graphics::WHITE
        );

        circle
    }
}

