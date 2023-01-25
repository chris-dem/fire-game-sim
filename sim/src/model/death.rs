use krabmaga::{
    bevy::{log, utils::HashSet},
    engine::location::Int2D,
};

pub trait DeathHandler {
    fn update_death(&mut self, loc: Int2D);

    fn is_dead(&self, loc: &Int2D) -> bool;

    fn clear(&mut self);
}

#[derive(Default)]
pub struct HMapHandler {
    death_bag: HashSet<Int2D>,
}

impl DeathHandler for HMapHandler {
    fn update_death(&mut self, loc: Int2D) {
        dbg!("Evacuee died at : ", (loc.x, loc.y));
        self.death_bag.insert(loc);
    }

    fn is_dead(&self, loc: &Int2D) -> bool {
        self.death_bag.contains(loc)
    }

    fn clear(&mut self) {
        self.death_bag.clear();
    }
}
