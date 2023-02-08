use krabmaga::engine::location::Int2D;

use super::misc::misc_func::Reset;

pub trait DeathHandler: Reset {
    fn update_death(&mut self, loc: Int2D);

    fn get_dead(&self) -> usize;
}

#[derive(Default)]
pub struct Announcer(usize);

impl Reset for Announcer {
    fn reset(&mut self) {
        self.0 = 0;
    }
}

impl DeathHandler for Announcer {
    fn update_death(&mut self, _loc: Int2D) {
        self.0 += 1;
    }

    fn get_dead(&self) -> usize {
        self.0
    }
}
