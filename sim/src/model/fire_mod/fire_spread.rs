use core::fmt;
use krabmaga::engine::{agent::Agent, schedule::Schedule, state::State};
use std::hash::{Hash, Hasher};

use crate::model::{fire_mod::fire_cell::CellType, misc::misc_func::Loc, state::CellGrid};
use krabmaga::rand as krand;

#[derive(Clone)]
pub struct FireRules {
    pub id: u32,
    pub spread: f32,
    pub fire_grid: Vec<CellType>,
}

impl Agent for FireRules {
    /// Should implement movement
    /// Might include in the future mutation of other states
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<CellGrid>().unwrap();
        let mut rng = krand::thread_rng();
        state.fire_step(self, &mut rng);
    }
}

impl FireRules {
    #[allow(dead_code)]
    fn update(
        _value: &CellType,
        _state: &mut dyn State,
        _schedule: &mut Schedule,
        _schedule_id: u32,
    ) {
    }

    pub fn new(dims: usize, id: u32, spread: f32, location: usize) -> Self {
        let mut fire_grid = vec![CellType::Empty; dims];
        fire_grid[location] = CellType::Fire;
        fire_grid.shrink_to_fit();
        Self {
            id,
            spread,
            fire_grid,
        }
    }
}

impl Hash for FireRules {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl fmt::Display for FireRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.spread)
    }
}

impl PartialEq for FireRules {
    fn eq(&self, other: &FireRules) -> bool {
        self.id == other.id
    }
}
