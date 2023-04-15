use core::panic;
use rand::prelude::*;
use rand::seq::SliceRandom;
use std::sync::RwLock;

use crate::model::{search::InputSearch, state::CellGrid};
use itertools::Itertools;
use krabmaga::{engine::schedule::Schedule, *};

pub const INIT_POPULATION: usize = 100;
pub const MAX_MUTATION: f64 = 0.05;
pub const REPETITIONS: u32 = 5;
pub const DNA_SIZE: usize = 7;
pub const DESIRED_FITNESS: f32 = 10.;
pub const MAX_GENERATIONS: u32 = 500;
pub const ALPHA: f32 = 0.1;

lazy_static! {
    pub static ref TEMPERATURE: Arc<RwLock<u32>> = Arc::new(RwLock::new(1));
    pub static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::from_entropy());
}

/// Maximum comparator
pub fn cmp_max(fitness1: &f32, fitness2: &f32) -> bool {
    *fitness1 > *fitness2
}

pub fn init_population() -> Vec<String> {
    let mut population = Vec::with_capacity(INIT_POPULATION);
    for _ in 0..INIT_POPULATION {
        let mut v = Vec::with_capacity(DNA_SIZE);
        {
            let mut rng = RNG.lock().unwrap();
            let generate = InputSearch::generate_set_of_parameters(&mut *rng);
            v.extend(generate.into_iter().map(|c| c.to_string()))
        }
        population.push(v.join(";"));
    }
    population
}

// Pie based
pub fn selection(population_fitness: &mut Vec<(String, f32)>) {
    let (_, fmax) = population_fitness
        .iter()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    let fmax = *fmax;
    let range = {
        let temp = TEMPERATURE.read().unwrap();
        (1. - (*temp as f32 / REPETITIONS as f32)).max(0.1)
    };
    population_fitness.retain(|(_, el)| *el >= fmax * (1. - range));
    population_fitness
        .sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    {
        let mut val = TEMPERATURE.write().unwrap();
        *val = (*val + 1).min(REPETITIONS as u32);
    }
}

pub fn crossover(population: &mut Vec<String>) {
    if population.len() == 0 {
        panic!("Population is empty")
    }
    let mut new_population = Vec::with_capacity(INIT_POPULATION);
    for _ in 0..INIT_POPULATION {
        let (par1, par2): (Vec<f32>, Vec<f32>) = population
            .iter()
            .enumerate()
            .collect_vec()
            .choose_multiple_weighted(&mut *RNG.lock().unwrap(), 2, |(rank, _)| {
                (*rank as f64 + 1.) / 5.
            })
            .unwrap()
            .map(|(_, el)| {
                el.split(";")
                    .map(|f| f.parse::<f32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect_tuple()
            .unwrap();
        new_population.push(
            par1.into_iter()
                .zip(par2)
                .enumerate()
                .map(|(ind, (el1, el2))| {
                    let (top_val, small_val) = maxmin(ind);
                    let rang = (el2 - el1).abs();
                    let pmin = (el2.min(top_val) - rang * ALPHA).max(small_val);
                    let pmax = (el2.max(small_val) + rang * ALPHA).min(top_val);
                    RNG.lock().unwrap().gen_range(pmin..=pmax).to_string()
                })
                .collect::<Vec<_>>()
                .join(";"),
        );
    }
    *population = new_population;
}

pub fn fitness(computed_ind: &mut Vec<(CellGrid, Schedule)>) -> f32 {
    let mut alive_vec = 0.;
    // let mut dead_vec  = 0.;
    let mut case_all = 0.;
    let mut case_one = 0.;
    let mut case_none = 0.;
    for (state, _) in computed_ind.iter() {
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
    let reps = computed_ind.len();
    let alive_num = alive_vec as f32 / reps as f32;
    let caseall = case_all as f32 / reps as f32;
    let caseone = case_one as f32 / reps as f32;
    let casenone = case_none as f32 / reps as f32;

    alive_num + 0.5 * caseall + 2. * caseone - 1.5 * casenone // Improve function
}

fn maxmin(val: usize) -> (f32, f32) {
    match val {
        0..=1 => (1., 0.005),
        2..=4 => (1., 0.),
        5..=6 => (2., 0.5),
        _ => unreachable!("Unreachable code"),
    }
}

pub fn mutation(indiv: &mut String) {
    *indiv = indiv
        .split(";")
        .enumerate()
        .map(|(ind, c)| {
            let (big, small) = maxmin(ind);
            let c = c.parse::<f32>().unwrap();
            RNG.lock()
                .unwrap()
                .gen_range((c - ALPHA).max(small)..(c + ALPHA).min(big))
                .to_string()
        })
        .collect_vec()
        .join(";");
}

#[cfg(test)]
mod tests {
    use krabmaga::{thread_rng, Rng};
    use rand::RngCore;

    use super::*;

    fn generate_random_population(
        rng: &mut dyn RngCore,
        pop_size: usize,
        dna_length: usize,
    ) -> Vec<String> {
        (0..pop_size)
            .map(|_| {
                (0..dna_length)
                    .map(|_| rng.gen_range(0. ..=1.).to_string())
                    .collect_vec()
                    .join(";")
            })
            .collect_vec()
    }

    #[test]
    fn expect_correct_order() {
        let mut rng = thread_rng();
        let f1 = rng.gen_range(0..50) as f32;
        let f2 = f1 * 10. + 10.;
        assert!(!cmp_max(&f1, &f2))
    }

    mod init_population_tests {

        use krabmaga::*;

        use crate::model::ga_search::ga_explore::{init_population, DNA_SIZE, INIT_POPULATION};

        #[test]
        fn init_population_test_length() {
            let v = init_population();

            assert_eq!(
                v.len(),
                INIT_POPULATION,
                "Must contain {} elements",
                INIT_POPULATION
            );
        }

        #[test]
        fn init_population_element_test() {
            let v = init_population();
            assert!(
                v.into_iter()
                    .map(|c| c.split(";").count())
                    .all(|c| c == DNA_SIZE),
                "DNA size is not {DNA_SIZE}"
            )
        }
    }

    mod test_crossover {
        use rand::{rngs::SmallRng, SeedableRng};

        use crate::model::ga_search::ga_explore::{
            crossover, tests::generate_random_population, INIT_POPULATION,
        };

        #[test]
        pub fn crossover_size() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size = 150;
            let dna_size = 7;
            let mut population = generate_random_population(&mut rng, pop_size, dna_size);
            crossover(&mut population);
            assert_eq!(population.len(), INIT_POPULATION);
        }

        #[test]
        pub fn crossover_shape_size() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size = 150;
            let dna_size = 7;
            let mut population = generate_random_population(&mut rng, pop_size, dna_size);
            // DNA SIZE must be 10
            crossover(&mut population);
            assert!(population
                .into_iter()
                .all(|s| s.matches(";").count() + 1 == dna_size))
        }

        #[test]
        pub fn crossover_value_range() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size = 150;
            let dna_size = 7;
            let mut population = generate_random_population(&mut rng, pop_size, dna_size);
            // DNA SIZE must be between
            crossover(&mut population);
            assert!(population.into_iter().all(|s| s.split(";").all(|c| {
                if let Ok(val) = c.parse::<f32>() {
                    val >= 0.
                } else {
                    false
                }
            })))
        }
    }

    mod test_selection {
        use super::*;

        fn prepare_selection_population(
            rng: &mut impl RngCore,
            pop_size: usize,
            dna_length: usize,
            percentile: f32,
        ) -> Vec<(String, f32)> {
            let max_f: f32 = rng.gen();
            let samples = generate_random_population(rng, pop_size, dna_length);
            let vals =
                (0..pop_size).map(|_| max_f * (1.0f32 - percentile * rng.gen_range(0. ..=1.0f32)));
            samples.into_iter().zip(vals).collect_vec()
        }
        #[test]
        fn expect_full_selection() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size: usize = 50;
            let dna_length = 7;
            let mut samples = prepare_selection_population(&mut rng, pop_size, dna_length, 0.5);
            selection(&mut samples);
            assert_eq!(samples.len(), pop_size)
        }

        #[test]
        #[ignore = "Due to mutex variable"]
        fn expect_half_selection() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size: usize = 50;
            let dna_length = 7;
            *TEMPERATURE.write().unwrap() = REPETITIONS as u32 - 2;
            let mut samples = prepare_selection_population(&mut rng, pop_size, dna_length, 0.9);
            selection(&mut samples);
            assert_ne!(samples.len(), pop_size)
        }

        #[test]
        fn expect_sorted() {
            let mut rng = SmallRng::seed_from_u64(250);
            let pop_size: usize = 50;
            let dna_length = 7;
            let mut samples = prepare_selection_population(&mut rng, pop_size, dna_length, 0.9);
            selection(&mut samples);
            let assertion = samples.windows(2).all(|win| {
                win[0]
                    .1
                    .partial_cmp(&win[1].1)
                    .map(|ord| match ord {
                        std::cmp::Ordering::Greater => false,
                        _ => true,
                    })
                    .unwrap_or(true)
            });
            assert!(assertion)
        }
    }
}
