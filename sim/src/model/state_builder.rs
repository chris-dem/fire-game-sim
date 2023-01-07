use crate::model::cell::*;
use crate::model::state::*;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;

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
        self
    }

    pub fn initial_config(mut self, initial_config: InitialConfig) -> Self {
        self.initial_config = Some(initial_config);
        self
    }

    pub fn build(&mut self) -> CellGrid {
        let dim = self
            .dim
            .clone()
            .unwrap_or((DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32));
        if let Some(ref v) = self.initial_config {
            assert!(v.initial_grid.len() as i32 == dim.0 as i32 * dim.1 as i32)
        }

        CellGrid {
            step: self.step,
            dim,
            grid: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
            initial_config: self.initial_config.clone().unwrap_or_default(),
        }
    }
}
