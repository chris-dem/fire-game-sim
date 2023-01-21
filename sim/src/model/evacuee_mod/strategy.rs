use rand_derive2::RandGen;
use serde::Deserialize;
use std::{cmp::Ordering, fmt};

#[derive(Debug, PartialEq, Eq)]
pub enum RuleCase {
    AllCoop,
    AllButOneCoop,
    Argument,
}

type RSTP = (f32, f32, f32, f32);

#[inline]
pub fn strategy_rewards(n: usize, r_t: f32) -> RSTP {
    (1. / n as f32, 0., (1. - r_t / n as f32), -r_t / n as f32)
}

#[inline]
pub fn s_x(rewards: RSTP, a_x: f32, reward: f32) -> f32 {
    let mx: f32 = [
        (rewards.0 - a_x).abs(),
        (rewards.1 - a_x).abs(),
        (rewards.2 - a_x).abs(),
        (rewards.3 - a_x).abs(),
    ]
    .into_iter()
    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    .unwrap();
    (reward - a_x) / mx
}

#[derive(Debug, PartialEq, Eq, RandGen, Clone, Copy, Deserialize)]
pub enum Strategy {
    Competitive,
    Cooperative,
}

impl Strategy {
    pub fn inverse(&self) -> Self {
        match self {
            Self::Competitive => Self::Cooperative,
            Self::Cooperative => Self::Competitive,
        }
    }

    pub fn game_rules(&self, neigh: &[Self]) -> RuleCase {
        let self_val = if *self == Self::Competitive { 1 } else { 0 };
        match neigh.iter().filter(|x| **x == Self::Competitive).count() + self_val {
            0 => RuleCase::AllCoop,
            1 => RuleCase::AllButOneCoop,
            _ => RuleCase::Argument,
        }
        // RuleCase::AllCoop
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_coop() {
        let curr_evac = Strategy::Cooperative;
        let others = [
            Strategy::Cooperative,
            Strategy::Cooperative,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::AllCoop);
    }

    #[test]
    fn test_all_but_one_coop() {
        let curr_evac = Strategy::Cooperative;
        let others = [
            Strategy::Cooperative,
            Strategy::Competitive,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::AllButOneCoop);
    }

    #[test]
    fn test_more_than_one_coop() {
        let curr_evac = Strategy::Competitive;
        let others = [
            Strategy::Cooperative,
            Strategy::Competitive,
            Strategy::Cooperative,
        ];
        assert_eq!(curr_evac.game_rules(&others), RuleCase::Argument);
    }
}
