use std::collections::HashSet;

use itertools::Itertools;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::model::{
    death::{Announcer, DeathHandler},
    escape::{EscapeHandler, EvacTime, TimeEscape},
    evacuee_mod::{
        evacuee_cell::EvacueeCell,
        fire_influence::{
            dynamic_influence::{ClosestDistance, DynamicInfluence},
            fire_influence::FireInfluence,
            frontier::{Frontier, FrontierStructure},
        },
        static_influence::{ExitInfluence, StaticInfluence},
        strategies::{
            aspiration_strategy::{AspirationStrategy, LogAspManip},
            ratio_strategy::{RatioStrategy, RootDist},
        },
    },
    // file_handling::file_handler::FileHandler,
    misc::misc_func::Loc,
    state::{CellGrid, InitialConfig},
};

use super::{
    fire_input::*,
    import::{DeathInput, EscapeInput, ImportImproved, Setup, StaticInput},
};

pub trait ToSimulationStruct {
    type T;
    type P;

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T;
}

//====================== FIRE IMPLS ======================
impl ToSimulationStruct for FrontierInput {
    type T = Box<dyn FrontierStructure + Send>;
    type P = usize;
    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        match self {
            FrontierInput::VecTree => Box::new(Frontier::new(*params)),
        }
    }
}

impl ToSimulationStruct for MovementInput {
    type T = Box<dyn DynamicInfluence + Send>;

    type P = ();

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        Box::new(match self {
            MovementInput::ClosestDistance(e) => ClosestDistance(e.get_val(rng)),
        })
    }
}

impl ToSimulationStruct for AspirationInput {
    type T = Box<dyn AspirationStrategy + Send>;

    type P = ();

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        Box::new(match self {
            AspirationInput::LogAspiration(e) => LogAspManip(e.get_val(rng)),
        })
    }
}

impl ToSimulationStruct for RatioInput {
    type T = Box<dyn RatioStrategy + Send>;

    type P = ();

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        Box::new(match self {
            RatioInput::Root(e) => RootDist(e.get_val(rng)),
        })
    }
}

impl ToSimulationStruct for FireInput {
    type T = FireInfluence;

    type P = usize;

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        FireInfluence {
            fire_area: 0,
            fire_state: self
                .frontier
                .clone()
                .map(|e| e.to_struct(rng, params))
                .unwrap_or_else(|| Box::new(Frontier::new(*params))),
            aspiration: self.aspiration.to_struct(rng, &()),
            movement: self.movement.to_struct(rng, &()),
            ratio: self.ratio.to_struct(rng, &()),
        }
    }
}

//===================== Setup =====================

impl ToSimulationStruct for Setup {
    type T = InitialConfig;

    type P = (i32, i32); // w ,h

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        let mut map_rng: Box<dyn RngCore> = if let Some(seed) = self.map_seed {
            Box::new(ChaCha8Rng::seed_from_u64(seed))
        } else {
            Box::new(thread_rng())
        };
        let (initial_grid, evac_locs) = match &self.initial_locs {
            super::import::FixedOrRandom::Fixed((fire_locs, evac_locs)) => {
                (fire_locs.clone(), evac_locs.clone())
            }
            super::import::FixedOrRandom::Random => {
                let x = map_rng.gen_range(0..params.0);
                let y = map_rng.gen_range(0..params.1);
                let f_loc = (x, y);
                let map_rng = &mut map_rng;
                let mut h = HashSet::new();
                h.insert(f_loc);
                for _ in 0..self.evac_number {
                    loop {
                        let locs = (
                            map_rng.gen_range(0..params.0),
                            map_rng.gen_range(0..params.1),
                        );
                        if !h.contains(&locs) {
                            h.insert(locs);
                            break;
                        }
                    }
                }
                h.remove(&f_loc);
                let mut evacs = h.into_iter().collect_vec();
                evacs.sort();
                (vec![f_loc], evacs)
            }
        };
        let strats = match &self.initial_evac_strats {
            super::import::FixedOrRandom::Fixed(v) => {
                assert_eq!(evac_locs.len(), v.len());
                v.clone()
            }
            super::import::FixedOrRandom::Random => (0..evac_locs.len())
                .map(|_| {
                    let s = map_rng.gen();
                    s
                })
                .collect_vec(),
        };
        let probs = match &self.initial_evac_prob_dist {
            super::import::FixedOrRandom::Fixed(p) => {
                [*p].into_iter().cycle().take(evac_locs.len()).collect_vec()
            }
            super::import::FixedOrRandom::Random => {
                (0..evac_locs.len()).map(|_| map_rng.gen()).collect_vec()
            }
        };
        let initial_evac_grid = evac_locs
            .into_iter()
            .zip(strats.into_iter())
            .zip(probs.into_iter())
            .map(|(((x, y), strategy), pr_c)| EvacueeCell {
                x,
                y,
                strategy,
                pr_c,
            })
            .collect_vec();
        let fire_spread = match self.fire_spread {
            super::import::FixedOrRandom::Fixed(f) => f,
            super::import::FixedOrRandom::Random => rng.gen(),
        };
        InitialConfig {
            initial_grid,
            initial_evac_grid,
            fire_spread,
        }
    }
}

//===================== Other =====================

impl ToSimulationStruct for EscapeInput {
    type P = ();
    type T = Box<dyn EscapeHandler<EvacTime> + Send>;

    fn to_struct(&self, _rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        let p = match self {
            EscapeInput::TimeTracker => TimeEscape::default(),
        };
        Box::new(p)
    }
}

impl ToSimulationStruct for DeathInput {
    type T = Box<dyn DeathHandler + Send>;

    type P = ();

    fn to_struct(&self, _rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        let p = match self {
            DeathInput::AnnounceInput => Announcer::default(),
        };
        Box::new(p)
    }
}

impl ToSimulationStruct for StaticInput {
    type T = Box<dyn StaticInfluence + Send>;

    type P = Loc;

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        let p = match self {
            StaticInput::ClosestToExit(f) => ExitInfluence::new(f.get_val(rng), params),
        };
        Box::new(p)
    }
}

//===================== Main =====================

impl ToSimulationStruct for ImportImproved {
    type T = CellGrid;

    type P = String;

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        let (w, h) = self.dim;
        CellGrid {
            step: 0,
            iteration: 0,
            simulation_type: self.sim_type,
            grid: DenseNumberGrid2D::new(w as i32, h as i32),
            evac_grid: DenseNumberGrid2D::new(w as i32, h as i32),
            dim: self.dim,
            initial_config: self.setup.to_struct(rng, &(w as i32, h as i32)),
            fire_influence: self.fire.to_struct(rng, &(w as usize)),
            escape_handler: self.escape.to_struct(rng, &()),
            death_handler: self.death.to_struct(rng, &()),
            static_influence: self
                .static_input
                .to_struct(rng, &Loc(w as i32 / 2, h as i32)),
            // file_handler: FileHandler::new(params),
        }
    }
}
