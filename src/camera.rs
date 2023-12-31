/// ------------------------------------------------------------------------------------------------------------
/// 相机
/// ------------------------------------------------------------------------------------------------------------
use bevy::prelude::*;

use crate::graphics::TILE_SIZE;

pub fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    // temporary ;)
    camera.projection.scale = 8.;
    camera.transform.translation = Vec3::new(
        20. * TILE_SIZE,
        10. * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
