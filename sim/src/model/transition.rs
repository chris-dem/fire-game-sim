use crate::model::cell::CellType;
use crate::model::fire_mod::fire_spread::FireRules;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait Transition {
    fn transition(&self, curr_cell: &CellType, neigh: &[CellType]) -> f32;
}

/// Transition mechanism
impl Transition for FireRules {
    /// Given current cell, return probability, given Von Neuman Neighbour
    fn transition(&self, curr_cell: &CellType, neigh: &[CellType]) -> f32 {
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
