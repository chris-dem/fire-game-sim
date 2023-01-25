use super::fire_influence::dynamic_influence::DynamicInfluence;
use super::fire_influence::fire_influence::FireInfluence;
use super::fire_influence::frontier::{Frontier, FrontierStructure};
use super::strategy::Strategy;
use super::{evacuee_cell::EvacueeCell, static_influence::StaticInfluence};
use crate::model::misc::misc_func::Loc;
use crate::model::state::CellGrid;
use itertools::Itertools;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::rand as krand;

// Cannot be implemented as an agent, due to possible collisions in cells
// Must consider the homoegenuious interaction between neighbouring cells
#[derive(Clone, Default)]
pub struct EvacueeAgent {
    /// Unique identifier
    pub id: usize,
    pub lc: f32,
    pub ld: f32,
}

/// Implementation of constructor methods
impl EvacueeAgent {
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
        fire_infl: &FireInfluence,
    ) -> Vec<f32> {
        let all = neigh
            .iter()
            .map(|cs| {
                let d = fire_infl.get_movement_influence(&cs);
                let s = static_st.static_influence(&Int2D::from(*cs));
                (-s + d).exp()
            })
            .collect_vec();
        let s: f32 = all.iter().sum();
        all.into_iter().map(|el| el / s).collect_vec()
    }

    pub fn calculate_strategies(
        &self,
        EvacueeCell { pr_c, strategy, .. }: &EvacueeCell,
        stim: f32,
    ) -> f32 {
        let con = if *strategy == Strategy::Cooperative {
            self.lc
        } else {
            self.ld
        };
        if stim.is_sign_positive() {
            pr_c + (1. - pr_c) * con * stim
        } else {
            (1. + con * stim) * pr_c
        }
    }
}

impl Agent for EvacueeAgent {
    fn step(&mut self, state: &mut dyn krabmaga::engine::state::State) {
        let state = state.as_any_mut().downcast_mut::<CellGrid>().unwrap();
        let mut rng = krand::thread_rng();

        state.evacuee_step(self, &mut rng);
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::model::evacuee_mod::{
        fire_influence::{
            dynamic_influence::MockDynamicInfluence, frontier::MockFrontierStructure,
        },
        static_influence::MockStaticInfluence,
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
        let front_struct = Box::new(MockFrontierStructure::new());
        let fire_infl = FireInfluence {
            fire_state: front_struct,
            movement: dynamic_inf,
            ..Default::default()
        };
        let evac_agent = EvacueeAgent::default();
        let probs = evac_agent.calculate_probabilities(
            &[Loc(1, 0), Loc(2, 1), Loc(3, 3)],
            static_inf.as_ref(),
            &fire_infl,
        );
        let arr = [1.0_f32.exp(), 3.0_f32.exp(), 6.0_f32.exp()];
        let s = arr.iter().sum::<f32>();
        assert_relative_eq!(probs[0], arr[0] / s);
        assert_relative_eq!(probs[1], arr[1] / s);
        assert_relative_eq!(probs[2], arr[2] / s);
    }
}
