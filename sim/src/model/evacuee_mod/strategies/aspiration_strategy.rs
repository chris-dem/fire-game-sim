use mockall::predicate::*;
use mockall::*;

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
        self.0 * (numb_cells as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::model::evacuee_mod::strategies::aspiration_strategy::AspirationStrategy;

    use super::{LogAspManip, RootAsp};

    #[test]
    fn check_default_strategies() {
        let default_root = RootAsp::default();
        let default_log = LogAspManip::default();
        let set_root = RootAsp(0.5);
        let set_log = LogAspManip(0.5);
        assert_relative_eq!(default_root.0, 1.);
        assert_relative_eq!(default_log.0, 1.);
        assert_relative_eq!(set_root.0, 0.5);
        assert_relative_eq!(set_log.0, 0.5);
    }

    #[test]
    fn check_calculations() {
        let number_of_cells = 50;
        let default_root = RootAsp::default();
        let default_log = LogAspManip::default();

        assert_relative_eq!(
            default_root.calculate_asp(number_of_cells),
            (number_of_cells as f32).sqrt() / 3.
        );

        assert_relative_eq!(
            default_log.calculate_asp(number_of_cells),
            (number_of_cells as f32).ln_1p() / 3.
        )
    }
}
