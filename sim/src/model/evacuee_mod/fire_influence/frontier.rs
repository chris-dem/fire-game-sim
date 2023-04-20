use std::collections::BTreeSet;

use krabmaga::engine::location::Int2D;

use crate::model::misc::misc_func::{distsq, Loc};
use crate::model::state::DEFAULT_WIDTH;
use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait FrontierStructure {
    fn on_fire_update(&mut self, loc: &Loc);

    fn closest_point(&self, loc: &Loc) -> Option<f32>;

    fn reset(&mut self);
}

#[derive(Debug, Clone)]
pub struct Frontier {
    trees: Vec<BTreeSet<i32>>,
}

impl Default for Frontier {
    fn default() -> Self {
        Self::new(DEFAULT_WIDTH as usize)
    }
}

impl Frontier {
    pub fn new(len: usize) -> Self {
        let mut trees = Vec::with_capacity(len);
        for _ in 0..len {
            trees.push(BTreeSet::new());
        }
        Self { trees }
    }

    fn update(&mut self, Loc(x, y): &Loc) -> Option<bool> {
        let x = *x as usize;
        self.trees.get_mut(x).map(|tree| tree.insert(*y))
    }

    fn nearest(&self, loc: &Loc) -> Option<f32> {
        let calc = |lc: &(i32, i32)| {
            // d^2, Reason of why not + 1. is because, a having a distance of 0 will be filtered from the neighbours
            distsq(&Int2D { x: loc.0, y: loc.1 }, &Int2D { x: lc.0, y: lc.1 })
        };

        self // Select the point with the smallest distance
            .trees
            .iter()
            .enumerate()
            .flat_map(|(indx, tree)| {
                let capt = |y: &i32| {
                    let x = indx as i32;
                    calc(&(x, *y))
                };
                // dbg!(tree);
                let next = tree.range(..loc.1).next_back().map(capt);
                let prev = tree.range(loc.1..).next().map(capt);
                [next, prev]
            })
            .fold(None, |acc, el| {
                acc.zip(el)
                    .map(|(el1, el2)| if el1 < el2 { el1 } else { el2 })
                    .or(acc.or(el))
            })
    }

    #[cfg(test)]
    fn update_vec(&mut self, v: &Vec<Loc>) {
        for el in v {
            self.update(el);
        }
    }
}

impl FrontierStructure for Frontier {
    fn on_fire_update(&mut self, loc: &Loc) {
        self.update(loc);
    }

    fn closest_point(&self, loc: &Loc) -> Option<f32> {
        self.nearest(loc)
    }

    fn reset(&mut self) {
        self.trees.iter_mut().for_each(|tree| {
            tree.clear();
        });
    }
}
#[cfg(test)]
mod frontier_tests {
    use approx::assert_relative_eq;
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn test_not_tree(x in (-50i32..0).prop_union(50..100), y in any::<i32>()) {
            let mut frontier = Frontier::new(50);
            prop_assert_eq!(frontier.update(&Loc(x,y)),None);
        }
        #[test]
        fn test_in_tree(x in (0..50), y in any::<i32>()) {
            let mut frontier = Frontier::new(50);
            prop_assert!(frontier.update(&Loc(x,y)).is_some());
        }

        #[test]
        fn find_closest_dist(a in prop::array::uniform32((0..50, -50i32..50)), new_locs in prop::array::uniform10((0..50, -50i32..50))) {
            let mut frontier = Frontier::new(50);
            for (x,y) in a.iter() {
                frontier.update(&Loc(*x,*y));
            }

            let from_frontier = new_locs.iter().map(|(x,y)| frontier.closest_point(&Loc(*x,*y)).unwrap());
            let from_two_d = new_locs.iter().map(|(x,y)| {
                let x = *x;
                let y = *y;
                a.iter().map(|(xa,ya)| (xa - x).pow(2) as f32 + (ya - y).pow(2) as f32).min_by(|a,b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)).unwrap()
            });

            let arr = from_frontier.zip(from_two_d).all(|(a,b)| a == b);

            prop_assert!(arr);
        }
    }

    #[test]
    fn frontier_empty() {
        let front = Frontier::new(5);
        assert_eq!(front.closest_point(&Loc(0, 0)), None)
    }

    #[test]
    fn frontier_one_point() {
        let mut front = Frontier::new(2);
        front.update(&Loc(0, 0));
        assert_relative_eq!(front.closest_point(&Loc(0, 0)).unwrap(), 0.);
        assert_relative_eq!(front.closest_point(&Loc(0, 0)).unwrap(), 0.);
        assert_relative_eq!(front.closest_point(&Loc(1, 0)).unwrap(), 1.);
    }

    #[test]
    fn frontier_straight_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![Loc(0, 0), Loc(1, 0), Loc(2, 0), Loc(3, 0)]);
        assert_relative_eq!(front.closest_point(&Loc(3, 0)).unwrap(), 0.);
        assert_relative_eq!(front.closest_point(&Loc(3, 1)).unwrap(), 1.);
        assert_relative_eq!(front.closest_point(&Loc(4, 1)).unwrap(), (2.0_f32));
    }

    #[test]
    fn frontier_column_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![Loc(0, 0), Loc(0, 1), Loc(0, 2), Loc(0, 3)]);
        assert_relative_eq!(front.closest_point(&Loc(0, 3)).unwrap(), 0.);
        assert_relative_eq!(front.closest_point(&Loc(1, 3)).unwrap(), 1.);
        assert_relative_eq!(front.closest_point(&Loc(1, 4)).unwrap(), (2.0_f32));
    }
}
