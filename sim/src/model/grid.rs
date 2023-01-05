use crate::model::cell::*;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::{fields::dense_number_grid_2d::DenseNumberGrid2D, location::Int2D};
use krabmaga::thread_rng;
use rand::RngCore;
// use std::cell::RefCell;
// use std::hash::{Hash, Hasher};

use super::transition::Transition;

/// Default Height of the room. Plus 1 for wall
const DEFAULT_HEIGHT: i32 = 51;
/// Default Width of the room. Plus 1 for wall
const DEFAULT_WIDTH: i32 = 51;

/// Initial Configuration of the simulation struct. Will be used to import the map or any other additional information
/// such as parameters
#[derive(Debug, Default, Clone)]
pub struct InitialConfig {
    initial_grid: Vec<Cell>,
}

/// Grid in which the simulation will be running on
/// The way the simulation will work is to imploy an external agent in which he will take control of the cells in the simulation.
///
/// Holds current step size, grid, dimensions and initial configuration
pub struct CellGrid {
    pub step: u64,
    pub rng: Box<dyn RngCore>,
    pub grid: DenseNumberGrid2D<Cell>,
    pub dim: (f32, f32),
    pub initial_config: InitialConfig,
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            step: 0,
            rng: Box::new(thread_rng()),
            grid: DenseNumberGrid2D::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            dim: (DEFAULT_WIDTH as f32, DEFAULT_HEIGHT as f32),
            initial_config: Default::default(),
        }
    }
}

fn within_bounds(val: i32, limit: i32) -> bool {
    val >= 0 && val < limit
}

impl CellGrid {
    /// Apply InitialConfiguration to the grid
    pub fn set_intial(&mut self) {
        for (indx, val) in self.initial_config.initial_grid.iter().enumerate() {
            self.grid.set_value_location(
                *val,
                &Int2D {
                    x: indx as i32 / self.dim.1 as i32,
                    y: indx as i32 % self.dim.1 as i32,
                },
            )
        }
        self.grid.lazy_update();
    }

    /// Encapsulates the entire fire step
    /// # Arguments
    /// `fire_agent` - Agent that implements the Transition trait. Will be responsbilee for the fire spread
    ///
    pub fn fire_step(&mut self, fire_agent: &impl Transition) {
        let mut updated = Vec::new();
        for x in 0..self.dim.0 as i32 {
            for y in 0..self.dim.1 as i32 {
                let mut n = Vec::with_capacity(8);
                let mut cell = self.grid.get_value(&Int2D { x, y }).unwrap();
                for i in -1..=1 {
                    for j in -1..=1 {
                        if (i == 0 && j == 0)
                            || !within_bounds(x + i, self.dim.0 as i32)
                            || !within_bounds(y + j, self.dim.1 as i32)
                        {
                            continue;
                        }
                        if let Some(c) = self.grid.get_value(&Int2D { x: x + i, y: y + j }) {
                            n.push(c.state);
                        }
                    }
                }
                if cell.spread(fire_agent, &n[..], self.rng.as_mut()) {
                    updated.push((Int2D { x, y }, Cell::new_with_fire(cell.id)));
                } else {
                    updated.push((Int2D { x, y }, cell));
                }
            }
        }

        for (pos, cell) in updated.into_iter() {
            self.grid.set_value_location(cell, &pos)
        }
    }
}

/// CellGrid Builder struct. Uses the builder consumer pattern in order to construct a CellGrid.
#[derive(Default)]
pub struct CellGridBuilder {
    step: u64,
    rng: Option<Box<dyn rand::RngCore>>,
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

    pub fn rng(mut self, rng: Box<dyn RngCore>) -> Self {
        self.rng = Some(rng);
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
        let rng = if let Some(rng) = self.rng.take() {
            rng
        } else {
            Box::new(thread_rng())
        };
        CellGrid {
            step: self.step,
            rng,
            dim,
            grid: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
            initial_config: self.initial_config.clone().unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, cell::RefCell};

    use super::*;
    use crate::model::transition::MockTransition;
    use krabmaga::engine::fields::field::Field;
    use mockall::predicate;
    use rand::SeedableRng;
    use rand_chacha::ChaCha12Rng;

    #[test]
    fn test_fire_step_with_high_spread() {
        let mut grid = CellGridBuilder::default()
            .dim(5, 5)
            .rng(Box::new(ChaCha12Rng::from_seed(Default::default())))
            .initial_config(InitialConfig {
                initial_grid: vec![
                    Cell::new_with_fire(1),
                    Cell::new(2),
                    Cell::new(3),
                    Cell::new(4),
                    Cell::new(5),
                    Cell::new(6),
                    Cell::new(7),
                    Cell::new(8),
                    Cell::new(9),
                    Cell::new(10),
                    Cell::new(11),
                    Cell::new(12),
                    Cell::new(13),
                    Cell::new(14),
                    Cell::new(15),
                    Cell::new(16),
                    Cell::new(17),
                    Cell::new(18),
                    Cell::new(19),
                    Cell::new(20),
                    Cell::new(21),
                    Cell::new(22),
                    Cell::new(23),
                    Cell::new(24),
                    Cell::new(25),
                ],
            })
            .build();
        grid.set_intial();
        let mut fire_agent = MockTransition::new();
        fire_agent
            .expect_transition()
            .with(
                predicate::always(),
                predicate::function(|neigh: &[CellType]| neigh.len() >= 3 && neigh.len() <= 8),
            )
            .returning(|c: &CellType, n: &[CellType]| {
                if *c == CellType::Fire {
                    0.
                } else {
                    let c = n.iter().filter(|c| **c == CellType::Fire).count();
                    if c > 0 {
                        1.
                    } else {
                        0.
                    }
                }
            });
        grid.fire_step(&fire_agent);
        grid.grid.lazy_update();

        let v = RefCell::new(Vec::with_capacity((grid.dim.0 * grid.dim.1) as usize));
        grid.grid.iter_values(|_, c| v.borrow_mut().push(*c));
        let v = v.into_inner();
        assert_eq!(v.len(), 25);
        assert!(vec![
            Cell::new_with_fire(1),
            Cell::new_with_fire(2),
            Cell::new(3),
            Cell::new(4),
            Cell::new(5),
            Cell::new_with_fire(6),
            Cell::new_with_fire(7),
            Cell::new(8),
            Cell::new(9),
            Cell::new(10),
            Cell::new(11),
            Cell::new(12),
            Cell::new(13),
            Cell::new(14),
            Cell::new(15),
            Cell::new(16),
            Cell::new(17),
            Cell::new(18),
            Cell::new(19),
            Cell::new(20),
            Cell::new(21),
            Cell::new(22),
            Cell::new(23),
            Cell::new(24),
            Cell::new(25),
        ]
        .iter()
        .zip(v.into_iter())
        .all(|(c1, c2)| c1.id == c2.id && c1.state == c2.state));
    }

    #[test]
    fn test_fire_step_with_small_spread() {
        let mut grid = CellGridBuilder::default()
            .dim(5, 5)
            .rng(Box::new(ChaCha12Rng::from_seed(Default::default())))
            .initial_config(InitialConfig {
                initial_grid: vec![
                    Cell::new_with_fire(1),
                    Cell::new(2),
                    Cell::new(3),
                    Cell::new(4),
                    Cell::new(5),
                    Cell::new(6),
                    Cell::new(7),
                    Cell::new(8),
                    Cell::new(9),
                    Cell::new(10),
                    Cell::new(11),
                    Cell::new(12),
                    Cell::new(13),
                    Cell::new(14),
                    Cell::new(15),
                    Cell::new(16),
                    Cell::new(17),
                    Cell::new(18),
                    Cell::new(19),
                    Cell::new(20),
                    Cell::new(21),
                    Cell::new(22),
                    Cell::new(23),
                    Cell::new(24),
                    Cell::new(25),
                ],
            })
            .build();
        grid.set_intial();
        let mut fire_agent = MockTransition::new();
        fire_agent
            .expect_transition()
            .with(
                predicate::always(),
                predicate::function(|neigh: &[CellType]| neigh.len() >= 3 && neigh.len() <= 8),
            )
            .returning(|c: &CellType, n: &[CellType]| {
                if *c == CellType::Fire {
                    0.
                } else {
                    let l = n.len() as f32;
                    let c = n.iter().filter(|c| **c == CellType::Fire).count() as f32;
                    c / l * 2.5
                }
            });
        grid.fire_step(&fire_agent);
        grid.grid.lazy_update();

        let v = RefCell::new(Vec::with_capacity((grid.dim.0 * grid.dim.1) as usize));
        grid.grid.iter_values(|_, c| v.borrow_mut().push(*c));
        let v = v.into_inner();
        assert_eq!(v.len(), 25);
        assert!(vec![
            Cell::new_with_fire(1),
            Cell::new_with_fire(2),
            Cell::new(3),
            Cell::new(4),
            Cell::new(5),
            Cell::new(6),
            Cell::new(7),
            Cell::new(8),
            Cell::new(9),
            Cell::new(10),
            Cell::new(11),
            Cell::new(12),
            Cell::new(13),
            Cell::new(14),
            Cell::new(15),
            Cell::new(16),
            Cell::new(17),
            Cell::new(18),
            Cell::new(19),
            Cell::new(20),
            Cell::new(21),
            Cell::new(22),
            Cell::new(23),
            Cell::new(24),
            Cell::new(25),
        ]
        .iter()
        .zip(v.into_iter())
        .all(|(c1, c2)| c1.id == c2.id && c1.state == c2.state));
    }
}
