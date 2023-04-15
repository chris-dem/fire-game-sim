use serde::Deserialize;

use crate::model::{evacuee_mod::evacuee_cell::EvacueeCell, state::SimType};

use super::fire_input::FireInput;

#[derive(Debug, Deserialize)]
pub struct ImportImproved {
    pub dim: (u32, u32),
    pub sim_type: SimType,
    pub param_seed: Option<u64>,
    pub setup: Setup,
    pub fire: FireInput,
    pub escape: EscapeInput,
    pub death: DeathInput,
    pub static_input: StaticInput,
}

#[derive(Debug, Deserialize)]
pub struct Setup {
    pub map_seed: Option<u64>,
    pub initial_fire: Option<(i32, i32)>,
    pub initial_evac: Option<Vec<EvacueeCell>>,
    pub evac_number: Option<usize>,
    pub fire_spread: Option<f32>,
    pub lc: Option<f32>,
    pub ld: Option<f32>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum EscapeInput {
    TimeTracker,
}

#[derive(Debug, Clone, Deserialize)]
pub enum DeathInput {
    AnnounceInput,
}

#[derive(Debug, Clone, Deserialize)]
pub enum StaticInput {
    ClosestToExit(Option<f32>),
}
