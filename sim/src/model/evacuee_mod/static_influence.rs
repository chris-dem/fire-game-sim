use crate::model::state::within_bounds;
use approx::Relative;
use krabmaga::engine::location::Int2D;

pub trait StaticInfluence {
    fn static_influence(&self, pos: &Int2D, end_pos: &Int2D, width: i32, height: i32) -> [f32; 4];
}

#[derive(Debug)]
pub struct ExitInfluence;

pub fn dist(p1: &Int2D, p2: &Int2D) -> f32 {
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32).sqrt()
}

impl StaticInfluence for ExitInfluence {
    fn static_influence(&self, pos: &Int2D, exit: &Int2D, width: i32, height: i32) -> [f32; 4] {
        if ((exit.x - pos.x), (exit.y - pos.y)) == (1, 0) {
            // The only case where we are going to face an issue
            return [1., 0., 0., 0.];
        }
        let vals = [(1, 0), (0, 1), (-1, 0), (0, -1)].map(|(i, j)| {
            let dis_cell = Int2D {
                x: pos.x + i,
                y: pos.y + j,
            };
            if !within_bounds(dis_cell.x, height) || !within_bounds(dis_cell.y, height) {
                0.
            } else {
                1. / dist(&dis_cell, exit)
            }
        });
        let sm: f32 = vals.iter().sum();
        vals.map(|el| el / sm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::{assert_relative_eq, Relative, RelativeEq};
    use itertools::Itertools;
    use krabmaga::engine::location::Int2D;

    #[test]
    fn static_influence_testing_on_a_random_setting() {
        let curr_pos = Int2D { x: 0, y: 0 };
        let dists = vec![1. / (5.0_f32).sqrt(), 1. / 3., 0., 0.];
        let sm: f32 = dists.iter().sum();
        let smax = dists.into_iter().map(|el| el / sm).collect::<Vec<f32>>();
        let w = 3;
        let h = 3;
        let infl = ExitInfluence;
        let [u, l, d, r] = infl.static_influence(&curr_pos, &Int2D { x: 3, y: 1 }, w, h);
        assert_relative_eq!(u, smax[0]);
        assert_relative_eq!(l, smax[1]);
        assert_relative_eq!(d, smax[2]);
        assert_relative_eq!(r, smax[3]);
    }

    #[test]
    fn static_influence_testing_on_a_random_setting2() {
        let curr_pos = Int2D { x: 0, y: 1 };
        let dists = vec![
            1. / (0.0f32 + 4.).sqrt(),
            1. / (9. + 1. as f32).sqrt(),
            0.,
            1. / (9. + 1. as f32).sqrt(),
        ];
        let sm: f32 = dists.iter().sum();
        let smax = dists.into_iter().map(|el| el / sm).collect::<Vec<f32>>();
        let w = 3;
        let h = 3;
        let infl = ExitInfluence;
        let [u, l, d, r] = infl.static_influence(&curr_pos, &Int2D { x: 3, y: 1 }, w, h);
        assert_relative_eq!(u, smax[0]);
        assert_relative_eq!(l, smax[1]);
        assert_relative_eq!(d, smax[2]);
        assert_relative_eq!(r, smax[3]);
    }

    #[test]
    fn static_influence_testing_on_a_extreme() {
        let curr_pos = Int2D { x: 2, y: 1 };
        let dists = vec![1., 0., 0., 0.];
        let sm: f32 = dists.iter().sum();
        let smax = dists.into_iter().map(|el| el / sm).collect::<Vec<f32>>();
        let w = 3;
        let h = 3;
        let infl = ExitInfluence;
        let [u, l, d, r] = infl.static_influence(&curr_pos, &Int2D { x: 3, y: 1 }, w, h);
        assert_relative_eq!(u, smax[0]);
        assert_relative_eq!(l, smax[1]);
        assert_relative_eq!(d, smax[2]);
        assert_relative_eq!(r, smax[3]);
    }

    #[test]
    fn static_influence_testing_on_a_extreme2() {
        let curr_pos = Int2D { x: 2, y: 2 };
        let dists = vec![0., 0., 1. / (1. + 4. as f32).sqrt(), 1.];
        let sm: f32 = dists.iter().sum();
        let smax = dists.into_iter().map(|el| el / sm).collect::<Vec<f32>>();
        let w = 3;
        let h = 3;
        let infl = ExitInfluence;
        let [u, l, d, r] = infl.static_influence(&curr_pos, &Int2D { x: 3, y: 1 }, w, h);
        assert_relative_eq!(u, smax[0]);
        assert_relative_eq!(l, smax[1]);
        assert_relative_eq!(d, smax[2]);
        assert_relative_eq!(r, smax[3]);
    }
}
