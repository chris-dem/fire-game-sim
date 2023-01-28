use std::fmt::Debug;

use krabmaga::engine::location::Int2D;
use mockall::automock;

use crate::model::misc::misc_func::distsq;

use crate::model::misc::misc_func::Loc;
use crate::model::state::{DEFAULT_HEIGHT, DEFAULT_WIDTH};

#[automock]
/// Calculate the static influence from the exit of a current cell
pub trait StaticInfluence {
    fn static_influence(&self, pos: &Int2D) -> f32;
    fn get_static_effect(&self) -> f32;
}

/// Structure that implements the Static Influence trait
/// Method used:
/// $$
///  { 1 \over |x|+1}
/// $$
#[derive(Debug, Clone)]
pub struct ExitInfluence {
    s_effect: f32,
    end_pos: Loc,
}
impl ExitInfluence {
    pub fn new(s_effect: f32, end_pos: &Loc) -> Self {
        Self {
            s_effect,
            end_pos: *end_pos,
        }
    }
}

impl Default for ExitInfluence {
    fn default() -> Self {
        Self {
            s_effect: 0.5,
            end_pos: Loc(DEFAULT_WIDTH as i32 / 2, DEFAULT_HEIGHT as i32 + 1),
        }
    }
}

impl StaticInfluence for ExitInfluence {
    fn static_influence(&self, pos: &Int2D) -> f32 {
        distsq(&self.end_pos.into(), pos).sqrt()
    }

    fn get_static_effect(&self) -> f32 {
        self.s_effect
    }
}

#[derive(Debug, Clone)]
pub struct ConstantInfluence;

impl StaticInfluence for ConstantInfluence {
    fn static_influence(&self, _pos: &Int2D) -> f32 {
        1.
    }

    fn get_static_effect(&self) -> f32 {
        1.
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_relative_eq;
    use krabmaga::engine::location::Int2D;

    #[test]
    fn static_influence_testing_on_a_random_setting() {
        let smax = vec![1. / (5. as f32 + 1.), 1. / (9. as f32 + 1.)];
        let infl = ExitInfluence::new(1., &Loc(3, 1));
        assert_relative_eq!(infl.static_influence(&Int2D { x: 1, y: 0 },), smax[0]); // up
        assert_relative_eq!(infl.static_influence(&Int2D { x: 0, y: 1 }), smax[1]);
        // right
    }

    #[test]
    fn static_influence_testing_on_a_random_setting2() {
        let end_pos = Int2D { x: 3, y: 1 };
        let smax = vec![
            Int2D { x: 2, y: 1 },
            Int2D { x: 0, y: 1 },
            Int2D { x: 3, y: 1 },
            Int2D { x: 1, y: 2 },
        ]
        .into_iter()
        .map(|el| inverse_plus_one(distsq(&el, &end_pos)))
        .collect::<Vec<_>>();

        let infl = ExitInfluence::new(1., &end_pos.into());
        assert_relative_eq!(infl.static_influence(&Int2D { x: 2, y: 1 }), smax[0]);
        assert_relative_eq!(infl.static_influence(&Int2D { x: 0, y: 1 }), smax[1]);
        assert_relative_eq!(infl.static_influence(&Int2D { x: 3, y: 1 }), smax[2]);
        assert_relative_eq!(infl.static_influence(&Int2D { x: 1, y: 2 }), smax[3]);
    }
}
