use krabmaga::log;
use rand::{seq::SliceRandom, RngCore};
use rand_derive2::RandGen;
use serde::Deserialize;
use std::{borrow::BorrowMut, cmp::Ordering, fmt};

use super::evacuee_cell::EvacueeCell;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RuleCase {
    AllCoop,
    AllButOneCoop,
    Argument,
}

pub type RSTP = (f32, f32, f32, f32);

#[inline]
pub fn strategy_rewards(n: usize, r_t: f32, b: f32) -> RSTP {
    (
        b / n as f32,
        0.,
        b * (1. - r_t / n as f32),
        -b * r_t / n as f32,
    )
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
    // dbg!(reward, a_x);
    (reward - a_x) / mx
}

pub fn rules(
    game_rules: RuleCase,
    mut competing: Vec<(RSTP, EvacueeCell)>,
    rng: &mut impl RngCore,
    asp: f32,
    // ) -> Result<Vec<(f32, EvacueeCell)>, Vec<(f32, EvacueeCell)>> {
) -> Result<
    Box<dyn Iterator<Item = (f32, EvacueeCell)>>,
    Box<dyn Iterator<Item = (f32, EvacueeCell)>>,
> {
    match game_rules {
        RuleCase::AllCoop => {
            // if everyone is cooperating randomly shuffle the list
            competing.shuffle(&mut *rng.borrow_mut());
            Ok(Box::new(
                competing
                    .into_iter()
                    .map(move |(rstp, e)| (s_x(rstp, asp, rstp.0), e)), // .collect::<Vec<_>>())
                                                                        // .into_iter(),
            ))
        } // any will do
        RuleCase::AllButOneCoop => {
            // put the competitive guy first and the rest second
            competing.sort_unstable_by(|a, b| match (a.1.strategy, b.1.strategy) {
                (Strategy::Competitive, _) => std::cmp::Ordering::Less,
                (_, Strategy::Competitive) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            });
            Ok(Box::new(
                competing.into_iter().map(move |(w, el)| {
                    let ret_w = if el.strategy == Strategy::Competitive {
                        w.2
                    } else {
                        w.1
                    };
                    (s_x(w, asp, ret_w), el)
                }), // .into_iter(),
            ))
        }
        RuleCase::Argument => Err(Box::new(
            competing.into_iter().map(move |(w, el)| {
                let retw = w.3;
                (s_x(w, asp, retw), el)
            }), // .into_iter(),
        )),
        // .collect()),
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
    use crate::model::{evacuee_mod::strategy::Strategy, misc::misc_func::relative_eq_close};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sx(s in (-10f32..10f32,-10f32..10f32,-10f32..10f32,-10f32..10f32), a in (0.5 ..10f32), idx in 0..4usize) {
            let arr : [f32 ; 4] = [s.0,s.1, s.2,s.3];
            let reward = arr[idx];
            let mx = arr.iter().map(|el| (el - a).abs()).max_by(|a,b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)).unwrap();
            let el = s_x(s, a, reward);
            prop_assert!(relative_eq_close(el, (reward - a) / mx))
        }

    }

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
        use itertools::Itertools;
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
                        pr_d: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Cooperative,
                        pr_c: 0.,
                        pr_d: 0.,
                    },
                ),
            ];
            let res = rules(game_rules, competing.clone(), &mut rng, asp)
                .ok()
                .unwrap()
                .collect_vec();
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
                        pr_d: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 0.1),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Cooperative,
                        pr_c: 0.,
                        pr_d: 0.,
                    },
                ),
            ];
            let res = rules(game_rules, competing.clone(), &mut rng, asp)
                .ok()
                .unwrap()
                .collect_vec();
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
                        pr_d: 0.,
                    },
                ),
                (
                    (0.7, 0.2, 0.1, 1.2),
                    EvacueeCell {
                        x: 0,
                        y: 0,
                        strategy: Strategy::Competitive,
                        pr_c: 0.,
                        pr_d: 0.,
                    },
                ),
            ];
            let res = dbg!(rules(game_rules, competing.clone(), &mut rng, asp)
                .err()
                .unwrap()
                .collect_vec());
            let expected = vec![1., 0.22222228];
            assert!(expected
                .into_iter()
                .zip(res.into_iter())
                .all(|(a1, a2)| (a1 as f32) == a2.0));
        }
    }
}
