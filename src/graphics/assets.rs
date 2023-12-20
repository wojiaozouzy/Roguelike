use super::{GraphicsAssets, PIECE_Z};
use crate::assets::AssetList;
use bevy::prelude::*;
const ATLAS_PATH: &str = "textures/ascii.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<AssetList>,
) {
    let texture = asset_server.load(ATLAS_PATH);//加载一个图片
    // info!("asset loading ok");
    asset_list.0.push(texture.clone().untyped());//丢到缓存中去
    let atlas = TextureAtlas::from_grid(
        texture,
        Vec2::splat(PIECE_Z),
        16,
        16,
        None,
        None
    );
    let handle = texture_atlasses.add(atlas);
    commands.insert_resource(
        GraphicsAssets { sprite_texture: handle }
    );

}



