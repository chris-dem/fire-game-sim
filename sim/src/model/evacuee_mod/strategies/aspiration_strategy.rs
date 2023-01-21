use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait AspirationStrategy {
    fn calculate_asp(&self, numb_cells: usize) -> f32;
}

pub struct LogAspManip(f32);

impl Default for LogAspManip {
    fn default() -> Self {
        Self(1.)
    }
}

impl AspirationStrategy for LogAspManip {
    fn calculate_asp(&self, numb_cells: usize) -> f32 {
        self.0 * (numb_cells as f32).ln()
    }
}
