use krabmaga::engine::location::Int2D;

use super::{
    evacuee_mod::evacuee_cell::EvacueeCell,
    misc::misc_func::{Loc, Reset},
    state::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
};

pub trait EscapeHandler<T>: Reset {
    fn escaped(&mut self, evac: EvacueeCell, step: usize);
    fn get_escaped(&self) -> Vec<T>;
    fn get_escaped_number(&self) -> usize;
    fn is_exit(&self, loc: &Loc) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub struct EvacTime {
    pub loc: EvacueeCell,
    pub time: usize,
}

pub struct TimeEscape {
    pub escaped_evac: Vec<EvacTime>,
    pub exit: Int2D,
}

impl Default for TimeEscape {
    fn default() -> Self {
        Self {
            escaped_evac: Default::default(),
            exit: Int2D {
                x: DEFAULT_WIDTH as i32 / 2,
                y: DEFAULT_HEIGHT as i32,
            },
        }
    }
}

impl Reset for TimeEscape {
    fn reset(&mut self) {
        self.escaped_evac.clear();
    }
}

impl EscapeHandler<EvacTime> for TimeEscape {
    fn escaped(&mut self, evac: EvacueeCell, step: usize) {
        self.escaped_evac.push(EvacTime {
            loc: evac,
            time: step,
        });
    }

    fn get_escaped(&self) -> Vec<EvacTime> {
        self.escaped_evac.clone()
    }

    fn is_exit(&self, loc: &Loc) -> bool {
        let r: Loc = self.exit.into();
        // dbg!(r, *loc);
        r == *loc
    }

    fn get_escaped_number(&self) -> usize {
        self.escaped_evac.len()
    }
}

#[cfg(test)]
mod tests {
    use krabmaga::engine::location::Int2D;

    use rand::prelude::*;

    use crate::model::{evacuee_mod::evacuee_cell::EvacueeCell, misc::misc_func::Loc};

    use super::{EscapeHandler, TimeEscape};

    #[test]
    fn escape_handler_create() {
        let height = 51;
        let width = 51;
        let time = TimeEscape {
            exit: Int2D {
                x: width / 2,
                y: height,
            },
            ..Default::default()
        };
        assert!(time.escaped_evac.is_empty());
        assert_eq!(
            Into::<Loc>::into(time.exit),
            Into::<Loc>::into(Int2D { x: 25, y: 51 }),
        );
    }

    #[test]
    fn escaped_handler_non_escape() {
        let mut time_escape = TimeEscape::default();
        let mut rng = thread_rng();
        let step = 5;
        let cell = EvacueeCell {
            strategy: rng.gen(),
            x: 30,
            y: 30,
            pr_c: rng.gen(),
            pr_d: rng.gen(),
        };
        time_escape.escaped(cell.clone(), step);
        assert_eq!(time_escape.escaped_evac[0].loc, cell);
    }

    #[test]
    fn escaped_check_exit() {
        let time_escape = TimeEscape {
            exit: Int2D { x: 25, y: 51 },
            ..Default::default()
        };
        assert!(time_escape.is_exit(&Loc(25, 51)))
    }
}
