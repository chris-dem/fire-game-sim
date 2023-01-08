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
use krabmaga::visualization::visualization_state::VisualizationState;
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
        state
            .grid
            .render(&mut *sprite_render_factory, commands, sim);
    }
}

impl VisualizationState<CellGrid> for CellGridVis {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_render_factory: &mut AssetHandleFactoryResource,
        state: &mut CellGrid,
        _schedule: &mut krabmaga::engine::schedule::Schedule,
        sim: &mut SimulationDescriptor,
    ) {
        Self::generate_field(&state, sprite_render_factory, commands, sim);
    }

    fn get_agent_render(
        &self,
        _agent: &Box<dyn krabmaga::engine::agent::Agent>,
        _state: &CellGrid,
    ) -> Option<Box<dyn krabmaga::visualization::agent_render::AgentRender>> {
        None
    }

    fn get_agent(
        &self,
        _agent_render: &Box<dyn krabmaga::visualization::agent_render::AgentRender>,
        _state: &Box<&dyn krabmaga::engine::state::State>,
    ) -> Option<Box<dyn krabmaga::engine::agent::Agent>> {
        None
    }
}
