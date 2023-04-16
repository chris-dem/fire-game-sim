use krabmaga::Distribution;
use lerp::Lerp;
use rand_distr::Standard;
use serde::Deserialize;

use crate::model::lerp::equations::Equation;

// use super::import::FixedOrRandom;

#[derive(Debug, Clone, Deserialize)]
pub struct FireInput {
    pub frontier: Option<FrontierInput>,
    pub movement: MovementInput,
    pub aspiration: AspirationInput,
    pub ratio: RatioInput,
    pub reward_game: RewardGameInput,
}

#[derive(Debug, Clone, Deserialize)]
pub enum FrontierInput {
    VecTree,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MovementInput {
    ClosestDistance(Option<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum AspirationInput {
    LogAspiration(Option<f32>),
    RootAspiration(Option<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct RatioInput(pub LerpInput);

#[derive(Debug, Clone, Deserialize)]
pub struct RewardGameInput(pub LerpInput);
// #[derive(Debug, Clone, Deserialize)]
// pub enum RewardGameInput {
//     InvLogRoot(Option<f32>),
//     RewardRoot(Option<f32>),
// }

#[derive(Debug, Clone, Deserialize)]
pub struct LerpInput {
    pub influence: Option<f32>,
    pub equation: Equation,
}
