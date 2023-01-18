use std::collections::BTreeSet;

use krabmaga::engine::location::Int2D;

use crate::model::misc::misc_func::distsq;

pub type Loc = (i32, i32);

#[derive(Debug, Clone)]
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

    #[inline]
    pub fn update(&mut self, (x, y): &Loc) {
        assert!(0 <= *x && (*x as usize) < self.trees.len());
        let x = *x as usize;
        self.trees[x].insert(*y);
    }

    pub fn closest_point(&self, loc: &Loc) -> f32 {
        let calc = |lc: &(i32, i32)| {
            // d^2, Reason of why not + 1. is because, if distance is 0 we do not want to go to that cell
            // but it is very unlikely
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
                let next = tree.range(..loc.1).next_back().map(capt);
                let prev = tree.range(loc.1..).next().map(capt);
                [next, prev]
            })
            .fold(None, |acc, el| {
                acc.zip(el)
                    .map(|(el1, el2)| if el1 < el2 { el1 } else { el2 })
                    .or(acc.or(el))
            })
            .unwrap_or(1.) // Essentially only time this is None is when there doesn't exist a fire_cell. If so by default there is no influence hence set it to 1
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
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn frontier_empty() {
        let front = Frontier::new(5);
        assert_relative_eq!(front.closest_point(&(0, 0)), 1.)
    }

    #[test]
    fn frontier_one_point() {
        let mut front = Frontier::new(2);
        front.update(&(0, 0));
        assert_relative_eq!(front.closest_point(&(0, 0)), 0.);
        assert_relative_eq!(front.closest_point(&(0, 0)), 0.);
        assert_relative_eq!(front.closest_point(&(1, 0)), 1.);
    }

    #[test]
    fn frontier_straight_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_relative_eq!(front.closest_point(&(3, 0)), 0.);
        assert_relative_eq!(front.closest_point(&(3, 1)), 1.);
        assert_relative_eq!(front.closest_point(&(4, 1)), (2.0_f32));
    }

    #[test]
    fn frontier_column_line() {
        let mut front = Frontier::new(5);
        front.update_vec(&vec![(0, 0), (0, 1), (0, 2), (0, 3)]);
        assert_relative_eq!(front.closest_point(&(0, 3)), 0.);
        assert_relative_eq!(front.closest_point(&(1, 3)), 1.);
        assert_relative_eq!(front.closest_point(&(1, 4)), (2.0_f32));
    }
}
