use crate::model::misc::misc_func::loc_to_int2d;
use itertools::Itertools;
use krabmaga::engine::agent::Agent;
use std::sync::Mutex;
use std::sync::RwLock;

use super::{
    dynamic_influence::DynamicInfluence, frontier::frontier_struct::Loc,
    static_influence::StaticInfluence,
};

// Cannot be implemented as an agent, due to possible collisions in cells
// Must consider the homoegenuious interaction between neighbouring cells
#[derive(Clone)]
pub struct EvacueeAgent {
    /// Unique identifier
    pub id: usize,
}

/// Implementation of constructor methods
impl EvacueeAgent {
    fn new(id: usize) -> Self {
        Self { id }
    }

    /// Calculate probabilities using neighbouring cells
    /// Simuating the forces acted on a cell being a linear combination of the forces we get:
    /// ```math
    /// force_influence = s_effect * static_influence +  d_effect * dynamic influence
    /// ```
    /// The higher the dynamic_influence, the higher the force incluence.
    /// To get probability distribution use the softmax function across all the neighbours
    pub fn calculate_probabilities(
        &self,
        neigh: &[Loc],
        static_st: &dyn StaticInfluence,
        dynamic_st: &dyn DynamicInfluence,
    ) -> Vec<f32> {
        let all = neigh
            .iter()
            .map(|cs| {
                let cs = loc_to_int2d(cs);
                (static_st.static_influence(&cs) * dynamic_st.dynamic_influence(&cs)).exp()
            })
            .collect_vec();
        let s: f32 = all.iter().sum();
        all.into_iter().map(|el| el / s).collect_vec()
    }
}

impl Agent for EvacueeAgent {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::model::evacuee_mod::{
        dynamic_influence::MockDynamicInfluence, static_influence::MockStaticInfluence,
    };

    use super::*;

    #[test]
    fn test_evaluate_on_empty_cell() {
        let mut static_inf = MockStaticInfluence::new();
        static_inf
            .expect_static_influence()
            .returning(|c| (c.x + c.y) as f32);
        let mut dynamic_inf = MockDynamicInfluence::new();
        dynamic_inf.expect_dynamic_influence().return_const(1.);
        let static_inf = Box::new(static_inf);
        let dynamic_inf = Box::new(dynamic_inf);
        let evac_agent = EvacueeAgent::new(1);
        let probs = evac_agent.calculate_probabilities(
            &[(1, 0), (2, 1), (3, 3)],
            static_inf.as_ref(),
            dynamic_inf.as_ref(),
        );
        let arr = [1.0_f32.exp(), 3.0_f32.exp(), 6.0_f32.exp()];
        let s = arr.iter().sum::<f32>();
        assert_relative_eq!(probs[0], arr[0] / s);
        assert_relative_eq!(probs[1], arr[1] / s);
        assert_relative_eq!(probs[2], arr[2] / s);
    }
}
