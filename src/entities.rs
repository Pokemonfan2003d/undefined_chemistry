use super::*;

pub enum Entities {
    NPC,
    Enemy,
    Dummy,
    Player
}

impl Entities {
    pub fn update(&mut self) -> GameResult<()>{
        Ok(())
    }
    pub fn renderable(&self, ctx: &mut Context) -> GameResult<Mesh> {
        unimplemented!()
    }
}

