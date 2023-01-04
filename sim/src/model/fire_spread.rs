use core::fmt;
use krabmaga::engine::{agent::Agent, location::Int2D, schedule::Schedule, state::State};
use std::hash::{Hash, Hasher};

use crate::model::cell::Cell;

use crate::model::{cell::CellType, grid::CellGrid};

use super::transition::*;

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
        let state = state.as_any_mut().downcast_mut::<CellGrid>().unwrap();
        let mut updated: Vec<(Int2D, usize)> = Vec::new();
        // let vals
        for r in 0..state.dim.0 as i32 {
            for c in 0..state.dim.1 as i32 {
                let cell = state.grid.get_value(&Int2D { x: r, y: c }).unwrap();
                let mut n = Vec::with_capacity(8);
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        if let Some(c) = state.grid.get_value(&Int2D { x: r + i, y: c + j }) {
                            n.push(c.state);
                        }
                    }
                }
                if cell.spread(self, &n[..], state.rng.as_mut()) {
                    updated.push((Int2D { x: r, y: c }, cell.id));
                }
            }
        }
        for (pos, id) in updated {
            state.grid.set_value_location(Cell::new_with_fire(id), &pos)
        }
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
