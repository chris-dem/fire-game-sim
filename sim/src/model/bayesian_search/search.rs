use crate::model::search::InputSearch;
use krabmaga::{engine::schedule::Schedule, lazy_static};
use rand::prelude::*;
use std::sync::Mutex;

lazy_static! {
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
}

pub const ITERATIONS: usize = 50;
pub const INIT_ELEMENTS: usize = 25;
pub const BATCH_SIZE: usize = 1000;

pub fn init_parameters() -> Vec<Vec<f64>> {
    const INITIAL_PARAMS: usize = 20;
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
    let reps = 100;
    let n_step = 750;
    // let lc = rng.gen_range(0.1 .. 1.0);
    let mut state = CellGrid::new_training(
        InputSearch {
            lc: x[0] as f32,
            ld: x[1] as f32,
            asp_infl: x[2] as f32,
            rat_infl: x[3] as f32,
            reward_infl: x[4] as f32,
            // static_infl: 1.5,
            // dynamc_infl: 1.,
            static_infl: x[3] as f32,
            dynamc_infl: x[4] as f32,
        },
        51,
        51,
    );
    let mut alive_vec = 0.;
    // let mut dead_vec  = 0.;
    let mut case_all = 0.;
    let mut case_one = 0.;
    let mut case_none = 0.;
    for _ in 0..ITERATIONS {
        let mut schedule = Schedule::new();
        state.initial_config.fire_spread = Some(rng.gen_range(0.1f32..=0.2f32));
        state.init(&mut schedule);
        for _ in 0..n_step {
            schedule.step(&mut state);
            if state.end_condition(&mut schedule) {
                break;
            }
        }
        let n = state.initial_config.evac_num as f64;
        alive_vec += state.escape_handler.get_escaped_number() as f64 / n;
        // dead_vec  += state.death_handler.get_dead() as f64 / n;
        let sm: u64 = state.output_vars.per_case_ratio_1
            + state.output_vars.per_case_ratio_2
            + state.output_vars.per_case_ratio_3;

        case_all += state.output_vars.per_case_ratio_1 as f64 / sm as f64;
        case_one += state.output_vars.per_case_ratio_2 as f64 / sm as f64;
        case_none += state.output_vars.per_case_ratio_3 as f64 / sm as f64;
    }
    let alive_num = alive_vec as f64 / reps as f64;
    // let dead_num  = dead_vec as f64 / reps as f64;
    let caseall = case_all as f64 / reps as f64;
    let caseone = case_one as f64 / reps as f64;
    let casenone = case_none as f64 / reps as f64;

    alive_num + 0.5 * caseall + 2. * caseone - 1.5 * casenone // Improve function
}

pub fn get_points(_x: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut rng = RNG.lock().unwrap();
    let trial_points: Vec<_> = (0..BATCH_SIZE)
        .into_iter()
        .map(|_| InputSearch::generate_set_of_parameters(&mut *rng))
        .collect();
    trial_points
}
