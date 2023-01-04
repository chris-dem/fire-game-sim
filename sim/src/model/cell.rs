use rand::distributions::Bernoulli;
use rand::prelude::*;
use rand_derive2::RandGen;
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
    pub id: usize,
}

impl Cell {
    /// Create a cell based on an id. By default the cell is empty
    /// # Argument
    /// * `id` - id of new cell
    ///
    pub fn new(id: usize) -> Self {
        Self {
            state: CellType::Empty,
            id,
        }
    }

    /// Create a cell based on an id. By default the cell is empty
    /// # Argument
    /// * `id` - id of new cell
    ///
    pub fn new_with_fire(id: usize) -> Self {
        Self {
            state: CellType::Fire,
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
    /// Flip the state of the cell based on the Transition agent, its neighbours and a random factor
    /// # Argument
    /// * `agent` - Agent that implements the transition mechanism
    /// * `neigh` - Reference to the list of neighbours of the current cell
    /// * `rand`  - Structure that implements the Rng trait to generate random values
    /// # Returns
    /// `bool` - True if the cell was flipped, false otherwise
    /// # Example
    /// ```
    /// let mut rng = ChaCha12Rng::from_seed(Default::default());
    /// let mut spread_handler = MockTransition::new();
    /// spread_handler
    ///     .expect_transition()
    ///     .once()
    ///     .return_const(0.7);
    ///         
    /// let cs = [CellType::generate_random() ; 5];
    /// let current_cell = Cell::new(5);
    /// let result = current_cell.spread(spread_handler, &cs, &mut rng); // ChaCha generates fixed random numbers, will always be true for this example
    /// assert!(result)
    /// ```
    pub fn spread<T: Rng + ?Sized>(
        &self,
        agent: &impl Transition,
        neigh: &[CellType],
        rng: &mut T,
    ) -> bool {
        if self.state == CellType::Fire {
            // If the cell type is empty, just return
            return false;
        }
        let dist = Bernoulli::new(agent.transition(&self.state, neigh).into()).unwrap();
        dist.sample(rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::transition::*;
    use rand_chacha::{self, ChaCha12Rng};

    #[test]
    fn test_spread_expect_to_convert_cell_to_fire() {
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        let mut spread_handler = MockTransition::new();
        spread_handler.expect_transition().once().return_const(0.7);
        let cs = [CellType::generate_random(); 5];
        let mut current_cell = Cell::new(5);
        let result = current_cell.spread(&spread_handler, &cs, &mut rng);
        assert!(result)
    }

    #[test]
    fn test_spread_expect_to_not_affect_cell() {
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        let mut spread_handler = MockTransition::new();
        spread_handler.expect_transition().never();
        let cs = [CellType::generate_random(); 5];
        let mut current_cell = Cell::new(5);
        current_cell.state = CellType::Fire;
        let result = current_cell.spread(&spread_handler, &cs, &mut rng);
        assert!(!result)
    }
}
