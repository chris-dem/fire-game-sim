use crate::model::cell::*;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use std::cmp::Eq;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Default Height of the room. Plus 1 for wall
const DEFAULT_HEIGHT: i32 = 51;
/// Default Width of the room. Plus 1 for wall
const DEFAULT_WIDTH: i32 = 51;

/// Initial Configuration of the simulation struct. Will be used to import the map or any other additional information
/// such as parameters
#[derive(Debug, Default, Clone)]
pub struct InitialConfig;

/// Grid in which the simulation will be running on
/// The way the simulation will work is to imploy an external agent in which he will take control of the cells in the simulation.
///
/// Holds current step size, grid, dimensions and initial configuration
///
/// TODO Implememnt state
pub struct CellGrid {
    pub step: u64,
    pub grid: DenseNumberGrid2D<Cell>,
    pub dim: (f32, f32),
    pub initial_config: InitialConfig,
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            step: 0,
            grid: DenseNumberGrid2D::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            dim: (DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32),
            initial_config: Default::default(),
        }
    }
}

/// CellGrid Builder struct. Uses the builder consumer pattern in order to construct a CellGrid.
#[derive(Default)]
pub struct CellGridBuilder {
    step: u64,
    grid: Option<DenseNumberGrid2D<Cell>>,
    dim: Option<(f32, f32)>,
    initial_config: Option<InitialConfig>,
}

impl CellGridBuilder {
    /// Consume current self and return updated CellGrid with new dimensions
    pub fn dim(mut self, w: i32, h: i32) -> Self {
        self.dim = Some((w as f32, h as f32));
        // self.grid = Some(DenseNumberGrid2D::new(w, h));
        self
    }

    pub fn initial_config(mut self, initial_config: InitialConfig) -> Self {
        self.initial_config = Some(initial_config);
        self
    }

    pub fn build(&self) -> CellGrid {
        let dim = self
            .dim
            .clone()
            .unwrap_or((DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32));
        CellGrid {
            step: self.step,
            dim,
            grid: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
            initial_config: self.initial_config.clone().unwrap_or_default(),
        }
    }
}
