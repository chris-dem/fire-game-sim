use mockall::predicate::*;
use mockall::*;

use crate::model::state::DEFAULT_WIDTH;

#[automock]
pub trait RewardStrategy {
    fn calculate_reward(&self, exit_dist : f32) -> f32;
}


///Inverse Log Square root calculates the following function
/// $f(x) = 1 + (ln 10) / ln(1 + sqrt(x))$
pub struct InverseLogRoot(pub f32, pub f32);

impl Default for InverseLogRoot {
    fn default() -> Self {
        Self(1.,DEFAULT_WIDTH as f32 / 2.)
    }
}

impl RewardStrategy for InverseLogRoot {
    fn calculate_reward(&self,exit_dist:f32) -> f32 {
        let calc =   1. + self.1.ln_1p() * (exit_dist / 1.3).ln_1p().recip();
        calc * self.0
    }
}

/// Self.0 denotes the effect of the reward
/// Self.1 denotes the max dist
/// So in this case it would be (max_h^2 + max_w^2).sqrt
pub struct RootReward(pub f32,pub f32);

impl Default for RootReward {
    fn default() -> Self {
        Self(1., 1.)
    }
}


impl RewardStrategy for RootReward {
    fn calculate_reward(&self,exit_dist:f32) -> f32 {
        self.0 * (self.1 - exit_dist)
    }
}