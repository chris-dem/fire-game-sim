// Global imports (needed for the simulation to run)
use color_eyre::eyre::Result;
use model::state::InitialConfig;
use serde::Deserialize;
mod import;
mod model;
// TODO evacuee exit
// TODO intialise with seed

#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
use {
    krabmaga::engine::schedule::*, krabmaga::engine::state::State, krabmaga::simulate_old,
    krabmaga::Info, krabmaga::ProgressBar, krabmaga::*, std::time::Duration,
};

// Visualization specific imports
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
use {
    crate::visualization::cell_vis::CellGridVis, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::visualization::Visualization,
};

// #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
mod visualization;

pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() {
    // let step = 100;

    // let num_agents = 20;
    // let dim: (f32, f32) = (400., 400.);

    // let state = Sea::new(dim, num_agents);

    // simulate!(state, step, 10);
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() -> Result<()> {
    // Initialize the simulation and its visualization here.

    use std::{fs, io::BufReader};

    use krabmaga::{
        bevy::prelude::IntoSystem, engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
        thread_rng, visualization::fields::number_grid_2d::BatchRender, Rng,
    };
    use model::state::CellGrid;

    use crate::{
        import::Import,
        model::{
            evacuee_mod::{
                evacuee_cell::EvacueeCell,
                fire_influence::fire_influence::FireInfluence,
                static_influence::{ConstantInfluence, ExitInfluence},
                strategy::Strategy,
            },
            fire_mod::fire_cell::CellType,
            misc::misc_func::Loc,
            state_builder::CellGridBuilder,
        },
    };
    let mut rng = thread_rng();
    let file = fs::File::open("./inputs/tests/base_input.json")?;
    let buf = BufReader::new(file);
    let mut init: Import = serde_json::from_reader(buf)?;
    init.init.initial_grid = (0..10)
        .flat_map(|c| {
            let ret = (0..rng.gen_range(1..6))
                .map(|_| {
                    let x = (c, rng.gen_range(0..10));
                    x
                })
                .collect::<Vec<_>>();
            ret
        })
        .collect();
    init.init.initial_evac_grid = vec![
        EvacueeCell {
            x: 2,
            y: 18,
            pr_c: 0.5,
            strategy: Strategy::Cooperative,
        },
        EvacueeCell {
            x: 2,
            y: 20,
            pr_c: 0.5,
            strategy: Strategy::Competitive,
        },
        EvacueeCell {
            x: 10,
            y: 17,
            pr_c: 0.5,
            strategy: Strategy::Competitive,
        },
        EvacueeCell {
            x: 10,
            y: 11,
            pr_c: 0.5,
            strategy: Strategy::Competitive,
        },
        EvacueeCell {
            x: 3,
            y: 10,
            pr_c: 0.5,
            strategy: Strategy::Competitive,
        },
    ];

    let state = CellGridBuilder::default()
        .initial_config(init.init)
        .dim(init.width, init.height)
        .static_influence(Box::new(ExitInfluence::new(
            1.5,
            &Loc(init.width / 2, init.height + 1),
        )))
        .build();

    let mut app = Visualization::default()
        .with_window_dimensions(800., 600.)
        .with_simulation_dimensions(1.5 * init.width as f32, 1.5 * init.height as f32)
        .with_background_color(Color::BLACK)
        .with_name("Fire evacuation simulator")
        .setup::<CellGridVis, CellGrid>(CellGridVis, state);
    app.add_system(DenseNumberGrid2D::<CellType>::batch_render.system());
    app.add_system(DenseNumberGrid2D::<EvacueeCell>::batch_render.system());
    app.run();
    Ok(())
}
