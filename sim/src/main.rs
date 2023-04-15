// Global imports (needed for the simulation to run)
use color_eyre::eyre::Result;
use krabmaga::engine::schedule::Schedule;
use model::state::CellGrid;
use rand::{rngs::StdRng, SeedableRng};
pub mod model;
use std::io::BufReader;

use crate::model::{
    arg_handling::MyArgs,
    input_handling::{import::ImportImproved, to_sim::ToSimulationStruct},
};
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use krabmaga::*;

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::cell_vis::CellGridVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "search",
    feature = "bayesian",
    feature = "ga_search"
)))]
fn main() -> Result<()> {
    use clap::Parser;

    let args = MyArgs::parse();

    // let file_name = "fire_spread/f_s_test_val_0.1.json";
    let file_name = args.file_name;

    let file = fs::File::open(format!("./inputs/tests/{}", file_name))?;
    let buf = BufReader::new(file);
    let init: ImportImproved = serde_json::from_reader(buf)?;
    let mut rng = thread_rng();
    let state = init.to_struct(&mut rng, &file_name.to_owned());
    dbg!("STARTING CLI SIMULATION");
    let _ = simulate!(state, 750, 30);
    println!("SIMULATION TERMINATED");
    Ok(())
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() -> Result<()> {
    // Initialize the simulation and its visualization here.

    use std::{fs, io::BufReader};

    use clap::Parser;
    use krabmaga::{
        bevy::prelude::IntoSystem, engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
        thread_rng, visualization::fields::number_grid_2d::BatchRender,
    };
    use model::state::CellGrid;

    use crate::model::{
        evacuee_mod::evacuee_cell::EvacueeCell,
        fire_mod::fire_cell::CellType,
        input_handling::{import::ImportImproved, to_sim::ToSimulationStruct},
    };
    let args = MyArgs::parse();

    let file_name = "fire_spread/f_s_test_val_0.1.json";
    // let file_name = args.file_name;

    let f = dbg!(format!("./inputs/tests/{}", file_name));
    let file = fs::File::open(f)?;
    let buf = BufReader::new(file);
    let init: ImportImproved = serde_json::from_reader(buf)?;
    let mut rng = thread_rng();
    let state = init.to_struct(&mut rng, &file_name.to_owned());

    let mut app = Visualization::default()
        .with_window_dimensions(800., 600.)
        .with_simulation_dimensions(1.5 * state.dim.0 as f32, 1.5 * state.dim.1 as f32)
        .with_background_color(Color::BLACK)
        .with_name("Fire evacuation simulator")
        .setup::<CellGridVis, CellGrid>(CellGridVis, state);
    app.add_system(DenseNumberGrid2D::<CellType>::batch_render.system());
    app.add_system(DenseNumberGrid2D::<EvacueeCell>::batch_render.system());
    app.run();
    Ok(())
}

#[cfg(all(feature = "bayesian", not(feature = "search")))]
fn main() -> Result<()> {
    use krabmaga::explore::bayesian::acquisition_function;

    use crate::model::bayesian_search::search::{get_points, init_parameters, objective};
    println!("============ STARTING BAYESIAN OPTIMIZATION ============");
    let mut op = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("output_eval/bayesian_res.txt")?;

    let iterations: usize = 10;
    for i in 0..10 {
        println!("In iteration {i}");
        let (x, y) = bayesian_search!(init_parameters, objective, get_points, iterations);
        // Currently for all points
        let line = format!(
            "{},{},{},{},{},{},{},{}\n",
            x[0], x[1], x[2], x[3], x[4], x[5], x[6], y
        );
        op.write(line.as_bytes())?;
    }
    Ok(())
}
#[cfg(all(feature = "bayesian", feature = "search"))]
use {chrono::Utc as utc, model::search::InputSearch};

#[cfg(all(feature = "bayesian", feature = "search"))]
fn main() -> Result<()> {
    use krabmaga::engine::state::State;

    // let arg = BayesianArgument::parse();
    // Initialise all non set to set arguments
    let mut op = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!(
            "output_eval/{}",
            utc::now().format("%d_%m_%y_%H_%M.txt")
        ))?;
    op.write("lc,ld,result\n".as_bytes())?;
    let mut rng = RNG.lock().unwrap();
    // let reps = 1;
    let reps = 100;
    let n_step = 500;
    let k = 10;
    let dims = [21, 31, 41, 51];
    for lc in 1..k + 1 {
        let lc = (1. / k as f32) * lc as f32;
        for ld in 1..k + 1 {
            let mut case_all = 0.;
            let mut case_one = 0.;
            let mut case_none = 0.;
            let mut alive_vec = 0.;
            let ld = (1. / k as f32) * ld as f32;
            for _ in 0..reps {
                let n = dims[rng.gen_range(0..4)];
                let x: Vec<f32> = vec![lc, ld, 0.0001, 0.56, 0.7, 1.4, 0.6];
                let mut state = CellGrid::new_training(
                    InputSearch {
                        lc: x[0] as f32,
                        ld: x[1] as f32,
                        asp_infl: x[2] as f32,
                        rat_infl: x[3] as f32,
                        reward_infl: x[4] as f32,
                        static_infl: x[3] as f32,
                        dynamc_infl: x[4] as f32,
                    },
                    n,
                    n,
                );
                // let lc = rng.gen_range(0.1 .. 1.0);
                // let mut dead_vec  = 0.;
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

            let out = alive_num + caseall + caseone - (2. * casenone); // Improve function
            op.write(format!("{lc},{ld},{out}\n").as_bytes())?;
        }
    }
    Ok(())
}

#[cfg(feature = "ga_search")]
fn main() -> Result<()> {
    use crate::model::ga_search::ga_explore::{
        cmp_max, crossover, fitness, init_population, mutation, selection, DESIRED_FITNESS,
        MAX_GENERATIONS, REPETITIONS,
    };
    use krabmaga::engine::state::State;
    use krabmaga::*;

    let result = explore_ga_parallel!(
        init_population,
        fitness,
        selection,
        mutation,
        crossover,
        cmp_max,
        CellGrid,
        DESIRED_FITNESS,
        MAX_GENERATIONS,
        750,
        REPETITIONS,
    );
    if !result.is_empty() {
        // I'm the master
        // build csv from all procexplore_result
        let name = "explore_result".to_string();
        write_csv(&name, &result).expect("Unable to write to csv");
    }
    Ok(())
}
