use serde::Deserialize;

use super::import::FixedOrRandom;

#[derive(Debug, Clone, Deserialize)]
pub struct FireInput {
    pub frontier: Option<FrontierInput>,
    pub movement: MovementInput,
    pub aspiration: AspirationInput,
    pub ratio: RatioInput,
}

#[derive(Debug, Clone, Deserialize)]
pub enum FrontierInput {
    VecTree,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MovementInput {
    ClosestDistance(FixedOrRandom<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum AspirationInput {
    LogAspiration(FixedOrRandom<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum RatioInput {
    Root(FixedOrRandom<f32>),
}
