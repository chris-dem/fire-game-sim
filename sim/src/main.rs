// Global imports (needed for the simulation to run)
use color_eyre::eyre::Result;
mod model;
// TODO intialise with seed

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

pub static DISCRETIZATION: f32 = 10.0 / 1.5;
pub static TOROIDAL: bool = true;

// Main used when only the simulation should run, without any visualization.
#[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
fn main() -> Result<()> {
    // let step = 100;

    // let num_agents = 20;
    // let dim: (f32, f32) = (400., 400.);

    use std::io::BufReader;

    use crate::model::input_handling::{import::ImportImproved, to_sim::ToSimulationStruct};

    let file = fs::File::open("./inputs/tests/base_input.json")?;
    let buf = BufReader::new(file);
    let init: ImportImproved = serde_json::from_reader(buf)?;
    let mut rng = thread_rng();
    let state = init.to_struct(&mut rng, &());

    // let state = Sea::new(dim, num_agents);

    let _ = simulate!(state, 500, 10);
    Ok(())
}

// Main used when a visualization feature is applied.
#[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
fn main() -> Result<()> {
    // Initialize the simulation and its visualization here.

    use std::{fs, io::BufReader};

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
    let file = fs::File::open("./inputs/tests/base_input.json")?;
    let buf = BufReader::new(file);
    let init: ImportImproved = serde_json::from_reader(buf)?;
    let mut rng = thread_rng();
    let state = init.to_struct(&mut rng, &());

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
