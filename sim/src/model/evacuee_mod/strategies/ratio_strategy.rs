use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait RatioStrategy {
    fn calculate_ratio(&self, fire_d: f32) -> f32;
}

pub struct RootDist(pub f32);

impl Default for RootDist {
    fn default() -> Self {
        Self(1.)
    }
}

impl RatioStrategy for RootDist {
    fn calculate_ratio(&self, fire_d: f32) -> f32 {
        fire_d.sqrt() * self.0
    }
}
