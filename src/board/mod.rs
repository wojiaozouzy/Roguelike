use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::MainState;
use crate::vectors::Vector2Int;
pub mod components;
mod dungeon;
mod systems;
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_systems(OnEnter(MainState::Game), systems::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}
