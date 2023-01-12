use std::fmt;

use super::strategy::Strategy;
impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Cannot be implemented as an agent, due to possible collisions in cells
// Must consider the homoegenuious interaction between neighbouring cells
#[derive(Debug)]

pub struct Evacuee {
    /// strategy for the evacuee
    pub strategy: Strategy,
    /// Unique identifier
    pub id: usize,
    /// TODO Static measurement
    pub static_influence: Option<u32>,
    /// TODO Dynamic measurement
    pub dynamic_influence: Option<u32>,
}

impl Default for Evacuee {
    fn default() -> Self {
        Self {
            strategy: Strategy::Cooperative,
            id: 1,
            static_influence: None,
            dynamic_influence: None,
        }
    }
}
