use crate::model::cell::Cell;
use crate::model::state::CellGrid;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;

impl Component for Cell {
    type Storage = TableStorage;
}

impl BatchRender<CellGrid> for DenseNumberGrid2D<Cell> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_value(loc) {
            None | Some(Cell)
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (800, 600)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &CellGrid) -> krabmaga::bevy::prelude::Image {
        state.grid.texture()
    }
}
