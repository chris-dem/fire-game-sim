use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use rand::prelude::*;

use crate::model::{
    death::{Announcer, DeathHandler},
    escape::{EscapeHandler, EvacTime, TimeEscape},
    evacuee_mod::{
        fire_influence::{
            dynamic_influence::{ClosestDistance, DynamicInfluence},
            fire_influence::FireInfluence,
            frontier::{Frontier, FrontierStructure},
        },
        static_influence::{ExitInfluence, StaticInfluence},
        strategies::{
            aspiration_strategy::{AspirationStrategy, LogAspManip, RootAsp},
            ratio_strategy::{IDdist, LogDist, RatioStrategy, RootDist},
            reward_strategy::{InverseLogRoot, RewardStrategy, RootReward},
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
    fn to_struct(&self, _rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
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
            MovementInput::ClosestDistance(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                ClosestDistance(e)
            }
        })
    }
}

impl ToSimulationStruct for AspirationInput {
    type T = Box<dyn AspirationStrategy + Send>;

    type P = ();

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        match self {
            AspirationInput::LogAspiration(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(LogAspManip(e))
            }
            AspirationInput::RootAspiration(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(RootAsp(e))
            }
        }
    }
}

impl ToSimulationStruct for RatioInput {
    type T = Box<dyn RatioStrategy + Send>;

    type P = ();

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        match self {
            RatioInput::Root(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(RootDist(e))
            }
            RatioInput::Log(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(LogDist(e))
            }
            RatioInput::Id(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(IDdist(e))
            }
        }
    }
}

impl ToSimulationStruct for RewardGameInput {
    type T = Box<dyn RewardStrategy + Send>;

    type P = (f32, f32);

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        match self {
            Self::InvLogRoot(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                Box::new(InverseLogRoot(e, 10.0f32.ln_1p()))
            }
            Self::RewardRoot(e) => {
                let e = e.unwrap_or_else(|| rng.gen());
                let max_dist = (params.0.powi(2) + params.1.powi(2)).sqrt();
                Box::new(RootReward(e, max_dist))
            }
        }
    }
}

impl ToSimulationStruct for FireInput {
    type T = FireInfluence;

    type P = (usize, usize);

    fn to_struct(&self, rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        FireInfluence {
            fire_area: 0,
            fire_state: self
                .frontier
                .clone()
                .map(|e| e.to_struct(rng, &params.0))
                .unwrap_or_else(|| Box::new(Frontier::new(params.0))),
            aspiration: self.aspiration.to_struct(rng, &()),
            movement: self.movement.to_struct(rng, &()),
            ratio: self.ratio.to_struct(rng, &()),
            reward_game: self
                .reward_game
                .to_struct(rng, &(params.0 as f32, params.1 as f32)),
        }
    }
}

//===================== Setup =====================

impl ToSimulationStruct for Setup {
    type T = InitialConfig;

    type P = (i32, i32); // w ,h

    fn to_struct(&self, _rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        InitialConfig {
            initial_grid: self.initial_fire,
            initial_evac_grid: self.initial_evac.clone(),
            fire_spread: self.fire_spread,
            map_seed: self.map_seed,
            evac_num: self
                .evac_number
                .unwrap_or((params.0 as f32 * params.1 as f32 * 0.085) as usize),
            lc: self.lc,
            ld: self.ld,
        }
    }
}

//===================== Other =====================

impl ToSimulationStruct for EscapeInput {
    type T = Box<dyn EscapeHandler<EvacTime> + Send>;
    type P = Loc;

    fn to_struct(&self, _rng: &mut dyn RngCore, params: &Self::P) -> Self::T {
        let p = match self {
            EscapeInput::TimeTracker => TimeEscape {
                exit: (*params).into(),
                ..Default::default()
            },
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
            StaticInput::ClosestToExit(f) => {
                ExitInfluence::new(f.unwrap_or_else(|| rng.gen()), params)
            }
        };
        Box::new(p)
    }
}

//===================== Main =====================

impl ToSimulationStruct for ImportImproved {
    type T = CellGrid;

    type P = String;

    fn to_struct(&self, rng: &mut dyn RngCore, _params: &Self::P) -> Self::T {
        let (w, h) = self.dim;
        CellGrid {
            step: 0,
            iteration: 0,
            simulation_type: self.sim_type,
            grid: DenseNumberGrid2D::new(w as i32, h as i32),
            evac_grid: DenseNumberGrid2D::new(w as i32, h as i32),
            dim: self.dim,
            param_seed: self.param_seed,
            initial_config: self.setup.to_struct(rng, &(w as i32, h as i32)),
            fire_influence: self.fire.to_struct(rng, &(w as usize, h as usize)),
            escape_handler: self.escape.to_struct(rng, &Loc(w as i32 / 2, h as i32)),
            death_handler: self.death.to_struct(rng, &()),
            static_influence: self
                .static_input
                .to_struct(rng, &Loc(w as i32 / 2, h as i32)),
            ..Default::default()
        }
    }
}
