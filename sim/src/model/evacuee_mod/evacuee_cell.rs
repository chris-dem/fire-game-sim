use serde::Deserialize;

use crate::model::evacuee_mod::strategy::Strategy;

#[derive(Debug, Clone, Copy, Deserialize, rand_derive2::RandGen)]
pub struct EvacueeCell {
    pub strategy: Strategy,
    pub x: i32,
    pub y: i32,
    /// probabillity of adopting strategy
    pub pr_c: f32,
}
