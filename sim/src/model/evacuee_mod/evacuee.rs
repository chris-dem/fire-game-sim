use super::fire_influence::fire_influence::FireInfluence;
use super::strategy::Strategy;
use super::{evacuee_cell::EvacueeCell, static_influence::StaticInfluence};
use crate::model::misc::misc_func::Loc;
use crate::model::state::CellGrid;
use itertools::Itertools;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::{rand as krand, Rng};
use krand::rngs::StdRng;
use krand::{RngCore, SeedableRng};

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
                let result = -s + d;
                let sign = result.signum();
                (result.abs().sqrt() * sign).exp() // Use sqrt since long distances can skyrocket the exponetial value
            })
            .collect_vec();
        let s: f32 = all.iter().sum();
        all.into_iter().map(|el| el / s).collect_vec()
    }

    pub fn calculate_strategies(&self, evac: &mut EvacueeCell, rng: &mut dyn RngCore, stim: f32) {
        match evac.strategy {
            Strategy::Competitive => {
                // ADOPT FOR COOP
                evac.pr_d = calc_prob(evac.pr_d, self.ld, stim);
                if !rng.gen_bool(evac.pr_d as f64) {
                    evac.strategy = Strategy::Cooperative;
                }
            }
            Strategy::Cooperative => {
                // ADOPT FOR COOP
                evac.pr_c = calc_prob(evac.pr_c, self.lc, stim);
                if !rng.gen_bool(evac.pr_c as f64) {
                    evac.strategy = Strategy::Competitive;
                }
            }
        }
    }
}

fn calc_prob(prob: f32, learning: f32, stim: f32) -> f32 {
    if stim.is_sign_positive() {
        prob + (1. - prob) * learning * stim
    } else {
        prob * (1. + learning * stim)
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
    use super::*;
    use crate::model::{
        evacuee_mod::{
            fire_influence::{
                dynamic_influence::MockDynamicInfluence, frontier::MockFrontierStructure,
            },
            static_influence::MockStaticInfluence,
        },
        misc::misc_func::relative_eq_close,
    };
    use proptest::prelude::*;
    use rand::prelude::*;

    proptest! {
        #[test]
        fn test_calculate_prop(arr in proptest::array::uniform8((0i32..50,0i32..50).prop_map(|(x,y)| Loc(x,y))), exit in (0i32..50, 0i32..50).prop_map(|(x,y)| Loc(x,y))) {
            let evac = EvacueeAgent::default();
            let mut stat = MockStaticInfluence::new();
            stat.expect_static_influence()
                .returning(move |Int2D {x,y} : &Int2D| {
                    let exit = exit;
                    ((*x - exit.0).abs() as f32 + (*y - exit.1).abs() as f32).ln_1p()
                });
            let fire_infl = FireInfluence::default();
            let from_evac = evac.calculate_probabilities(&arr, &stat, &fire_infl);
            let from_arr = arr.iter().map(|l| {
                let s = stat.static_influence(&Into::into(*l));
                let d = fire_infl.get_movement_influence(&Into::into(*l));
                let arg = -s + d;
                (arg.abs().sqrt() * arg.signum()).exp()
            }).collect_vec();

            let s : f32 = from_arr.iter().sum::<f32>();
            let from_arr  : Vec<f32> = from_arr.into_iter().map(|x| x / s).collect_vec();
            let x = from_evac.iter().zip(from_arr.iter()).all(|(a,b)|relative_eq_close(*a,*b));
            prop_assert!(x);
        }

        #[test]
        fn test_probability(lc in 0.0f32..1.0f32, ld in 0.0f32..1.0f32, prob_c in 0.0f32..1.0f32, prob_d in 0.0f32..1.0f32, st in -1.0f32..=1.0f32, strat in 0..=1u32) {
            let mut strategy = if strat == 0 { crate::model::evacuee_mod::strategy::Strategy::Cooperative } else { crate::model::evacuee_mod::strategy::Strategy::Competitive };
            let mut rng = StdRng::seed_from_u64(50);
            let evac = EvacueeAgent {
                id : 1,
                lc,
                ld
            };

            let mut evac_cell = EvacueeCell {
                strategy,
                x : 0,
                y : 0,
                pr_c : prob_c,
                pr_d : prob_d,
            };

            evac.calculate_strategies(&mut evac_cell, &mut rng, st);
            let mut pr_d = prob_d;
            let mut pr_c = prob_c;
            let mut rng = StdRng::seed_from_u64(50);
            match strategy {
                crate::model::evacuee_mod::strategy::Strategy::Competitive => {
                    // ADOPT FOR COOP
                    pr_d = calc_prob(pr_d,ld, st);
                    if !rng.gen_bool(pr_d as f64) {
                       strategy = crate::model::evacuee_mod::strategy::Strategy::Cooperative;
                    }
                }
                crate::model::evacuee_mod::strategy::Strategy::Cooperative => {
                    // ADOPT FOR COOP
                    pr_c = calc_prob(pr_c, lc, st);
                    if !rng.gen_bool(pr_c as f64) {
                        strategy = crate::model::evacuee_mod::strategy::Strategy::Competitive;
                    }
                }
            }

            prop_assert_eq!(strategy, evac_cell.strategy);
            prop_assert!(relative_eq_close(pr_c, evac_cell.pr_c));
            prop_assert!(relative_eq_close(pr_d, evac_cell.pr_d));
        }
    
    
    }
}
