use crate::model::{
    evacuee_mod::{
        strategies::{
            aspiration_strategy::{AspirationStrategy, LogAspManip},
            ratio_strategy::{RatioStrategy, RootDist, IDdist}, reward_strategy::{RewardStrategy, InverseLogRoot},
        },
        strategy::{strategy_rewards, RSTP},
    },
    // file_handling::file_handler::FileHandler,
    misc::misc_func::Loc,
};

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
    pub ratio: Box<dyn RatioStrategy + Send>,
    /// Reward game function used
    pub reward_game: Box<dyn RewardStrategy + Send>,
}


impl FireInfluence {
    pub fn reset(&mut self) {
        self.fire_area = 0;
        self.fire_state.reset();
    }
}

impl Default for FireInfluence {
    fn default() -> Self {
        Self {
            fire_area: 0,
            movement: Box::new(ClosestDistance::default()),
            aspiration: Box::new(LogAspManip::default()),
            ratio: Box::new(IDdist::default()),
            reward_game: Box::new(InverseLogRoot::default()),
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

    #[cfg(any(feature = "visualization", feature = "visualization_wasm"))]
    pub fn calculcate_rewards(&self, n: usize, point: &Loc, reward_b : f32) -> RSTP {
        let d = self.fire_state.closest_point(point).unwrap_or(0.5).sqrt();
        let r_t = self
            .ratio
            .calculate_ratio(self.fire_state.closest_point(point).unwrap_or(0.5)); // If there are no points we set it to its smallest possible value
                                                                                   // fh.curr_line.ratio.update(r_t);
        strategy_rewards(n, r_t, reward_b)
    }

    #[cfg(not(any(feature = "visualization", feature = "visualization_wasm")))]
    pub fn calculcate_rewards(&self, n: usize, point: &Loc, reward_b : f32) -> RSTP {
        use krabmaga::{plot, *};

        use crate::model::misc::misc_func::round;
        let d = self.fire_state.closest_point(point).unwrap_or(0.5).sqrt();
        let r_t = self.ratio.calculate_ratio(d); // If there are no points we set it to its smallest possible value
        plot!(
            "RatioDistance".to_owned(),
            "series".to_owned(),
            round(d as f64,3),
            round(r_t as f64,3),
            csv : true
        );
        strategy_rewards(n, r_t, reward_b)
    }

    pub fn calculate_aspiration(&self) -> f32 {
        self.aspiration.calculate_asp(self.fire_area)
    }

    pub fn on_step(&mut self, loc: &Loc) {
        self.fire_area += 1;
        self.fire_state.on_fire_update(loc);
    }
}

#[cfg(all(test,not(any(feature = "visualization", feature = "visualization_wasm"))))]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    use crate::model::evacuee_mod::{fire_influence::{
        fire_influence::FireInfluence, frontier::MockFrontierStructure,
    }, strategies::reward_strategy::MockRewardStrategy};
    use mockall::predicate;

    use crate::model::evacuee_mod::strategies::{
        aspiration_strategy::MockAspirationStrategy, ratio_strategy::MockRatioStrategy,
    };

    #[test]
    fn empty_fire_influence() {
        let mut frontier = MockFrontierStructure::new();

        frontier.expect_closest_point().return_const(None);

        let mut ratio_un = MockRatioStrategy::new();

        ratio_un
            .expect_calculate_ratio()
            .with(predicate::eq(0.5))
            .returning(|c| c);


        let fire = FireInfluence {
            fire_state: Box::new(frontier),
            ratio: Box::new(ratio_un),
            ..Default::default()
        };

        assert_relative_eq!(fire.calculate_aspiration(), 0.);
        assert_relative_eq!(fire.get_movement_influence(&Loc(1, 1)), 0.5);
        assert_eq!(
            fire.calculcate_rewards(3, &Loc(1, 1), 1.),
            (1. / 3. as f32, 0. as f32, (1. - 0.5 / 3.), -0.5 / 3. as f32)
        );
    }

    #[test]
    fn one_fire_cell_influence_mocked() {
        let mut front_st = MockFrontierStructure::new();

        front_st
            .expect_closest_point()
            .with(predicate::eq(Loc(1, 1)))
            .returning(|Loc(a, b): &Loc| Some((*a + *b) as f32));

        front_st.expect_on_fire_update().return_const(());

        let mut asp_st = MockAspirationStrategy::new();

        asp_st.expect_calculate_asp().returning(|c| 10. * c as f32);

        let mut ratio_st = MockRatioStrategy::new();

        ratio_st.expect_calculate_ratio().returning(|c| c.exp());


        let mut fire = FireInfluence {
            fire_state: Box::new(front_st),
            aspiration: Box::new(asp_st),
            ratio: Box::new(ratio_st),
            ..Default::default()
        };

        fire.on_step(&Loc(0, 0));

        assert_relative_eq!(fire.calculate_aspiration(), 10.);
        assert_relative_eq!(fire.get_movement_influence(&Loc(1, 1)), 1.);
        assert_eq!(
            fire.calculcate_rewards(3, &Loc(1, 1),1.),
            (
                1. / 3. as f32,
                0. as f32,
                (1. - 2.0f32.exp() / 3.),
                -2.0f32.exp() / 3. as f32
            )
        );
    }

    #[test]
    fn mock_trivial() {
        let mut rng = ChaCha8Rng::seed_from_u64(2);
        
        let mut fire = FireInfluence::default();
        let rn = rng.gen_range(5..=10);

        // Locations:
        // 9,6
        // 6,2
        // 8,3
        // 7,3
        // 9,2
        // 3,8

        for _ in 0..rn {
            let x = rng.gen_range(2..10);
            let y = rng.gen_range(2..10);
            fire.on_step(&Loc(x, y));
        }

        assert_relative_eq!(fire.calculate_aspiration(), 6.0f32.ln_1p() * 2.);
        assert_relative_eq!(fire.get_movement_influence(&Loc(3, 9)), 0.5);
        assert_relative_eq!(fire.get_movement_influence(&Loc(7, 10)), 10.);
        assert_eq!(
            fire.calculcate_rewards(2, &Loc(6, 6),1.),
            (1. / 2., 0., (1. - 3.0 / 2.), -3. / 2.)
        );
    }
}
