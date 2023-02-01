use krabmaga::engine::location::Int2D;

pub trait DeathHandler {
    fn update_death(&mut self, loc: Int2D);

    fn get_dead(&self) -> usize;
}

#[derive(Default)]
pub struct Announcer(usize);

impl DeathHandler for Announcer {
    fn update_death(&mut self, loc: Int2D) {
        self.0 += 1;
    }

    fn get_dead(&self) -> usize {
        self.0
    }
}
