use core::fmt;
use krabmaga::engine::{agent::Agent, location::Int2D, schedule::Schedule, state::State};
use std::hash::{Hash, Hasher};

use crate::model::cell::Cell;

use super::cell::CellType;

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
        todo!()
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

/// Transition mechanism
impl FireRules {
    /// Given current cell, return probability, given Von Neuman Neighbour
    pub fn transition(&self, curr_cell: &CellType, neigh: &[CellType]) -> f32 {
        match curr_cell {
            CellType::Fire => 0.,
            CellType::Empty => {
                neigh.iter().filter(|x| **x == CellType::Fire).count() as f32 / 8. * self.spread
            }
        }
    }
}

/// Module used for testing and mock testing
#[cfg(test)]
mod tests {
    use crate::model::cell::CellType;

    use super::*;
    use approx::assert_relative_eq;
    use rand::prelude::*;

    #[test]
    fn testing_fire_rules_on_fire_cell_constant_test() {
        let neigh = [
            CellType::Fire,
            CellType::Empty,
            CellType::Fire,
            CellType::Fire,
            CellType::Fire,
            CellType::Empty,
            CellType::Fire,
            CellType::Fire,
        ];
        let curr_cell = CellType::Fire;
        let agent = FireRules { id: 1, spread: 0.7 };
        assert_relative_eq!(agent.transition(&curr_cell, &neigh), 0.);
    }

    #[test]
    fn testing_fire_rules_on_fire_cell_random_test() {
        let mut rng = thread_rng();
        let neigh: Vec<CellType> = (0..8).map(|_| rng.gen()).collect();
        let curr_cell = CellType::Fire;
        let agent = FireRules { id: 1, spread: 0.7 };
        assert_relative_eq!(agent.transition(&curr_cell, &neigh), 0.);
    }

    #[test]
    fn testing_fire_rules_on_empty_cell_constant_test() {
        let neigh = [
            CellType::Fire,
            CellType::Empty,
            CellType::Fire,
            CellType::Fire,
            CellType::Fire,
            CellType::Empty,
            CellType::Fire,
            CellType::Fire,
        ];
        let curr_cell = CellType::Empty;
        let agent = FireRules { id: 1, spread: 0.7 };
        let estimated = 6. / 8. * 0.7;
        assert_relative_eq!(agent.transition(&curr_cell, &neigh), estimated);
    }

    #[test]
    fn testing_fire_rules_on_empty_cell_random_test() {
        let mut rng = thread_rng();
        let neigh: Vec<CellType> = (0..8).map(|_| rng.gen()).collect();
        let curr_cell = CellType::Empty;
        let agent = FireRules { id: 1, spread: 0.7 };
        let estimated =
            neigh.iter().filter(|x| **x == CellType::Fire).count() as f32 / 8. * agent.spread;
        assert_relative_eq!(agent.transition(&curr_cell, &neigh), estimated);
    }
}
