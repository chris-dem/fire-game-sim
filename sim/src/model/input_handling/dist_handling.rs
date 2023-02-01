use krabmaga::Distribution;
use rand::RngCore;
use rand_distr::{uniform::SampleUniform, Normal, Uniform};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum AgentDistribution {
    NormalDistr(f32, f32),
    AgentUniform,
}
