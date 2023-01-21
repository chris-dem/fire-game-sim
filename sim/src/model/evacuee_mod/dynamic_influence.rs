use std::fmt::Debug;

use krabmaga::engine::location::Int2D;
use mockall::automock;

use crate::model::evacuee_mod::frontier::frontier_struct::Frontier;

#[automock]
pub trait DynamicInfluence {
    fn dynamic_influence(&self, cell: &Int2D) -> f32;

    fn on_fire_update(&mut self, fire_cell: &Int2D);

    fn get_number_of_cells(&self) -> usize;

    fn get_dynamic_effect(&self) -> f32;
}

/// Dynamic Influence structure that uses closest point to the frontier point in order to calculate values such as
/// r_t = cost-to-ratio
/// A_t = Aspiration
#[derive(Debug)]
pub struct ClosestDistance {
    distance_vec: Frontier,
    // calc_asp: Box<dyn AspirationStrategy>,
    // calc_: Box<dyn >,
    num_fcells: usize,
    d_effect: f32,
}

impl ClosestDistance {
    pub fn new(width: usize, d_effect: f32) -> Self {
        let distance_vec = Frontier::new(width);
        Self {
            distance_vec,
            num_fcells: 0,
            d_effect,
        }
    }
}

impl DynamicInfluence for ClosestDistance {
    #[inline]
    fn dynamic_influence(&self, Int2D { x, y }: &Int2D) -> f32 {
        self.distance_vec.closest_point(&(*x, *y))
    }

    #[inline]
    fn on_fire_update(&mut self, Int2D { x, y }: &Int2D) {
        self.distance_vec.update(&(*x, *y));
        self.num_fcells += 1
    }

    #[inline]
    fn get_number_of_cells(&self) -> usize {
        self.num_fcells
    }

    fn get_dynamic_effect(&self) -> f32 {
        self.d_effect
    }
}
