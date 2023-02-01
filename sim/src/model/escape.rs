use krabmaga::engine::location::Int2D;

use super::{
    evacuee_mod::evacuee_cell::EvacueeCell,
    misc::misc_func::Loc,
    state::{DEFAULT_HEIGHT, DEFAULT_WIDTH},
};

pub trait EscapeHandler<T> {
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

impl EscapeHandler<EvacTime> for TimeEscape {
    fn escaped(&mut self, evac: EvacueeCell, step: usize) {
        dbg!("Evacuee  escaped");
        self.escaped_evac.push(EvacTime {
            loc: evac,
            time: step,
        })
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
