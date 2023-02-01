use std::fmt::Debug;

use krabmaga::engine::location::Int2D;
use mockall::automock;

use super::frontier::{Frontier, FrontierStructure};

#[automock]
pub trait DynamicInfluence {
    fn dynamic_influence(&self, cell: &Int2D, front: &dyn FrontierStructure) -> f32;

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
    fn dynamic_influence(&self, loc: &Int2D, front: &dyn FrontierStructure) -> f32 {
        front.closest_point(&(*loc).into()).unwrap_or(1.).sqrt()
    }

    fn get_dynamic_effect(&self) -> f32 {
        self.0
    }
}
