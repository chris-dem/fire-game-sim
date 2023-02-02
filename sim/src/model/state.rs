use crate::model::fire_mod::fire_cell::*;
use itertools::Itertools;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::state::State;
use krabmaga::engine::{fields::dense_number_grid_2d::DenseNumberGrid2D, location::Int2D};
use krabmaga::*;
use krabmaga::{Distribution, HashMap, Rng};
use rand::distributions::WeightedIndex;
use rand::RngCore;
use serde::Deserialize;
use std::collections::HashSet;

use super::death::{Announcer, DeathHandler};
use super::escape::{EscapeHandler, EvacTime, TimeEscape};
use super::evacuee_mod::evacuee::EvacueeAgent;
use super::evacuee_mod::evacuee_cell::EvacueeCell;
use super::evacuee_mod::fire_influence::fire_influence::FireInfluence;
use super::evacuee_mod::static_influence::{ExitInfluence, StaticInfluence};
use super::evacuee_mod::strategy::{rules, RuleCase};
// use super::file_handling::file_handler::FileHandler;
use super::misc::misc_func::Loc;
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
    pub initial_grid: Vec<(i32, i32)>,
    pub initial_evac_grid: Vec<EvacueeCell>,
    // TODO
    pub fire_spread: f32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum SimType {
    Flow,
    Total,
}

/// Grid in which the simulation will be running on
/// The way the simulation will work is to imploy an external agent in which he will take control of the cells in the simulation.
///
/// Holds current step size, grid, dimensions and initial configuration
pub struct CellGrid {
    pub simulation_type: SimType,
    pub iteration: u16,
    pub step: u64,
    pub grid: DenseNumberGrid2D<CellType>,
    pub evac_grid: DenseNumberGrid2D<EvacueeCell>,
    pub dim: (u32, u32),
    pub initial_config: InitialConfig,
    pub fire_influence: FireInfluence,
    pub escape_handler: Box<dyn EscapeHandler<EvacTime> + Send>,
    pub death_handler: Box<dyn DeathHandler + Send>,
    pub static_influence: Box<dyn StaticInfluence + Send>,
    // pub file_handler: FileHandler,
}

impl Default for CellGrid {
    fn default() -> Self {
        Self {
            step: 0,
            iteration: 0,
            simulation_type: SimType::Total,
            grid: DenseNumberGrid2D::new(DEFAULT_WIDTH as i32, DEFAULT_HEIGHT as i32),
            evac_grid: DenseNumberGrid2D::new(DEFAULT_WIDTH as i32, DEFAULT_HEIGHT as i32),
            dim: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
            initial_config: Default::default(),
            static_influence: Box::new(ExitInfluence::default()),
            death_handler: Box::new(Announcer::default()),
            escape_handler: Box::new(TimeEscape::default()),
            fire_influence: Default::default(),
            // file_handler: Default::default(),
        }
    }
}

pub fn within_bounds(val: i32, limit: i32) -> bool {
    val >= 0 && val < limit
}

impl CellGrid {
    /// Apply InitialConfiguration to the grid
    pub fn set_intial(&mut self) {
        let st: HashSet<(i32, i32)> =
            HashSet::from_iter(self.initial_config.initial_grid.iter().cloned());
        let to_grid = (0..self.dim.0 * self.dim.1).map(|indx| {
            let el = (
                indx as i32 % self.dim.1 as i32,
                indx as i32 / self.dim.1 as i32,
            );
            let c = if st.contains(&el) {
                CellType::Fire
            } else {
                CellType::Empty
            };
            (el, c)
        });
        for ((x, y), val) in to_grid {
            let loc = Int2D { x, y };
            self.grid.set_value_location(val, &loc);
            if val == CellType::Fire {
                self.fire_influence.on_step(&loc.into());
            }
        }

        for e in self.initial_config.initial_evac_grid.iter() {
            self.evac_grid
                .set_value_location(*e, &Int2D { x: e.x, y: e.y })
        }
    }

    pub fn get_neigh(&self, x: i32, y: i32) -> Vec<Loc> {
        let mut empty_vec = Vec::with_capacity(4);
        for (i, j) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let loc = Loc(x + i, y + j);
            if self.escape_handler.is_exit(&loc)
                || (within_bounds(x + i, self.dim.0 as i32)
                && within_bounds(y + j, self.dim.1 as i32) // if we are not out of bounds
                && self
                    .evac_grid
                    .get_value(&loc.into())
                    .is_none()
                && self.grid.get_value(&loc.into()).unwrap() == CellType::Empty)
            // if the cell is empty
            // if there are no evacuees
            {
                empty_vec.push(loc)
            }
        }
        empty_vec
    }

    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    fn get_distinations(
        &mut self,
        evacuee_agent: &EvacueeAgent,
        rng: &mut impl RngCore,
    ) -> (HashMap<Loc, Vec<EvacueeCell>>, Vec<EvacueeCell>) {
        let mut updates = HashMap::new();
        let mut still = vec![];
        // Extract intended movements of every agent, if agents want to move to the same square, add them to the queue
        for val in self.evac_grid.locs.values().iter().map(|c| *c) {
            let loc = Int2D { x: val.x, y: val.y };
            if *self.grid.locs.get_read(&loc).unwrap() == CellType::Fire {
                self.death_handler.update_death(loc);
                continue;
            }
            let empty_cells = self.get_neigh(val.x, val.y);
            if empty_cells.len() == 0 {
                // If there are no available cells, stay still
                still.push(*val);
                continue;
            }
            let weights = evacuee_agent.calculate_probabilities(
                // else calculate the probability distribution of the neighbouring cells
                &empty_cells,
                self.static_influence.as_ref(),
                &self.fire_influence,
            );
            let dist = WeightedIndex::new(weights).unwrap();
            let opted_dist = empty_cells[dist.sample(rng)];
            if self.escape_handler.is_exit(&opted_dist) {
                self.escape_handler.escaped(*val, self.step as usize);
            } else {
                updates // look for opted disk in the hashmap
                    .entry(opted_dist)
                    .and_modify(|c: &mut Vec<EvacueeCell>| c.push(*val)) // if it exists, add the evacuee who wants to occupy the wanted square to the queue
                    .or_insert(vec![*val]); // else create a new vector with the evacuee in
            }
        }
        (updates, still)
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    fn get_distinations(
        &mut self,
        evacuee_agent: &EvacueeAgent,
        rng: &mut impl RngCore,
    ) -> (HashMap<Loc, Vec<EvacueeCell>>, Vec<EvacueeCell>) {
        use std::cell::RefCell;

        let updates = RefCell::new(HashMap::new());
        let still = RefCell::new(vec![]);
        let dead = RefCell::new(vec![]);
        let escape = RefCell::new(vec![]);
        let rng = RefCell::new(rng);
        // Extract intended movements of every agent, if agents want to move to the same square, add them to the queue
        self.evac_grid.iter_values(|loc, val| {
            if self.grid.get_value(loc).unwrap() == CellType::Fire {
                // self.death_handler.update_death(*loc);
                dead.borrow_mut().push(*loc);
                return;
            }
            let empty_cells = self.get_neigh(val.x, val.y);
            if empty_cells.len() == 0 {
                // If there are no available cells, stay still
                still.borrow_mut().push(*val);
                return;
            }
            let weights = evacuee_agent.calculate_probabilities(
                // else calculate the probability distribution of the neighbouring cells
                &empty_cells,
                self.static_influence.as_ref(),
                &self.fire_influence,
            );
            let dist = WeightedIndex::new(weights).unwrap();
            let opted_dist = empty_cells[dist.sample(*rng.borrow_mut())];
            if self.escape_handler.is_exit(&opted_dist) {
                // self.escape_handler.escaped(*val, self.step as usize);
                escape.borrow_mut().push((*val, self.step as usize))
            } else {
                updates // look for opted disk in the hashmap
                    .borrow_mut()
                    .entry(opted_dist)
                    .and_modify(|c: &mut Vec<EvacueeCell>| c.push(*val)) // if it exists, add the evacuee who wants to occupy the wanted square to the queue
                    .or_insert(vec![*val]); // else create a new vector with the evacuee in
            }
        });
        // self.file_handler.curr_line.escaped += escape.borrow().len();
        // self.file_handler.curr_line.dead += dead.borrow().len();
        for loc in dead.take().into_iter() {
            self.death_handler.update_death(loc);
        }

        for (val, step) in escape.take().into_iter() {
            self.escape_handler.escaped(val, step);
        }
        (updates.take(), still.take())
    }

    fn play_game(
        &mut self,
        dist: Loc,
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

        // Could be optimised with no need to return new location
        let game = competing[0].strategy.game_rules(
            // this section returns a shuffled list, the first is the user who will occupy the square where the rest will wait
            // else put the rules into play
            &competing[1..]
                .iter()
                .map(|e| e.strategy)
                .collect::<Vec<_>>(),
        );
        // match &game {
        //     RuleCase::AllCoop => self.file_handler.curr_line.all_cnt += 1,
        //     RuleCase::AllButOneCoop => self.file_handler.curr_line.abo_cnt += 1,
        //     RuleCase::Argument => self.file_handler.curr_line.no_cnt += 1,
        // };
        let n = competing.len();
        let competing: Vec<_> = competing
            .into_iter()
            .map(|e| {
                (
                    self.fire_influence.calculcate_rewards(
                        n,
                        &Loc(e.x, e.y),
                        // &mut self.file_handler,
                    ),
                    e,
                )
            })
            .collect();

        let asp = self.fire_influence.calculate_aspiration();
        // self.file_handler.curr_line.asp = asp;
        let ids = rules(game, competing, rng, asp);
        let lis = if let Ok(ids) = ids {
            [(dist, ids[0])]
                .into_iter()
                .chain(ids[1..].into_iter().map(|(d, c)| (Loc(c.x, c.y), (*d, *c))))
                .collect_vec()
        } else {
            ids.unwrap_err()
                .into_iter()
                .map(|(d, c)| (Loc(c.x, c.y), (d, c)))
                .collect_vec()
        };
        lis.into_iter()
            .map(|(c, (stim, evac))| {
                // self.file_handler.curr_line.reward.update(stim);
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
            .chain(still.into_iter())
            .collect::<Vec<_>>();
        for e in lp.into_iter() {
            self.evac_grid
                .set_value_location(e, &Int2D { x: e.x, y: e.y })
        }
    }

    // #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    /// Encapsulates the entire fire step
    /// # Arguments
    /// `fire_agent` - Agent that implements the Transition trait. Will be responsbilee for the fire spread
    ///
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
                        n.push(self.grid.get_value(&Int2D { x: x + i, y: y + j }).unwrap())
                    }
                }
                if cell.spread(fire_agent, &n[..], rng) {
                    let loc = Int2D { x, y };
                    updated.push((loc, CellType::Fire));
                    self.fire_influence.on_step(&loc.into());
                } else {
                    updated.push((Int2D { x, y }, cell));
                }
            }
        }
        for (pos, cell) in updated.into_iter() {
            self.grid.set_value_location(cell, &pos);
        }
    }

    // #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    // pub fn fire_step(&mut self, fire_agent: &impl Transition, rng: &mut impl RngCore) {
    //     let updated: RefCell<Vec<(Int2D, CellType)>> = RefCell::new(Vec::new());
    //     let rng = RefCell::new(thread_rng());
    //     self.grid.iter_values(|&Int2D { x, y }, cell| {
    //         let mut n = Vec::with_capacity(8);
    //         for i in -1..=1 {
    //             for j in -1..=1 {
    //                 if (i == 0 && j == 0)
    //                     || !within_bounds(x + i, self.dim.0 as i32)
    //                     || !within_bounds(y + j, self.dim.1 as i32)
    //                 {
    //                     continue;
    //                 }
    //                 if let Some(c) = self.grid.get_value(&Int2D { x: x + i, y: y + j }) {
    //                     n.push(c);
    //                 }
    //             }
    //         }
    //         if cell.spread(fire_agent, &n[..], &mut *rng.borrow_mut()) {
    //             updated.borrow_mut().push((Int2D { x, y }, CellType::Fire));
    //         } else {
    //             updated.borrow_mut().push((Int2D { x, y }, cell));
    //         }
    //     });

    //     for (pos, cell) in updated.take().into_iter() {
    //         self.grid.set_value_location(cell, &pos)
    //     }
    // }
}

impl State for CellGrid {
    fn update(&mut self, step: u64) {
        self.step = step;
        self.grid.lazy_update();
        self.evac_grid.lazy_update();
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

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    fn after_step(&mut self, _schedule: &mut engine::schedule::Schedule) {
        // plot!(
        //     "Evacuees".to_owned(),
        //     "Steps".to_owned(),
        // todo!("Implement file handler")
        // if self.step % 10 == 0 {
        //     self.file_handler.add_line();
        // }
        // );
    }

    fn end_condition(&mut self, _schedule: &mut krabmaga::engine::schedule::Schedule) -> bool {
        self.fire_influence.fire_area == (self.dim.0 * self.dim.1) as usize
    }

    // Determine fire_out
    fn reset(&mut self) {
        self.step = 0;
        // self.file_handler.update_file(self.iteration);
        // self.file_handler.reset();
        self.fire_influence.fire_state.reset();
        self.grid = DenseNumberGrid2D::new(self.dim.0 as i32, self.dim.1 as i32);
        self.evac_grid = DenseNumberGrid2D::new(self.dim.0 as i32, self.dim.1 as i32);
    }

    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.reset();
        self.set_intial();
        let fire_rules = FireRules {
            spread: self.initial_config.fire_spread,
            id: 1,
        };
        let evac_agent = EvacueeAgent {
            id: 1,
            lc: 0.7,
            ld: 0.9,
        };

        self.grid.update();
        self.evac_grid.update();
        schedule.schedule_repeating(Box::new(fire_rules), 0., 0);
        schedule.schedule_repeating(Box::new(evac_agent), 0., 1);
        dbg!("===========NEW SIM==============");
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    fn init(&mut self, schedule: &mut krabmaga::engine::schedule::Schedule) {
        self.iteration += 1;
        self.reset();
        self.set_intial();
        let fire_rules = FireRules {
            spread: self.initial_config.fire_spread,
            id: 1,
        };
        let evac_agent = EvacueeAgent {
            id: 1,
            lc: 0.7,
            ld: 0.9,
        };

        // self.grid.update();
        // self.evac_grid.update();
        schedule.schedule_repeating(Box::new(fire_rules), 0., 0);
        schedule.schedule_repeating(Box::new(evac_agent), 0., 1);
    }
}

#[cfg(all(test, any(feature = "visualization", feature = "visualization_wasm")))]
mod tests {

    use super::*;
    // use crate::model::state_builder::CellGridBuilder;
    use crate::model::transition::MockTransition;
    use itertools::Itertools;
    use krabmaga::engine::fields::field::Field;
    use mockall::predicate;
    use rand::SeedableRng;
    use rand_chacha::ChaCha12Rng;

    mod fire_step {
        use crate::model::state_builder::CellGridBuilder;

        use super::*;
        #[test]
        fn test_fire_step_with_high_spread() {
            let mut grid = CellGridBuilder::default()
                .dim(5, 5)
                // .rng(Box::new(ChaCha12Rng::from_seed(Default::default())))
                .initial_config(InitialConfig {
                    fire_spread: 0.7,
                    initial_evac_grid: vec![],
                    initial_grid: vec![(0, 0)], //only the first
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
            // only first 6th and 7th
            let mut v = [CellType::Empty; 25];
            v[0] = CellType::Fire;
            v[1] = CellType::Fire;
            v[5] = CellType::Fire;
            v[6] = CellType::Fire;
            assert!(v.into_iter().zip_eq(v.into_iter()).all(|(c1, c2)| c1 == c2));
        }

        #[test]
        fn test_fire_step_with_small_spread() {
            let mut grid = CellGridBuilder::default()
                .dim(5, 5)
                .initial_config(InitialConfig {
                    fire_spread: 0.7,
                    initial_evac_grid: vec![],
                    initial_grid: vec![(0, 0)],
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
            assert!((0..25)
                .into_iter()
                .map(|c| {
                    if c == 0 {
                        CellType::Fire
                    } else {
                        CellType::Empty
                    }
                })
                .collect_vec()
                .into_iter()
                .zip_eq(v.into_iter())
                .all(|(c1, c2)| c1 == c2));
        }
    }

    mod evac_tests {
        use approx::Relative;
        use mockall::*;
        use rand::SeedableRng;
        use rand_chacha::ChaChaRng;

        use crate::model::{
            evacuee_mod::{
                evacuee::EvacueeAgent,
                evacuee_cell::EvacueeCell,
                fire_influence::{fire_influence::FireInfluence, frontier::MockFrontierStructure},
                strategies::aspiration_strategy::MockAspirationStrategy,
                strategy::Strategy,
            },
            misc::misc_func::Loc,
            state::CellGrid,
        };

        mod no_fire {
            use super::*;
            #[test]
            fn test_play_game_no_fire() {
                let mut rng = ChaChaRng::seed_from_u64(30);
                let competing = vec![
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        pr_c: 0.,
                        strategy: Strategy::Cooperative,
                    },
                    EvacueeCell {
                        x: 0,
                        y: 1,
                        pr_c: 0.,
                        strategy: Strategy::Cooperative,
                    },
                    EvacueeCell {
                        x: 1,
                        y: 0,
                        pr_c: 0.,
                        strategy: Strategy::Cooperative,
                    },
                ];
                let mut front_st = MockFrontierStructure::new();

                front_st
                    .expect_closest_point()
                    .with(predicate::function(|el| {
                        let cells = vec![Loc(0, 0), Loc(1, 0), Loc(0, 1)];
                        cells.contains(el)
                    }))
                    .return_const(None);

                let mut asp_st = MockAspirationStrategy::new();

                asp_st.expect_calculate_asp().returning(|c| 10. * c as f32);

                let fire = CellGrid {
                    fire_influence: FireInfluence {
                        fire_state: Box::new(front_st),
                        aspiration: Box::new(asp_st),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let new_state = fire.play_game(
                    Loc(1, 1),
                    competing.clone(),
                    &mut rng,
                    &EvacueeAgent {
                        id: 1,
                        lc: 0.7,
                        ld: 0.9,
                    },
                );
                let expected = vec![
                    EvacueeCell {
                        strategy: Strategy::Cooperative,
                        x: 1,
                        y: 1,
                        pr_c: 0.30529115,
                    },
                    EvacueeCell {
                        strategy: Strategy::Cooperative,
                        x: 0,
                        y: 0,
                        pr_c: 0.30529115,
                    },
                    EvacueeCell {
                        strategy: Strategy::Competitive,
                        x: 1,
                        y: 0,
                        pr_c: 0.30529115,
                    },
                ];
                assert!(new_state
                    .into_iter()
                    .zip(expected.into_iter())
                    .all(|(e1, e2)| {
                        let r = Relative::default().eq(&e1.pr_c, &e1.pr_c);
                        r && e1 == e2 && e1.strategy == e2.strategy
                    }))
            }

            #[test]
            fn test_play_game_no_fire_higher_adopting() {
                let mut rng = ChaChaRng::seed_from_u64(31);

                let competing = vec![
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        pr_c: 0.5,
                        strategy: Strategy::Cooperative,
                    },
                    EvacueeCell {
                        x: 0,
                        y: 1,
                        pr_c: 1.,
                        strategy: Strategy::Cooperative,
                    },
                    EvacueeCell {
                        x: 1,
                        y: 0,
                        pr_c: 0.8,
                        strategy: Strategy::Cooperative,
                    },
                ];
                let mut front_st = MockFrontierStructure::new();

                front_st
                    .expect_closest_point()
                    .with(predicate::function(|el| {
                        let cells = vec![Loc(0, 0), Loc(1, 0), Loc(0, 1)];
                        cells.contains(el)
                    }))
                    .return_const(None);

                let mut asp_st = MockAspirationStrategy::new();

                asp_st.expect_calculate_asp().return_const(0.);

                let fire = CellGrid {
                    fire_influence: FireInfluence {
                        fire_state: Box::new(front_st),
                        aspiration: Box::new(asp_st),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let new_state = fire.play_game(
                    Loc(1, 1),
                    competing.clone(),
                    &mut rng,
                    &EvacueeAgent {
                        id: 1,
                        lc: 0.7,
                        ld: 0.9,
                    },
                );
                let expected = vec![
                    EvacueeCell {
                        strategy: Strategy::Competitive,
                        x: 1,
                        y: 1,
                        pr_c: 0.6526456,
                    },
                    EvacueeCell {
                        strategy: Strategy::Competitive,
                        x: 1,
                        y: 0,
                        pr_c: 0.86105824,
                    },
                    EvacueeCell {
                        strategy: Strategy::Competitive,
                        x: 0,
                        y: 1,
                        pr_c: 1.0,
                    },
                ];
                assert!(new_state
                    .into_iter()
                    .zip(expected.into_iter())
                    .all(|(e1, e2)| {
                        let r = Relative::default().eq(&e1.pr_c, &e1.pr_c);
                        r && e1 == e2 && e1.strategy == e2.strategy
                    }))
            }
        }

        mod diretion_tests {}
    }
}
