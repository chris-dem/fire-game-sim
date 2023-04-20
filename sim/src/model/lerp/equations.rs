use lerp::Lerp;
use serde::Deserialize;
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Equation {
    Linear,
    Smooth,
    EaseIn,
    EaseOut,
}

impl Equation {
    #[inline]
    pub fn eval(&self, val: f32, min: f32, max: f32) -> f32 {
        let t = (val.clamp(min, max) - min) / (max - min);
        match self {
            Equation::Linear => t,
            Equation::Smooth => t * t * (3. - 2. * t),
            Equation::EaseIn => 1. - (1. - t) * (1. - t),
            Equation::EaseOut => t * t,
        }
    }

    #[inline]
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Linear),
            1 => Some(Self::Smooth),
            2 => Some(Self::EaseIn),
            3 => Some(Self::EaseOut),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LerpStruct {
    input_limits: (f32, f32),
    output_limits: (f32, f32),
    factor: f32,
    equation: Equation,
}

impl LerpStruct {
    pub fn new(x1: f32, x2: f32, y1: f32, y2: f32, factor: f32, equation: Equation) -> Self {
        Self {
            input_limits: (x1.min(x2), x1.max(x2)),
            output_limits: (y1, y2),
            factor,
            equation,
        }
    }

    pub fn eval(&self, input: f32) -> f32 {
        let t = self
            .equation
            .eval(input, self.input_limits.0, self.input_limits.1);
        self.output_limits.0.lerp(self.output_limits.1, t) * self.factor
    }
}

#[cfg(test)]
mod tests {
    use super::{Equation, LerpStruct};

    #[test]
    fn linear_test() {
        let inp = (0..10).map(|el| el as f32);
        let val = (0..10).map(|el| el as f32 / 9.);
        let lerp_struct = LerpStruct::new(0., 9., 0., 1., 1., super::Equation::Linear);
        for (x, y) in inp.zip(val) {
            assert_eq!(y, lerp_struct.eval(x))
        }
    }

    #[test]
    fn linear_flip_test() {
        let inp = (0..10).map(|el| el as f32);
        let val = (0..10).map(|el| 1. - el as f32 / 9.);
        let lerp_struct = LerpStruct::new(0., 9., 1., 0., 1., super::Equation::Linear);
        for (x, y) in inp.zip(val) {
            assert_eq!(y, lerp_struct.eval(x))
        }
    }

    #[test]
    fn smooth_test() {
        let inp = (0..10).map(|el| el as f32);
        let val = (0..10).map(|el| {
            let t = el as f32 / 9.;
            t * t * (3. - 2. * t)
        });
        let lerp_struct = LerpStruct::new(0., 9., 0., 1., 1., super::Equation::Smooth);
        for (x, y) in inp.zip(val) {
            assert_eq!(y, lerp_struct.eval(x))
        }
    }
    #[test]
    fn ease_in_test() {
        let inp = (0..10).map(|el| el as f32);
        let val = (0..10).map(|el| {
            let t = el as f32 / 9.;
            1. - (1. - t).powi(2)
        });
        let lerp_struct = LerpStruct::new(0., 9., 0., 1., 1., super::Equation::EaseIn);
        for (x, y) in inp.zip(val) {
            assert_eq!(y, lerp_struct.eval(x))
        }
    }
    #[test]
    fn ease_out_test() {
        let inp = (0..10).map(|el| el as f32);
        let val = (0..10).map(|el| {
            let t = el as f32 / 9.;
            t.powi(2)
        });
        let lerp_struct = LerpStruct::new(0., 9., 0., 1., 1., super::Equation::EaseOut);
        for (x, y) in inp.zip(val) {
            assert_eq!(y, lerp_struct.eval(x))
        }
    }
    mod prop_tests {
        use super::*;
        #[allow(unused_imports)]
        use crate::model::misc::misc_func::relative_eq_close;
        use lerp::Lerp;
        use proptest::prelude::*;
        #[allow(unused_imports)]
        use std::f32::{MAX, MIN};

        proptest! {
            #[test]
            fn test_linear(x in MIN..=MAX) {
                let val = x.clamp(0.,10.) / 10.;
                prop_assert!(relative_eq_close(val, Equation::EaseOut.eval(x, 0., 10.)))
            }

            #[test]
            fn test_smooth(x in MIN..=MAX) {
                let val = x.clamp(0.,10.) / 10.;
                let in_lerp  = Equation::EaseIn.eval(x, 0., 10.);
                let out_lerp = Equation::EaseOut.eval(x, 0., 10.);
                prop_assert!(relative_eq_close(val * val  * (3. - 2. * val), Equation::Smooth.eval(x, 0., 10.)));
                prop_assert!(relative_eq_close(in_lerp.lerp(out_lerp, val), Equation::Smooth.eval(x, 0., 10.)));
            }

            #[test]
            fn test_ease_in(x in MIN..=MAX) {
                let val = x.clamp(0.,10.) / 10.;
                prop_assert!(relative_eq_close(1. - (1. - val).powi(2),Equation::EaseIn.eval(x, 0., 10.)))
            }

            #[test]
            fn test_ease_out(x in MIN..=MAX) {
                let val = x.clamp(0.,10.) / 10.;
                prop_assert!(relative_eq_close(val.powi(2), Equation::EaseOut.eval(x, 0., 10.)))
            }

        }

        proptest! {}
    }
}
