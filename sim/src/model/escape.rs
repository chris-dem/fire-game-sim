use krabmaga::engine::location::Int2D;

use super::{
    evacuee_mod::evacuee_cell::EvacueeCell,
    misc::misc_func::{Loc, Reset},
    state::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
};

pub trait EscapeHandler<T>: Reset {
    fn escaped(&mut self, evac: EvacueeCell, step: usize);

    fn get_escaped(&self) -> Vec<T>;

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

}
