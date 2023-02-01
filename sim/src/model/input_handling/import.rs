use rand::prelude::*;
use rand_distr::Distribution;
use serde::Deserialize;

use crate::model::{
    evacuee_mod::strategy::Strategy,
    state::{InitialConfig, SimType},
};

use super::fire_input::FireInput;

#[derive(Debug, Deserialize)]
pub struct Import {
    pub init: InitialConfig,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct ImportImproved {
    pub dim: (u32, u32),
    pub sim_type: SimType,
    pub setup: Setup,
    pub fire: FireInput,
    pub escape: EscapeInput,
    pub death: DeathInput,
    pub static_input: StaticInput,
}

#[derive(Debug, Clone, Deserialize)]
pub enum FixedOrRandom<T> {
    Fixed(T),
    Random,
}

impl FixedOrRandom<f32> {
    fn get_f32(&self, rng: &mut dyn RngCore) -> f32 {
        match self {
            FixedOrRandom::Fixed(f) => *f,
            FixedOrRandom::Random => rng.gen(),
        }
    }
}

impl FixedOrRandom<f32> {
    pub fn get_val(&self, rng: &mut dyn RngCore) -> f32 {
        match self {
            Self::Fixed(f) => *f,
            Self::Random => rng.gen(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Setup {
    pub map_seed: Option<u64>,
    pub initial_locs: FixedOrRandom<(Vec<(i32, i32)>, Vec<(i32, i32)>)>,
    pub evac_number: usize,
    pub initial_evac_strats: FixedOrRandom<Vec<Strategy>>,
    pub initial_evac_prob_dist: FixedOrRandom<f32>,
    pub fire_spread: FixedOrRandom<f32>,
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
    ClosestToExit(FixedOrRandom<f32>),
}
