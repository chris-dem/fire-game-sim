use crate::model::cell::*;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::state::State;
use krabmaga::engine::{fields::dense_number_grid_2d::DenseNumberGrid2D, location::Int2D};
use rand::RngCore;
use serde::Deserialize;

use super::transition::Transition;
use crate::model::fire_mod::fire_spread::FireRules;

/// Default Height of the room. Plus 1 for wall
pub const DEFAULT_HEIGHT: u32 = 51;
/// Default Width of the room. Plus 1 for wall
pub const DEFAULT_WIDTH: u32 = 51;

/// Initial Configuration of the simulation struct. Will be used to import the map or any other additional information
/// such as parameters
#[derive(Debug, Default, Clone, Deserialize)]
pub struct InitialConfig {
    pub initial_grid: Vec<CellType>,
    pub fire_spread: f32,
}

/// Grid in which the simulation will be running on
/// The way the simulation will work is to imploy an external agent in which he will take control of the cells in the simulation.
///
/// Holds current step size, grid, dimensions and initial configuration
pub struct CellGrid {
    pub step: u64,
    pub grid: DenseNumberGrid2D<CellType>,
    pub dim: (u32, u32),
    pub initial_config: InitialConfig,
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            step: 0,
            grid: DenseNumberGrid2D::new(DEFAULT_WIDTH as i32, DEFAULT_HEIGHT as i32),
            dim: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            initial_config: Default::default(),
        }
    }
}

pub fn within_bounds(val: i32, limit: i32) -> bool {
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
        // self.grid.update();
    }

    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    pub fn fire_step(&mut self, fire_agent: &impl Transition, rng: &mut impl RngCore) {
        let mut updated = Vec::new();
        for x in 0..self.dim.0 as i32 {
            for y in 0..self.dim.1 as i32 {
                let mut n = Vec::with_capacity(8);
                let cell = self.grid.get_value(&Int2D { x, y }).unwrap();
                for i in -1..=1 {
                    for j in -1..=1 {
                        if (i == 0 && j == 0)
                            || !within_bounds(x + i, self.dim.0 as i32)
                            || !within_bounds(y + j, self.dim.1 as i32)
                        {
                            continue;
                        }
                        if let Some(c) = self.grid.get_value(&Int2D { x: x + i, y: y + j }) {
                            n.push(c);
                        }
                    }
                }
                if cell.spread(fire_agent, &n[..], rng) {
                    updated.push((Int2D { x, y }, CellType::Fire));
                } else {
                    updated.push((Int2D { x, y }, cell));
                }
            }
        }

        for (pos, cell) in updated.into_iter() {
            self.grid.set_value_location(cell, &pos)
        }
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    /// Encapsulates the entire fire step
    /// # Arguments
    /// `fire_agent` - Agent that implements the Transition trait. Will be responsbilee for the fire spread
    ///
    pub fn fire_step(&mut self, fire_agent: &impl Transition, rng: &mut impl RngCore) {
        let updated: RefCell<Vec<(Int2D, CellType)>> = RefCell::new(Vec::new());
        let rng = RefCell::new(thread_rng());
        self.grid.iter_values(|&Int2D { x, y }, cell| {
            let mut n = Vec::with_capacity(8);
            for i in -1..=1 {
                for j in -1..=1 {
                    if (i == 0 && j == 0)
                        || !within_bounds(x + i, self.dim.0 as i32)
                        || !within_bounds(y + j, self.dim.1 as i32)
                    {
                        continue;
                    }
                    if let Some(c) = self.grid.get_value(&Int2D { x: x + i, y: y + j }) {
                        n.push(c);
                    }
                }
            }
            if cell.spread(fire_agent, &n[..], &mut *rng.borrow_mut()) {
                updated.borrow_mut().push((Int2D { x, y }, CellType::Fire));
            } else {
                updated.borrow_mut().push((Int2D { x, y }, cell));
            }
        });

        for (pos, cell) in updated.take().into_iter() {
            self.grid.set_value_location(cell, &pos)
        }
    }
}

impl State for CellGrid {
    fn update(&mut self, step: u64) {
        self.step = step;
        self.grid.lazy_update();
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn reset(&mut self) {
        self.step = 0;
        self.grid = DenseNumberGrid2D::new(self.dim.0 as i32, self.dim.1 as i32);
    }

    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.reset();
        self.set_intial();
        let fire_rules = FireRules {
            spread: self.initial_config.fire_spread,
            id: 1,
        };
        self.grid.update();
        schedule.schedule_repeating(Box::new(fire_rules), 0., 0);
    }
}

#[cfg(all(test, any(feature = "visualization", feature = "visualization_wasm")))]
mod tests {

    use super::*;
    use crate::model::state_builder::CellGridBuilder;
    use crate::model::transition::MockTransition;
    use itertools::Itertools;
    use krabmaga::engine::fields::field::Field;
    use mockall::predicate;
    use rand::SeedableRng;
    use rand_chacha::ChaCha12Rng;

    #[test]
    fn test_fire_step_with_high_spread() {
        let mut grid = CellGridBuilder::default()
            .dim(5, 5)
            // .rng(Box::new(ChaCha12Rng::from_seed(Default::default())))
            .initial_config(InitialConfig {
                fire_spread: 0.7,
                initial_grid: vec![
                    CellType::Fire,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                ],
            })
            .build();
        grid.set_intial(); // Create initial config and move values
        grid.grid.lazy_update();
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
        let mut rng = ChaCha12Rng::from_seed(Default::default());
        grid.fire_step(&fire_agent, &mut rng);
        grid.grid.lazy_update();

        let mut v = vec![];
        (0..grid.grid.width)
            .cartesian_product(0..grid.grid.height)
            .for_each(|(x, y)| {
                let c = grid.grid.get_value(&Int2D { x, y }).unwrap();
                v.push(c);
            });
        assert_eq!(v.len(), 25);
        assert!(vec![
            CellType::Fire,  //1
            CellType::Fire,  //2
            CellType::Empty, //3
            CellType::Empty, //4
            CellType::Empty, //5
            CellType::Fire,  //6
            CellType::Fire,  //7
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
        ]
        .into_iter()
        .zip_eq(v.into_iter())
        .all(|(c1, c2)| c1 == c2));
    }

    #[test]
    fn test_fire_step_with_small_spread() {
        let mut grid = CellGridBuilder::default()
            .dim(5, 5)
            .initial_config(InitialConfig {
                fire_spread: 0.7,
                initial_grid: vec![
                    CellType::Fire,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                    CellType::Empty,
                ],
            })
            .build();
        grid.set_intial();
        grid.grid.lazy_update();
        let mut fire_agent = MockTransition::new();
        fire_agent
            .expect_transition()
            .with(
                predicate::always(),
                predicate::function(|neigh: &[CellType]| neigh.len() >= 3 && neigh.len() <= 8),
            )
            .return_const(0.);
        let mut rng = ChaCha12Rng::seed_from_u64(10);
        grid.fire_step(&fire_agent, &mut rng);
        grid.grid.lazy_update();

        let mut v = vec![];
        (0..grid.grid.width)
            .cartesian_product(0..grid.grid.height)
            .for_each(|(x, y)| {
                let c = grid.grid.get_value(&Int2D { x, y }).unwrap();
                v.push(c);
            });
        assert_eq!(v.len(), 25);
        assert!(vec![
            CellType::Fire,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
            CellType::Empty,
        ]
        .into_iter()
        .zip_eq(v.into_iter())
        .all(|(c1, c2)| c1 == c2));
    }
}
