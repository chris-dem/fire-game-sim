// Global imports (needed for the simulation to run)
use color_eyre::eyre::Result;
use model::state::InitialConfig;
use serde::Deserialize;
mod model;

#[derive(Debug, Deserialize)]
struct Import {
    init: InitialConfig,
    width: i32,
    height: i32,
}

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
        visualization::fields::number_grid_2d::BatchRender,
    };
    use model::state::CellGrid;

    use crate::model::{cell::CellType, state::InitialConfig, state_builder::CellGridBuilder};

    let file = fs::File::open("./inputs/tests/base_input.json")?;
    let buf = BufReader::new(file);
    let init: Import = serde_json::from_reader(buf)?;

    let state = CellGridBuilder::default()
        .initial_config(init.init)
        .dim(init.width, init.height)
        .build();

    let mut app = Visualization::default()
        .with_window_dimensions(800., 600.)
        .with_simulation_dimensions(1.5 * init.width as f32, 1.5 * init.height as f32)
        .with_background_color(Color::BLACK)
        .with_name("Template")
        .setup::<CellGridVis, CellGrid>(CellGridVis, state);
    app.add_system(DenseNumberGrid2D::batch_render.system());
    app.run();
    Ok(())
}
