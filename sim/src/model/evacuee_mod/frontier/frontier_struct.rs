use std::collections::BTreeSet;

use approx::Relative;
use krabmaga::engine::location::Int2D;

use crate::model::evacuee_mod::static_influence::dist;

type Loc = (i32, i32);

#[derive(Debug)]
pub struct Frontier {
    trees: Vec<BTreeSet<i32>>,
}

impl Frontier {
    pub fn new(len: usize) -> Self {
        let mut trees = Vec::with_capacity(len);
        for _ in 0..len {
            trees.push(BTreeSet::new());
        }
        Self { trees }
    }

    pub fn update(&mut self, (x, y): &Loc) {
        assert!(0 <= *x && (*x as usize) < self.trees.len());
        let x = *x as usize;
        self.trees[x].insert(*y);
    }

    pub fn closest_point(&self, loc: &Loc) -> Option<(f32, Loc)> {
        let calc = |lc: &(i32, i32)| {
            let d = dist(&Int2D { x: loc.0, y: loc.1 }, &Int2D { x: lc.0, y: lc.1 });
            (d, *lc)
        };
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(indx, tree)| {
                let capt = |y: &i32| {
                    let x = indx as i32;
                    calc(&(x, *y))
                };
                let next = tree.range(..loc.1).next_back().map(capt);
                let prev = tree.range(loc.1..).next().map(capt);
                [next, prev]
            })
            .fold(None, |acc, el| {
                acc.zip(el)
                    .map(|(el1, el2)| if el1.0 < el2.0 { el1 } else { el2 })
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

#[cfg(test)]
mod frontier_tests {
    use super::*;

    #[test]
    fn frontier_empty() {
        let front = Frontier::new(5);
        assert!(front.closest_point(&(0, 0)).is_none())
    }

    #[test]
    fn frontier_one_point() {
        let mut front = Frontier::new(2);
        front.update(&(0, 0));
        assert_eq!(front.closest_point(&(0, 0)), Some((f32::MAX, (0, 0))));
        assert_eq!(front.closest_point(&(0, 0)), Some((f32::MAX, (0, 0))));
        assert_eq!(front.closest_point(&(1, 0)), Some((1., (0, 0))));
    }

    #[test]
    fn frontier_straight_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(front.closest_point(&(3, 0)), Some((0., (3, 0))));
        assert_eq!(front.closest_point(&(3, 1)), Some((1., (3, 0))));
        assert_eq!(
            front.closest_point(&(4, 1)),
            Some(((2.0_f32).sqrt(), (3, 0)))
        );
    }

    #[test]
    fn frontier_column_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        assert_eq!(front.closest_point(&(0, 3)), Some((0., (0, 3))));
        assert_eq!(front.closest_point(&(1, 3)), Some((1., (0, 3))));
        assert_eq!(
            front.closest_point(&(1, 4)),
            Some(((2.0_f32).sqrt(), (0, 3)))
        );
    }
}
