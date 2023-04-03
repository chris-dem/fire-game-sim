use crate::model::evacuee_mod::evacuee_cell::EvacueeCell;
use crate::model::evacuee_mod::strategy::Strategy;
use crate::model::fire_mod::fire_cell::CellType;
use crate::model::state::CellGrid;
use krabmaga::bevy::ecs::component::TableStorage;
use krabmaga::bevy::prelude::Component;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;

impl Component for CellType {
    type Storage = TableStorage;
}

impl Component for EvacueeCell {
    type Storage = TableStorage;
}

impl BatchRender<CellGrid> for DenseNumberGrid2D<CellType> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        match self.get_value(loc) {
            None => [0u8; 4],
            Some(CellType::Empty) => [191, 191, 191, 10],
            Some(CellType::Fire) => [210, 48, 8, 255],
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &CellGrid) -> krabmaga::bevy::prelude::Image {
        state.grid.texture()
    }
}

impl BatchRender<CellGrid> for DenseNumberGrid2D<EvacueeCell> {
    fn get_pixel(&self, loc: &Int2D) -> [u8; 4] {
        if let Some(c) = self.get_value(loc) {
            match c.strategy {
                Strategy::Competitive => [153, 255, 51, 255],
                Strategy::Cooperative => [51, 255, 255, 255],
            }
        } else {
            [0u8; 4]
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn get_layer(&self) -> f32 {
        0.
    }

    fn get_texture_from_state(state: &CellGrid) -> krabmaga::bevy::prelude::Image {
        state.evac_grid.texture()
    }
}
