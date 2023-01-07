use crate::model::state::CellGrid;
use krabmaga::bevy::prelude::Commands;
// use krabmaga::engine::agent::Agent;
// use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
// use krabmaga::engine::fields::field::Field;
// use krabmaga::engine::schedule::Schedule;
// use krabmaga::engine::state::State;
use krabmaga::visualization::asset_handle_factory::AssetHandleFactoryResource;
use krabmaga::visualization::fields::number_grid_2d::BatchRender;
// use krabmaga::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use krabmaga::visualization::simulation_descriptor::SimulationDescriptor;
// use krabmaga::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct CellGridVis;

impl CellGridVis {}

impl CellGridVis {
    fn generate_field(
        state: &CellGrid,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        commands: &mut Commands,
        sim: &mut SimulationDescriptor,
    ) {
        state.grid.render(sprite_render_factory, commands, sim);
    }
}

// impl VisualizationState<CellGrid> for CellGridVis {
//     fn on_init(
//         &self
//         commands : &mut Commands,
//         _sprite_factory : &mut AssetHandleFactoryResource
//     )
// }
