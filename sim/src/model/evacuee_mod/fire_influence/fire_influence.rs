use crate::model::{
    evacuee_mod::{
        strategies::aspiration_strategy::{AspirationStrategy, LogAspManip},
        strategy::{strategy_rewards, RSTP},
    },
    lerp::equations::LerpStruct,
    // file_handling::file_handler::FileHandler,
    misc::misc_func::Loc,
    state::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
};

pub const MAX_REWARD: f32 = 20.;

use super::{
    dynamic_influence::{ClosestDistance, DynamicInfluence},
    frontier::{Frontier, FrontierStructure},
};

/// Fire Influence structure
///
pub struct FireInfluence {
    pub fire_state: Box<dyn FrontierStructure + Send>,
    pub fire_area: usize,
    /// Dynamic measurement
    pub movement: Box<dyn DynamicInfluence + Send>,
    /// Aspiration function used
    pub aspiration: Box<dyn AspirationStrategy + Send>,
    /// Ratio function used
    pub ratio: LerpStruct,
    /// Reward game function used
    pub reward_game: LerpStruct,
}

impl FireInfluence {
    pub fn reset(&mut self) {
        self.fire_area = 0;
        self.fire_state.reset();
    }
}

impl Default for FireInfluence {
    fn default() -> Self {
        let mx_dist = ((DEFAULT_HEIGHT * DEFAULT_HEIGHT) as f32
            + (DEFAULT_WIDTH * DEFAULT_WIDTH) as f32)
            .sqrt();
        Self {
            fire_area: 0,
            movement: Box::new(ClosestDistance::default()),
            aspiration: Box::new(LogAspManip::default()),
            ratio: LerpStruct::new(
                0.,
                mx_dist,
                0.,
                MAX_REWARD,
                1.,
                crate::model::lerp::equations::Equation::Linear,
            ),
            reward_game: LerpStruct::new(
                0.,
                mx_dist,
                MAX_REWARD,
                0.,
                1.,
                crate::model::lerp::equations::Equation::Linear,
            ),
            fire_state: Box::new(Frontier::default()),
        }
    }
}

impl FireInfluence {
    pub fn get_movement_influence(&self, loc: &Loc) -> f32 {
        self.movement
            .dynamic_influence(&(*loc).into(), self.fire_state.as_ref())
            * self.movement.get_dynamic_effect()
    }

    // #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    // pub fn calculcate_rewards(&self, n: usize, point: &Loc, reward_b: f32) -> RSTP {
    //     let d = self.fire_state.closest_point(point).unwrap_or(0.5).sqrt();
    //     let r_t = self
    //         .ratio
    //         .calculate_ratio(self.fire_state.closest_point(point).unwrap_or(0.5)); // If there are no points we set it to its smallest possible value
    //                                                                                // fh.curr_line.ratio.update(r_t);
    //     strategy_rewards(n, r_t, reward_b)
    // }

    pub fn calculcate_rewards(&self, n: usize, point: &Loc, reward_b: f32) -> RSTP {
        let d = self.fire_state.closest_point(point).unwrap_or(0.5).sqrt();
        let r_t = self.calculate_ratio(d); // If there are no points we set it to its smallest possible value
        #[cfg(not(any(feature = "bayesian", feature = "ga_search")))]
        {
            use krabmaga::{plot, *};

            use crate::model::misc::misc_func::round;
            plot!(
                "RatioDistance".to_owned(),
                "series".to_owned(),
                round(d as f64,3),
                round(r_t as f64,3),
                csv : true
            );
        }
        strategy_rewards(n, r_t, reward_b)
    }

    #[inline]
    pub fn calculate_aspiration(&self) -> f32 {
        self.aspiration.calculate_asp(self.fire_area)
    }

    #[inline]
    pub fn calculate_ratio(&self, dist: f32) -> f32 {
        self.ratio.eval(dist)
    }

    #[inline]
    pub fn calculate_reward(&self, dist: f32) -> f32 {
        self.reward_game.eval(dist)
    }

    pub fn on_step(&mut self, loc: &Loc) {
        self.fire_area += 1;
        self.fire_state.on_fire_update(loc);
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        evacuee_mod::fire_influence::frontier::MockFrontierStructure,
        misc::misc_func::relative_eq_close,
    };

    use super::*;
    use crate::model::lerp::equations::*;
    use lerp::num_traits::Pow;
    use mockall::{
        predicate::{self, eq},
        Sequence,
    };
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_movement_influence(x in 0i32..100, y in 0i32..100) {
            let mut frontier = MockFrontierStructure::new();

            frontier.expect_closest_point()
                    .returning(|Loc(x,y) : &Loc| Some((x + y) as f32))
                    .once();

            let fire_infl = FireInfluence {
                fire_state : Box::new(frontier),
                ..Default::default()
            };
            let dist = fire_infl.get_movement_influence(&Loc(x,y));
            prop_assert!(relative_eq_close(dist, ((x + y) as f32).sqrt()))
        }

        #[test]
        fn test_rewards(x in 0i32..100, y in 0i32..10, b in 0.1..50.0f32, n in 1..=4usize) {
            let mut frontier = MockFrontierStructure::new();

            frontier
                .expect_closest_point()
                .returning(|Loc(x, y): &Loc| Some((x + y) as f32))
                .once();

            let ratio = LerpStruct::new(0., 100., 0., MAX_REWARD, 1., Equation::EaseIn);
            let fire_infl = FireInfluence {
                ratio: ratio.clone(),
                fire_state: Box::new(frontier),
                ..Default::default()
            };
            let r = ratio.eval((x as f32 + y as f32).sqrt());
            let vals = (b / n as f32,  0., b * (1. - r / n as f32), -(b * r) / n as f32);
            let vals_fire = fire_infl.calculcate_rewards(n, &Loc(x, y), b);
            prop_assert!(relative_eq_close(vals.0, vals_fire.0));
            prop_assert!(relative_eq_close(vals.1, vals_fire.1));
            prop_assert!(relative_eq_close(vals.2, vals_fire.2));
            prop_assert!(relative_eq_close(vals.3, vals_fire.3));
        }
        #[test]
        fn fire_update(x in 1..100i32, y in 1..100i32) {
            let mut frontier = MockFrontierStructure::new();

            let mut seq = Sequence::new();
            frontier
                .expect_closest_point()
                .return_const( None)
                .times(1)
                .in_sequence(&mut seq);
            frontier
                .expect_on_fire_update()
                .times(1)
                .with(predicate::eq(Loc(x, y)))
                .in_sequence(&mut seq);
            frontier
                .expect_closest_point()
                .times(1)
                .returning(move |Loc(a, b): &Loc| Some((*a - x).pow(2) as f32 + (b - y).pow(2) as f32))
                .in_sequence(&mut seq);
            let mut fire_infl = FireInfluence {
                fire_state: Box::new(frontier),
                ..Default::default()
            };

            assert_eq!(fire_infl.fire_state.closest_point(&Loc(x, y)), None);
            fire_infl.on_step(&Loc(x, y));
            assert_eq!(fire_infl.fire_area, 1);
            assert_eq!(fire_infl.fire_state.closest_point(&Loc(x, y)), Some(0.));
        }
    }
}
