use approx::Relative;
use krabmaga::engine::location::Int2D;
use rand_distr::num_traits::Pow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Loc(pub i32, pub i32);

impl From<Int2D> for Loc {
    fn from(value: Int2D) -> Self {
        Self(value.x, value.y)
    }
}

impl From<Loc> for Int2D {
    fn from(value: Loc) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub fn round(s: f64, dec: u8) -> f64 {
    let b = 10.pow(dec) as f64;
    (s * b).round() / b
}

pub fn relative_eq_close(x: f32, y: f32) -> bool {
    Relative::default().epsilon(f32::EPSILON).eq(&x, &y)
}

/// Distance squared between two 2D locations
#[inline]
pub fn distsq(p1: &Int2D, p2: &Int2D) -> f32 {
    ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32
}

pub trait Reset {
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use krabmaga::{engine::location::Int2D, thread_rng, Rng};

    use crate::model::misc::misc_func::distsq;

    #[test]
    fn test_dist() {
        let mut rng = thread_rng();
        let one = Int2D { x: 0, y: 0 };
        let two = Int2D {
            x: rng.gen_range(0..1000),
            y: rng.gen_range(0..1000),
        };
        // assert_almost_eq!(
        assert_relative_eq!(
            distsq(&one, &two).sqrt(),
            ((two.x.pow(2) + two.y.pow(2)) as f32).sqrt()
        );
    }
}
