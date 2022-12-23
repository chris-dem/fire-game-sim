use std::cell::RefCell;
use std::cmp::Eq;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};

use krabmaga::engine::{
    // fields::dense_number_grid_2d::DenseNumberGrid2D,
    fields::dense_number_grid_2d::DenseNumberGrid2D,
    location::Int2D,
    schedule::Schedule,
    state::State,
};

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Fire,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    state: CellType,
    id: usize,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.id == other.id
    }
}
impl Eq for Cell {}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} status {}", self.id, self.state)
    }
}

pub InitialConfig

pub struct CellGrid {
    pub step: u64,
    pub grid: DenseNumberGrid2D<Cell>,
    pub dim : (f32, f32),
    pub initial_config : ()
}

impl CellGrid {
    pub fn new(dim : (f32,f32),)
}



