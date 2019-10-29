extern crate ggez;

mod base;
use base::*;

mod ground;
use ground::*;

mod media;
use media::*;

mod player;
use player::*;

mod entities;
use entities::*;

mod obstacles;
use obstacles::*;

struct State {
    abs_time : std::time::Duration,
    keys : [bool;4],
    player1 : Player,
    entities : Vec<Entities>,
    obstacles: Vec<Obstacles>
}

impl State {
    fn init() -> State {
        State {
            abs_time : std::time::Duration::new(0, 0),
            keys: [false;4],
            player1: Player::init(),

            entities: Vec::new(),
            obstacles: vec![
                Obstacles::Rectangle(Rectangle::new(0.0, 0.0, 400.0, 40.0) ),
            ]
        }
    }

    fn set_key(&mut self, keycode: KeyCode, value: bool) {
        match keycode {
            KeyCode::W => self.keys[0] = value,
            KeyCode::A => self.keys[1] = value,
            KeyCode::S => self.keys[2] = value,
            KeyCode::D => self.keys[3] = value,
            _ => {}
        }
    }
}



impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.abs_time = timer::delta(ctx);

        self.player1.update(&self.keys)?;

        for mut entity in &mut self.entities {
            entity.update()?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // println!("Time is: {} ns", self.abs_time.subsec_nanos());
        
        graphics::clear(ctx, [0.3, 0.3, 0.3, 1.0].into());

        for mut entity in &mut self.entities {
            let e_render = entity.renderable(ctx)?;
            graphics::draw(ctx, &e_render, graphics::DrawParam::default())?;
        }
        for mut obstacle in &mut self.obstacles {
            let e_render = obstacle.renderable(ctx)?;
            graphics::draw(ctx, &e_render, graphics::DrawParam::default())?;
        }

        
        let player1_render = self.player1.renderable(ctx)?;
        graphics::draw(ctx, &player1_render, graphics::DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymod: KeyMods, repeat: bool) {
        self.set_key(keycode, true);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
        self.set_key(keycode, false);
    }
}

fn main() {

    let mut state = State::init();
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "yay")
        .conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, &mut state).unwrap();
}
