use std::fmt::Debug;

use mockall::automock;

use crate::model::misc::misc_func::Loc;

use super::frontier::FrontierStructure;

#[automock]
pub trait DynamicInfluence {
    fn dynamic_influence(&self, cell: &Loc, front: &dyn FrontierStructure) -> f32;

    fn get_dynamic_effect(&self) -> f32;
}

/// Dynamic Influence structure that uses closest point to the frontier point in order to calculate values such as
/// r_t = cost-to-ratio
/// A_t = Aspiration
#[derive(Debug)]
pub struct ClosestDistance(pub f32);

impl Default for ClosestDistance {
    fn default() -> Self {
        Self(1.)
    }
}

impl DynamicInfluence for ClosestDistance {
    #[inline]
    fn dynamic_influence(&self, loc: &Loc, front: &dyn FrontierStructure) -> f32 {
        front.closest_point(&loc).unwrap_or(1.).sqrt()
    }

    fn get_dynamic_effect(&self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_distances(x in 0i32..100i32, y in 0i32..100) {
            let mut mock_structure = MockFrontierStructure::new();

            mock_structure
                .expect_closest_point()
                .returning(|Loc(x,y) : &Loc| Some(*x as f32 + *y as f32))
                .once();
            let dist = ClosestDistance::default();
            prop_assert!(crate::model::misc::misc_func::relative_eq_close(dist.dynamic_influence(&Loc(x,y) ,&mock_structure), (x  as f32+ y as f32).sqrt()))
        }
    }
    use crate::model::{
        evacuee_mod::fire_influence::frontier::MockFrontierStructure, misc::misc_func::Loc,
    };

    use super::*;

    mod init {
        use super::*;
        #[test]
        fn test_distance_default() {
            let dist = ClosestDistance::default();
            assert_eq!(dist.get_dynamic_effect(), 1.);
        }

        #[test]
        fn test_distance_with_effect() {
            let dist = ClosestDistance(0.5);
            assert_eq!(dist.get_dynamic_effect(), 0.5);
        }
    }

    mod caclulating_distance {
        use approx::assert_relative_eq;

        use super::*;
        #[test]
        fn closest_distance_from_structure() {
            let mut mock_structure = MockFrontierStructure::new();

            mock_structure
                .expect_closest_point()
                .returning(|Loc(x, y)| Some((x * x + y * y) as f32))
                .once();
            let cloest_distance = ClosestDistance::default();
            assert_relative_eq!(
                cloest_distance.dynamic_influence(&Loc(3, 4), &mock_structure),
                5.
            );
        }
        #[test]
        fn closest_distance_from_empty_grid() {
            let mut mock_structure = MockFrontierStructure::new();

            mock_structure
                .expect_closest_point()
                .returning(|_| None)
                .once();
            let cloest_distance = ClosestDistance::default();
            assert_relative_eq!(
                cloest_distance.dynamic_influence(&Loc(3, 4), &mock_structure),
                1.
            );
        }
    }
}
