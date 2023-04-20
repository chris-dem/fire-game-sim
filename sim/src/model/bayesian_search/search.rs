use crate::model::{ga_search::ga_explore::fitness, search::InputSearch};
use krabmaga::{engine::schedule::Schedule, lazy_static};
use rand::prelude::*;
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
}

pub const ITERATIONS: usize = 50;
pub const INIT_ELEMENTS: usize = 250;
pub const BATCH_SIZE: usize = 3000;
pub const INITIAL_PARAMS: usize = INIT_ELEMENTS;

pub fn init_parameters() -> Vec<Vec<f64>> {
    let mut ret = Vec::with_capacity(INITIAL_PARAMS);
    /* Non-sim params
     * lc
     * ld
     * asp_def
     * reward_r
     * root_r
     * static_infl
     * dynamic_infl
     */
    let mut rng = RNG.lock().unwrap();

    for _ in 0..INITIAL_PARAMS {
        ret.push(InputSearch::generate_set_of_parameters(&mut *rng));
    }
    ret
}

pub fn objective(x: &Vec<f64>) -> f64 {
    use krabmaga::engine::state::State;

    use crate::model::state::CellGrid;
    let mut rng = RNG.lock().unwrap();
    let n_step = 750;
    let mut computed_results = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        let mut state = CellGrid::new_training(
            InputSearch::from_float_vertor(x, 51. * 2.0f32.sqrt()),
            51,
            51,
        );
        let mut schedule = Schedule::new();
        state.initial_config.fire_spread = Some(rng.gen_range(0.1f32..=0.2f32));
        state.init(&mut schedule);
        for _ in 0..n_step {
            schedule.step(&mut state);
            if state.end_condition(&mut schedule) {
                break;
            }
        }
        computed_results.push((state, schedule));
    }
    fitness(&mut computed_results) as f64
}

pub fn get_points(_x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut rng = RNG.lock().unwrap();
    let trial_points: Vec<_> = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| InputSearch::generate_set_of_parameters(&mut *rng))
        .collect();
    trial_points
}

// proptest! {
//     #[test]
//     fn test_param_dims() {}
// }

#[cfg(test)]
mod tests {
    use crate::model::ga_search::ga_explore::DNA_SIZE;

    use super::*;

    use super::init_parameters;

    #[test]
    fn test_init_params_dims() {
        let res = init_parameters();
        assert_eq!(res.len(), INITIAL_PARAMS);
        assert!(res.iter().all(|el| el.len() == DNA_SIZE))
    }
}
