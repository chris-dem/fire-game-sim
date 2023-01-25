use serde::Deserialize;

use crate::model::state::InitialConfig;

#[derive(Debug, Deserialize)]
pub struct Import {
    pub init: InitialConfig,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Deserialize)]
pub struct ImportRandom {}
