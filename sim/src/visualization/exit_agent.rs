use krabmaga::bevy::prelude::Quat;
use krabmaga::engine::state::State;
use krabmaga::visualization::agent_render::{AgentRender, SpriteType};
use krabmaga::visualization::fields::network::bevy_prototype_lyon::prelude::tess::geom::Translation;
use krabmaga::{engine::agent::Agent, bevy::prelude::Component};
use krabmaga::bevy::ecs as bevy_ecs;

use crate::model::state::CellGrid;

#[derive(Hash,Copy,Clone,PartialEq,Eq)]
pub struct ExitAgent(pub u32);


impl Agent for ExitAgent {
    fn step(&mut self, _state: &mut dyn krabmaga::engine::state::State) {
        
    }
}


#[derive(Component)]
pub struct ExitVis {
    pub id : u32
}

impl AgentRender for ExitVis {
    fn sprite(&self,_agent: &Box<dyn Agent>,_state: &Box<&dyn State>) -> SpriteType {
        SpriteType::Emoji(String::from("square"))
    }

    fn location(&self,_agent: &Box<dyn Agent>,state: &Box<&dyn State>) -> (f32,f32,f32) {
        let state = state.as_any().downcast_ref::<CellGrid>().unwrap();

        (state.dim.0 as f32  - 13., state.dim.1 as f32 + 14. , 1. )
    }

    fn scale(&self,_agent: &Box<dyn Agent>,_state: &Box<&dyn State>) -> (f32,f32) {
        (0.003,0.003)
    }

    fn rotation(&self,_agent: &Box<dyn Agent>,_state: &Box<&dyn State>) -> f32 {
        0.
    }

    fn update(&mut self,_agent: &Box<dyn Agent>,transform: &mut krabmaga::bevy::prelude::Transform,state: &Box<&dyn State>,visible: &mut krabmaga::bevy::prelude::Visibility,) {
        let translation = &mut transform.translation;
        let state = state.as_any().downcast_ref::<CellGrid>().unwrap();

        translation.x = state.dim.0 as f32  - 13.;
        translation.y = state.dim.1 as f32 + 14. ;
        translation.z = 1.;

        transform.scale.x = 0.003;
        transform.scale.y = 0.003;

        transform.rotation = Quat::from_rotation_z(0.);
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

