use rand::{seq::SliceRandom, RngCore};
use rand_derive2::RandGen;
use serde::Deserialize;
use std::{borrow::BorrowMut, cmp::Ordering, fmt};

use super::evacuee_cell::EvacueeCell;

#[derive(Debug, PartialEq, Eq)]
pub enum RuleCase {
    AllCoop,
    AllButOneCoop,
    Argument,
}

pub type RSTP = (f32, f32, f32, f32);

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

pub fn rules(
    game_rules: RuleCase,
    mut competing: Vec<(RSTP, EvacueeCell)>,
    rng: &mut impl RngCore,
    asp: f32,
) -> Result<Vec<(f32, EvacueeCell)>, Vec<(f32, EvacueeCell)>> {
    match game_rules {
        RuleCase::AllCoop => {
            // if everyone is cooperating randomly shuffle the list
            competing.shuffle(&mut *rng.borrow_mut());
            Ok(competing
                .into_iter()
                .map(|(rstp, e)| (s_x(rstp, asp, rstp.0), e))
                .collect::<Vec<_>>())
        } // any will do
        RuleCase::AllButOneCoop => {
            // put the competitive guy first and the rest second
            competing.sort_unstable_by(|a, b| match (a.1.strategy, b.1.strategy) {
                (Strategy::Competitive, _) => std::cmp::Ordering::Less,
                (_, Strategy::Competitive) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            });
            Ok(competing
                .into_iter()
                .map(|(w, el)| {
                    let ret_w = if el.strategy == Strategy::Competitive {
                        w.2
                    } else {
                        w.1
                    };
                    (s_x(w, asp, ret_w), el)
                })
                .collect())
        }
        RuleCase::Argument => Err(competing
            .into_iter()
            .map(|(w, el)| (s_x(w, asp, w.3), el))
            .collect()),
    }
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

    mod rule_tests {
        use rand::SeedableRng;
        use rand_chacha::ChaChaRng;

        use super::*;

        #[test]
        fn test_rules_all_coop() {
            let game_rules = RuleCase::AllCoop;
            let mut rng = ChaChaRng::seed_from_u64(3);
            let asp = 1.;
            let competing = vec![
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Cooperative,
                        pr_c: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Cooperative,
                        pr_c: 0.,
                    },
                ),
            ];
            let res = rules(game_rules, competing.clone(), &mut rng, asp).unwrap();
            let expected = vec![-0.33333334, -0.33333334];
            assert!(expected
                .into_iter()
                .zip(res.into_iter())
                .all(|(a1, a2)| (a1 as f32) == a2.0));
        }

        #[test]
        fn test_rules_all_but_one_coop() {
            let game_rules = RuleCase::AllButOneCoop;
            let mut rng = ChaChaRng::seed_from_u64(3);
            let asp = 1.;
            let competing = vec![
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Competitive,
                        pr_c: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Cooperative,
                        pr_c: 0.,
                    },
                ),
            ];
            let res = rules(game_rules, competing.clone(), &mut rng, asp).unwrap();
            let expected = vec![-1., -0.88888896];
            assert!(expected
                .into_iter()
                .zip(res.into_iter())
                .all(|(a1, a2)| (a1 as f32) == a2.0));
        }

        #[test]
        fn test_rules_no_coop() {
            let game_rules = RuleCase::Argument;
            let mut rng = ChaChaRng::seed_from_u64(3);
            let asp = 1.;
            let competing = vec![
                (
                    (0.7, 0.2, 0.1, 1.9),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Competitive,
                        pr_c: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 1.2),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Competitive,
                        pr_c: 0.,
                    },
                ),
            ];
            let res = dbg!(rules(game_rules, competing.clone(), &mut rng, asp).unwrap_err());
            let expected = vec![1., 0.22222228];
            assert!(expected
                .into_iter()
                .zip(res.into_iter())
                .all(|(a1, a2)| (a1 as f32) == a2.0));
        }
    }
}
