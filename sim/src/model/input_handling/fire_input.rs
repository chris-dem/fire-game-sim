use serde::Deserialize;

// use super::import::FixedOrRandom;

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
    ClosestDistance(Option<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum AspirationInput {
    LogAspiration(Option<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum RatioInput {
    Root(Option<f32>),
}
