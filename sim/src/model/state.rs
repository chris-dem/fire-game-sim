use std::borrow::BorrowMut;
use std::cell::RefCell;

use crate::model::evacuee_mod::strategy;
use crate::model::evacuee_mod::strategy::strategy_rewards;
use crate::model::fire_mod::fire_cell::*;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::state::State;
use krabmaga::engine::{fields::dense_number_grid_2d::DenseNumberGrid2D, location::Int2D};
use krabmaga::{Distribution, HashMap, Rng};
use rand::distributions::WeightedIndex;
use rand::seq::SliceRandom;
use rand::RngCore;
use serde::Deserialize;

use super::evacuee_mod::dynamic_influence::{ClosestDistance, DynamicInfluence};
use super::evacuee_mod::evacuee::EvacueeAgent;
use super::evacuee_mod::evacuee_cell::EvacueeCell;
use super::evacuee_mod::frontier::frontier_struct::Loc;
use super::evacuee_mod::static_influence::{ExitInfluence, StaticInfluence};
use super::evacuee_mod::strategies::aspiration_strategy::{AspirationStrategy, LogAspManip};
use super::evacuee_mod::strategies::ratio_strategy::{RatioStrategy, RootDist};
use super::evacuee_mod::strategy::{s_x, Strategy};
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
    pub initial_evac_grid: Vec<EvacueeCell>,
    pub fire_spread: f32,
}

/// Grid in which the simulation will be running on
/// The way the simulation will work is to imploy an external agent in which he will take control of the cells in the simulation.
///
/// Holds current step size, grid, dimensions and initial configuration
pub struct CellGrid {
    pub step: u64,
    pub grid: DenseNumberGrid2D<CellType>,
    pub evac_grid: DenseNumberGrid2D<EvacueeCell>,
    pub dim: (u32, u32),
    pub initial_config: InitialConfig,
    /// Aspiration function used
    pub aspiration_st: Box<dyn AspirationStrategy + Send>,
    /// Ratio function used
    pub ratio_st: Box<dyn RatioStrategy + Send>,
    // Static measurement
    pub static_influence: Box<dyn StaticInfluence + Send>,
    /// Dynamic measurement
    pub dynamic_influence: Box<dyn DynamicInfluence + Send>,
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            step: 0,
            grid: DenseNumberGrid2D::new(DEFAULT_WIDTH as i32, DEFAULT_HEIGHT as i32),
            evac_grid: DenseNumberGrid2D::new(DEFAULT_WIDTH as i32, DEFAULT_HEIGHT as i32),
            dim: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            initial_config: Default::default(),
            static_influence: Box::new(ExitInfluence::new(
                1.5,
                &(DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32),
            )),
            aspiration_st: Box::new(LogAspManip::default()),
            ratio_st: Box::new(RootDist::default()),
            dynamic_influence: Box::new(ClosestDistance::new(DEFAULT_WIDTH as usize, 0.5)),
        }
    }
}

pub fn within_bounds(val: i32, limit: i32) -> bool {
    val >= 0 && val < limit
}

impl CellGrid {
    #[allow(unreachable_code)]
    /// Apply InitialConfiguration to the grid
    pub fn set_intial(&mut self) {
        todo!("Set evacuees to the grid");
        for (indx, val) in self.initial_config.initial_grid.iter().enumerate() {
            self.grid.set_value_location(
                *val,
                &Int2D {
                    x: indx as i32 % self.dim.1 as i32,
                    y: indx as i32 / self.dim.1 as i32,
                },
            )
        }
    }

    pub fn get_neigh(&self, x: i32, y: i32) -> (Vec<Loc>, Vec<EvacueeCell>) {
        let mut empty_vec = Vec::with_capacity(4);
        let mut evac_vec = Vec::with_capacity(4);
        for (i, j) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            if within_bounds(x + i, self.dim.0 as i32)
                && within_bounds(y + j, self.dim.1 as i32) // if we are not out of bounds
                && self.grid.get_value(&Int2D { x: x + i, y: y + j }).unwrap() == CellType::Empty
            // if the cell is empty
            // if there are no evacuees
            {
                if let Some(evac) = self.evac_grid.get_value(&Int2D { x: x + i, y: y + j }) {
                    evac_vec.push(evac)
                } else {
                    empty_vec.push((x + i, y + j))
                }
            }
        }
        (empty_vec, evac_vec)
    }

    fn get_distinations(
        &self,
        evacuee_agent: &EvacueeAgent,
        rng: &mut impl RngCore,
    ) -> (HashMap<(i32, i32), Vec<EvacueeCell>>, Vec<EvacueeCell>) {
        let updates = RefCell::new(HashMap::new());
        let still = RefCell::new(vec![]);
        let rng_cell = RefCell::new(rng);
        self.evac_grid.apply_to_all_values(
            // Extract intended movements of every agent, if agents want to move to the same square, add them to the queue
            |val| {
                let (empty_cells, _) = self.get_neigh(val.x, val.y);
                if empty_cells.len() == 0 {
                    // If there are no available cells, stay still
                    still.borrow_mut().push(*val);
                    return *val;
                }
                let weights = evacuee_agent.calculate_probabilities(
                    // else calculate the probability distribution of the neighbouring cells
                    &empty_cells,
                    self.static_influence.as_ref(),
                    self.dynamic_influence.as_ref(),
                );
                let dist = WeightedIndex::new(weights).unwrap();
                let opted_dist = empty_cells[dist.sample(&mut *rng_cell.borrow_mut())];
                updates // look for opted disk in the hashmap
                    .borrow_mut()
                    .entry(opted_dist)
                    .and_modify(|c: &mut Vec<EvacueeCell>| c.push(*val)) // if it exists, add the evacuee who wants to occupy the wanted square to the queue
                    .or_insert(vec![*val]); // else create a new vector with the evacuee in
                *val
            },
            krabmaga::engine::fields::grid_option::GridOption::READ,
        );
        (updates.take(), still.take())
    }

    fn play_game(
        &self,
        dist: (i32, i32),
        competing: Vec<EvacueeCell>,
        rng: &mut impl RngCore,
        evac_agent: &EvacueeAgent,
    ) -> Vec<EvacueeCell> {
        if competing.len() == 1 {
            // if there is only one competing agent, allow him to occupy the square
            return vec![EvacueeCell {
                x: dist.0,
                y: dist.1,
                ..competing[0]
            }];
            // return vec![(dist, competing[0])];
        }

        let game = competing[0].strategy.game_rules(
            // this section returns a shuffled list, the first is the user who will occupy the square where the rest will wait
            // else put the rules into play
            &competing[1..]
                .iter()
                .map(|e| e.strategy)
                .collect::<Vec<_>>(),
        );
        let n = competing.len();
        let mut competing: Vec<_> = competing
            .into_iter()
            .map(|e| {
                (
                    strategy_rewards(
                        n,
                        self.ratio_st.calculate_ratio(
                            self.dynamic_influence
                                .dynamic_influence(&Int2D { x: e.x, y: e.y }),
                        ),
                    ),
                    e,
                )
            })
            .collect();
        let asp = self
            .aspiration_st
            .calculate_asp(self.dynamic_influence.get_number_of_cells());
        let ids = match game {
            strategy::RuleCase::AllCoop => {
                // if everyone is cooperating randomly shuffle the list
                competing.shuffle(&mut *rng.borrow_mut());
                competing
                    .into_iter()
                    .map(|(rstp, e)| (s_x(rstp, asp, rstp.0), e))
                    .collect::<Vec<_>>()
            } // any will do
            strategy::RuleCase::AllButOneCoop => {
                // put the competitive guy first and the rest second
                competing.sort_by(|a, b| match (a.1.strategy, b.1.strategy) {
                    (Strategy::Competitive, _) => std::cmp::Ordering::Greater,
                    (_, Strategy::Competitive) => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Equal,
                });
                competing
                    .into_iter()
                    .map(|(w, el)| {
                        let ret_w = if el.strategy == Strategy::Competitive {
                            w.2
                        } else {
                            w.1
                        };
                        (s_x(w, asp, ret_w), el)
                    })
                    .collect()
            }
            strategy::RuleCase::Argument => competing
                .into_iter()
                .map(|(w, el)| (s_x(w, asp, w.3), el))
                .collect(),
        };
        [(dist, ids[0])]
            .into_iter()
            .chain(ids[1..].into_iter().map(|(d, c)| ((c.x, c.y), (*d, *c))))
            .map(|(c, (stim, evac))| {
                let pr_prime = evac_agent.calculate_strategies(&evac, stim);
                let mut strategy = evac.strategy;
                if rng.gen_bool(pr_prime as f64) {
                    strategy = strategy.inverse();
                }
                EvacueeCell {
                    x: c.0,
                    y: c.1,
                    strategy,
                    pr_c: pr_prime,
                }
            })
            .collect()
    }

    pub fn evacuee_step(&mut self, evacuee_agent: &EvacueeAgent, rng: &mut impl RngCore) {
        let (updates, still) = self.get_distinations(evacuee_agent, rng);
        let lp = updates //calculates which agent will occupy their intended square based on their game rules and preferences
            .into_iter()
            .flat_map(|(dist, competing)| self.play_game(dist, competing, rng, evacuee_agent))
            .chain(still.into_iter());
        for e in lp {
            self.evac_grid
                .set_value_location(e, &Int2D { x: e.x, y: e.y })
        }
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
                    let loc = Int2D { x, y };
                    updated.push((loc, CellType::Fire));
                    self.dynamic_influence.on_fire_update(&loc);
                } else {
                    updated.push((Int2D { x, y }, cell));
                }
            }
        }
        for (pos, cell) in updated.into_iter() {
            self.grid.set_value_location(cell, &pos)
        }
    }

    // TODO FIX FOR UNVISUALIZED VERSION
    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    /// Encapsulates the entire fire step
    /// # Arguments
    /// `fire_agent` - Agent that implements the Transition trait. Will be responsbilee for the fire spread
    ///
    pub fn fire_step(
        &mut self,
        fire_agent: &impl Transition<Cell = CellType>,
        rng: &mut impl RngCore,
    ) {
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
                initial_evac_grid: vec![],
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
                initial_evac_grid: vec![],
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

#[cfg(test)]
mod evac_tests {
    use super::*;

    #[test]
    fn test_empty_grid() { // test movement based on pure
    }
}
