use crate::model::fire_mod::fire_cell::CellType;
use crate::model::fire_mod::fire_spread::FireRules;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait Transition {
    fn transition(&self, curr_cell: &CellType, neigh: &[CellType]) -> f32;
    fn transition_with_number(&self, curr_cell: &CellType, neigh: usize) -> f32;
    fn handle_grid(&mut self) -> &mut Vec<CellType>;
}

/// Transition mechanism
impl Transition for FireRules {
    /// Given current cell, return probability, given Von Neuman Neighbour
    fn transition(&self, curr_cell: &CellType, neigh: &[CellType]) -> f32 {
        assert!(neigh.len() <= 8);
        self.transition_with_number(
            curr_cell,
            neigh.iter().filter(|x| **x == CellType::Fire).count(),
        )
    }

    fn transition_with_number(&self, curr_cell: &CellType, neigh: usize) -> f32 {
        assert!(neigh <= 8);
        match curr_cell {
            CellType::Fire => 0.,
            CellType::Empty => neigh as f32 / 8. * self.spread,
        }
    }

    fn handle_grid(&mut self) -> &mut Vec<CellType> {
        &mut self.fire_grid
    }
}

/// Module used for testing and mock testing
#[cfg(test)]
mod tests {

    // use crate::model::fire_mod::fire_cell::CellType;

    // use super::*;
    // use approx::{assert_relative_eq, Relative};
    // use rand::prelude::*;

    // #[test]
    // fn testing_fire_rules_on_fire_cell_constant_test() {
    //     let neigh = [
    //         CellType::Fire,
    //         CellType::Empty,
    //         CellType::Fire,
    //         CellType::Fire,
    //         CellType::Fire,
    //         CellType::Empty,
    //         CellType::Fire,
    //         CellType::Fire,
    //     ];
    //     let curr_cell = CellType::Fire;
    //     let agent = FireRules { id: 1, spread: 0.7 };
    //     assert_relative_eq!(agent.transition(&curr_cell, &neigh), 0.);
    // }

    // #[test]
    // fn testing_fire_rules_on_fire_cell_random_test() {
    //     let mut rng = thread_rng();
    //     let neigh: Vec<CellType> = (0..8).map(|_| rng.gen()).collect();
    //     let curr_cell = CellType::Fire;
    //     let agent = FireRules { id: 1, spread: 0.7 };
    //     assert_relative_eq!(agent.transition(&curr_cell, &neigh), 0.);
    // }

    // #[test]
    // fn testing_fire_rules_on_empty_cell_constant_test() {
    //     let neigh = [
    //         CellType::Fire,
    //         CellType::Empty,
    //         CellType::Fire,
    //         CellType::Fire,
    //         CellType::Fire,
    //         CellType::Empty,
    //         CellType::Fire,
    //         CellType::Fire,
    //     ];
    //     let curr_cell = CellType::Empty;
    //     let agent = FireRules { id: 1, spread: 0.7 };
    //     let estimated = 6. / 8. * 0.7;
    //     assert_relative_eq!(agent.transition(&curr_cell, &neigh), estimated);
    // }

    // #[test]
    // fn testing_fire_rules_on_empty_cell_random_test() {
    //     let mut rng = thread_rng();
    //     let neigh: Vec<CellType> = (0..8).map(|_| rng.gen()).collect();
    //     let curr_cell = CellType::Empty;
    //     let agent = FireRules { id: 1, spread: 0.7 };
    //     let estimated =
    //         neigh.iter().filter(|x| **x == CellType::Fire).count() as f32 / 8. * agent.spread;
    //     assert_relative_eq!(agent.transition(&curr_cell, &neigh), estimated);
    // }

    // #[test]
    // fn testing_fire_rules_on_with_number() {
    //     let curr_cell = CellType::Empty;
    //     let agent = FireRules { id: 1, spread: 0.7 };
    //     let neigh = 7;
    //     let estimated = 7. / 8. * 0.7;
    //     assert_relative_eq!(agent.transition_with_number(&curr_cell, neigh), estimated);
    // }

    // #[test]
    // fn testing_fire_rules_with_many_cases() {
    //     let curr_cell = CellType::Empty;
    //     let mut rng = thread_rng();
    //     let mut v = Vec::with_capacity(100);
    //     for _ in 0..100 {
    //         let spread_generated: f32 = rng.gen();
    //         let n: usize = rng.gen_range(0..9);
    //         let agent = FireRules {
    //             id: 1,
    //             spread: spread_generated,
    //         };
    //         v.push((
    //             agent.transition_with_number(&curr_cell, n),
    //             n as f32 / 8. * spread_generated,
    //         ))
    //     }
    //     assert!(v.into_iter().all(|(a, b)| Relative::default().eq(&a, &b)))
    // }

    // #[test]
    // fn test_with_fixed_number() {
    //     let mut rng = SmallRng::seed_from_u64(32);
    //     // 0.41460901
    //     let spread = rng.gen();
    //     let curr_cell = CellType::Empty;
    //     let agent = FireRules { id: 1, spread };
    //     // 4 3 4 0 5
    //     for el in (0..5).map(|_| rng.gen_range(0..9)) {
    //         assert_relative_eq!(
    //             agent.transition_with_number(&curr_cell, el),
    //             el as f32 / 8. * spread
    //         );
    //     }
    // }
}
