use krabmaga::{
    bevy::{log, utils::HashSet},
    engine::location::Int2D,
};

pub trait DeathHandler {
    fn update_death(&mut self, loc: Int2D);
}

#[derive(Default)]
pub struct Announcer;

impl DeathHandler for Announcer {
    fn update_death(&mut self, loc: Int2D) {
        dbg!("Evacuee died at : ", (loc.x, loc.y));
    }
}
