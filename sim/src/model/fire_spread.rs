use core::fmt;
use krabmaga::engine::{agent::Agent, location::Int2D, schedule::Schedule, state::State};
use std::hash::{Hash, Hasher};

use crate::model::cell::Cell;

use crate::model::{cell::CellType, grid::CellGrid};

#[derive(Clone, Copy)]
pub struct FireRules {
    pub id: u32,
    pub spread: f32,
}

impl Agent for FireRules {
    /// TODO
    /// Should implement movement
    /// Might include in the future mutation of other states
    fn step(&mut self, state: &mut dyn State) {
        // let real_state = state.as_any().downcast_ref::<CellGrid>().unwrap();
        todo!("Step for agent not implemented")
    }
}

impl FireRules {
    #[allow(dead_code)]
    fn update(
        _loc: &Int2D,
        _value: &Cell,
        _state: &mut dyn State,
        _schedule: &mut Schedule,
        _schedule_id: u32,
    ) {
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

