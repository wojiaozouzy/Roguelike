use bevy::prelude::*;

use crate::{board::components::Position, states::TurnSet, vectors::Vector2Int};

mod assets;
mod components;
mod pieces;
mod tiles;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 60.;
pub const POSITION_TOLERANCE: f32 = 0.1;
#[derive(Event)]
pub struct GraphicsWaitEvent;
#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GraphicsWaitEvent>()
            .add_systems(Startup, assets::load_assets)
            .add_systems(
                Update,
                (
                    pieces::walk_animation,
                    pieces::melee_animation,
                    pieces::path_animator_update,
                )
                    .in_set(TurnSet::Animation),
            )
            .add_systems(
                Update,
                (pieces::spawn_piece_renderer, tiles::spawn_tile_renderer),
            );
    }
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        TILE_SIZE * position.v.x as f32,
        TILE_SIZE * position.v.y as f32,
        z,
    )
}

fn get_world_vec(v: Vector2Int, z: f32) -> Vec3 {
    Vec3::new(TILE_SIZE * v.x as f32, TILE_SIZE * v.y as f32, z)
}
