use mockall::predicate::*;
use mockall::*;

// TODO  ASPIRATION IS SQRT
// TODO  REWARD IS MAXSQRT - DIST


#[automock]
pub trait AspirationStrategy {
    fn calculate_asp(&self, numb_cells: usize) -> f32;
}

pub struct LogAspManip(pub f32);

impl Default for LogAspManip {
    fn default() -> Self {
        Self(1.)
    }
}

impl AspirationStrategy for LogAspManip {
    fn calculate_asp(&self, numb_cells: usize) -> f32 {
        // Use ln + 1 to avoid inf cases
        self.0 * (numb_cells as f32).ln_1p()
    }
}


pub struct RootAsp(pub f32);

impl Default for RootAsp {
    fn default() -> Self {
        Self(1.)
    }
}

impl AspirationStrategy for RootAsp {
    fn calculate_asp(&self, numb_cells: usize) -> f32 {
        self.0 * (numb_cells as f32).sqrt() / 3.
    }
}

