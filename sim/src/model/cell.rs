use rand::distributions::Bernoulli;
use rand_derive2::RandGen;
use rand::prelude::*;
use std::fmt;
use std::hash::{Hash, Hasher};

use super::transition::Transition;
/// Cell Type of the simulation. This means the type of the current cell.
/// For now, treating the fire model and the agent model in the same.
///
/// TODO Determine how addition of different fields might affect the situation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RandGen)]
pub enum CellType {
    Fire,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Cell struct for the simulation. Holds state of the cell and id.
/// Since I am planning to use a dense number grid, Cell has to derive:
/// Debug Clone Copy Hash PartialEq Eq Display
#[derive(Debug, Clone, Copy, RandGen)]
pub struct Cell {
    pub state: CellType,
    id: usize,
}

impl Cell {
    fn new(id: usize) -> Self {
        Self {
            state: CellType::Empty,
            id,
        }
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.id == other.id
    }
}

impl Eq for Cell {}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} status {}", self.id, self.state)
    }
}

impl Cell {
    fn spread(&mut self, agent : impl Transition, neigh : &[CellType], rand : &mut impl Rng) {
        // let dist = Bernoulli::new(agent.transition(&self.state, neigh).into()).unwrap();
        // if dist.sample(rand)  {
        //     self.state = CellType::Fire;
        // }
        todo!()
    }
}

// #[cfg(tests)]
mod tests {
    use super::*;
    use crate::model::transition::*;
    use rand_chacha::{self, ChaCha12Rng};
    fn flip_state() {
        let rng = ChaCha12Rng::from_seed(Default::default());

        let mut spread_handler = MockTransition::new();
        let exec = spread_handler
                                        .expect_transition()
                                        .once()
                                        .return_const(0.7);
        
        let cs = [CellType::generate_random() ; 5]; 
        let mut current_cell = Cell::new(5);
        current_cell.spread(exec, &cs, &mut rng);

    }
}
