use mockall::predicate::*;
use mockall::*;
use rand_derive2::RandGen;
use std::fmt;
use std::hash::{Hash, Hasher};
/// Cell Type of the simulation. This means the type of the current cell.
/// For now, treating the fire model and the agent model in the same.
///
/// TODO Determine how addition of different fields might affect the situation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, RandGen)]
pub enum CellType {
    Fire,
    Empty,
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Cell struct for the simulation. Holds state of the cell and id.
/// Since I am planning to use a dense number grid, Cell has to derive:
/// Debug Clone Copy Hash PartialEq Eq Display
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub state: CellType,
    pub id: usize,
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
