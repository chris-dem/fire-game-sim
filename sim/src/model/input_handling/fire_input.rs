use serde::Deserialize;

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
}

#[derive(Debug, Clone, Deserialize)]
pub enum RatioInput {
    Root(Option<f32>),
    Log(Option<f32>),
    Id(Option<f32>),
}

#[derive(Debug, Clone, Deserialize)]
pub enum RewardGameInput {
    InvLogRoot(Option<f32>),
}
