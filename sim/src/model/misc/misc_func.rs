use krabmaga::engine::location::Int2D;

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

// /// Implementation of $f(x) = 1 / (x + 1)$
// #[inline]
// pub fn inverse_plus_one(val: f32) -> f32 {
//     (val + 1.).recip()
// }

/// Distance squared between two 2D locations
#[inline]
pub fn distsq(p1: &Int2D, p2: &Int2D) -> f32 {
    ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32
}

pub trait Reset {
    fn reset(&mut self);
}
