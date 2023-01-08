use rand::prelude::*;
use rand_derive2::RandGen;
use serde::Deserialize;
use std::fmt;

use super::transition::Transition;
/// Cell Type of the simulation. This means the type of the current cell.
/// For now, treating the fire model and the agent model in the same.
///
/// TODO Determine how addition of different fields might affect the situation
#[derive(Debug, Clone, Copy, PartialEq, Eq, RandGen, Deserialize)]
pub enum CellType {
    Fire,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CellType {
    /// Return if the state of the current cell should be flipped
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
    /// let current_cell = CellType::Empty;
    /// let result = current_cell.spread(spread_handler, &cs, &mut rng); // ChaCha generates fixed random numbers, will always be true for this example
    /// assert!(result)
    /// ```
    pub fn spread<T: Rng + ?Sized>(
        &self,
        agent: &impl Transition,
        neigh: &[CellType],
        rng: &mut T,
    ) -> bool {
        if *self == CellType::Fire {
            // If the cell type is empty, just return
            return false;
        }
        rng.gen_bool(agent.transition(self, neigh).into())
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
        let current_cell = CellType::Empty;
        let result = current_cell.spread(&spread_handler, &cs, &mut rng);
        assert!(result)
    }
    #[test]
    fn test_spread_expect_to_not_convert_cell_to_fire() {
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        let mut spread_handler = MockTransition::new();
        spread_handler.expect_transition().once().return_const(0.);
        let cs = [CellType::generate_random(); 5];
        let current_cell = CellType::Empty;
        let result = current_cell.spread(&spread_handler, &cs, &mut rng);
        assert!(!result)
    }

    #[test]
    fn test_spread_expect_to_not_affect_cell() {
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        let mut spread_handler = MockTransition::new();
        spread_handler.expect_transition().never();
        let cs = [CellType::generate_random(); 5];
        let current_cell = CellType::Fire;
        let result = current_cell.spread(&spread_handler, &cs, &mut rng);
        assert!(!result)
    }
}
