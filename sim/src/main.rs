// Global imports (needed for the simulation to run)
// use crate::model::sea::Sea;
mod model;

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
fn main() {
    // Initialize the simulation and its visualization here.

    use krabmaga::{
        bevy::prelude::IntoSystem, engine::fields::dense_number_grid_2d::DenseNumberGrid2D,
        visualization::fields::number_grid_2d::BatchRender,
    };
    use model::state::CellGrid;

    use crate::model::{
        cell::Cell,
        state::{InitialConfig, DEFAULT_HEIGHT, DEFAULT_WIDTH},
        state_builder::CellGridBuilder,
    };

    let w = 50;
    let h = 50;
    let mut map: Vec<Cell> = (0..w * h).map(|c| Cell::new(c as usize)).collect();
    map[0] = Cell::new_with_fire(0);

    let init = InitialConfig {
        fire_spread: 0.7,
        initial_grid: map,
    };

    let state = CellGridBuilder::default()
        .initial_config(init)
        .dim(w, h)
        .build();

    let mut app = Visualization::default()
        .with_window_dimensions(800., 600.)
        .with_simulation_dimensions(1.5 * w as f32, 1.5 * h as f32)
        .with_background_color(Color::BLACK)
        .with_name("Template")
        .setup::<CellGridVis, CellGrid>(CellGridVis, state);
    app.add_system(DenseNumberGrid2D::batch_render.system());
    app.run()
}
