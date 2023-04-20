// use crate::model::state::*;
// use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
// // TODO finish
// use super::{
//     escape::TimeEscape,
//     evacuee_mod::{
//         fire_influence::fire_influence::FireInfluence,
//         static_influence::{ExitInfluence, StaticInfluence},
//     },
//     misc::misc_func::Loc,
// };

// /// CellGrid Builder struct. Uses the builder consumer pattern in order to construct a CellGrid.
// #[derive(Default)]
// pub struct CellGridBuilder {
//     step: u64,
//     dim: Option<(u32, u32)>,
//     initial_config: Option<InitialConfig>,
//     fire_influence: Option<FireInfluence>,
//     static_influence: Option<Box<dyn StaticInfluence + Send>>,
// }

// impl CellGridBuilder {
//     /// Consume current self and return updated CellGrid with new dimensions
//     pub fn dim(mut self, w: i32, h: i32) -> Self {
//         self.dim = Some((w as u32, h as u32));
//         self
//     }

//     pub fn initial_config(mut self, initial_config: InitialConfig) -> Self {
//         self.initial_config = Some(initial_config);
//         self
//     }

//     pub fn fire_influence(mut self, fire_infl: FireInfluence) -> Self {
//         self.fire_influence = Some(fire_infl);
//         self
//     }

//     pub fn static_influence(mut self, static_influence: Box<dyn StaticInfluence + Send>) -> Self {
//         self.static_influence = Some(static_influence);
//         self
//     }

//     // TODO could add input checking methods
//     pub fn build(self) -> CellGrid {
//         let dim = self.dim.unwrap_or((DEFAULT_WIDTH, DEFAULT_HEIGHT));
//         CellGrid {
//             step: self.step,
//             dim,
//             grid: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
//             evac_grid: DenseNumberGrid2D::new(dim.0 as i32, dim.1 as i32),
//             initial_config: self.initial_config.unwrap_or_default(),
//             static_influence: self.static_influence.unwrap_or(Box::new(ExitInfluence::new(
//                 1.5,
//                 &Loc(DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32),
//             ))),
//             fire_influence: self.fire_influence.unwrap_or_default(),
//             escape_handler: Box::new(TimeEscape {
//                 exit: Loc(dim.0 as i32 / 2, dim.1 as i32).into(),
//                 ..Default::default()
//             }),
//             ..Default::default()
//         }
//     }
// }
