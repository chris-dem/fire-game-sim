use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum AgentDistribution {
    NormalDistr(f32, f32),
    AgentUniform,
}
