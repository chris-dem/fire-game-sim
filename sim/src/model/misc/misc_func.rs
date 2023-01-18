use krabmaga::engine::location::Int2D;

use crate::model::evacuee_mod::frontier::frontier_struct::Loc;

/// Implementation of $f(x) = 1 / (x + 1)$
#[inline]
pub fn inverse_plus_one(val: f32) -> f32 {
    (val + 1.).recip()
}

/// Distance squared between two 2D locations
#[inline]
pub fn distsq(p1: &Int2D, p2: &Int2D) -> f32 {
    ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2)) as f32
}

/// Convert location type to Int2D
pub fn loc_to_int2d((x, y): &Loc) -> Int2D {
    Int2D { x: *x, y: *y }
}

/// Convert Int2D to location type
pub fn int2d_to_loc(Int2D { x, y }: &Int2D) -> Loc {
    (*x, *y)
}
