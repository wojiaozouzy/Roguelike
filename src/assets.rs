use bevy::prelude::*;

use bevy::asset::*;

use crate::states::MainState;
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>().add_systems(
            Update,
            check_asset_loading.run_if(in_state(MainState::LoadAssets)),
        );
    }
}
#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    let mut load_state = LoadState::Loaded;
    for ele in asset_list.0.iter() {
        match asset_server.load_state(ele.id()) {
            LoadState::Loaded => continue,
            state => {
                load_state = state;
                break;
            }
        }
    }
    match load_state {
        LoadState::Loaded => {
            next_state.set(MainState::Game);
        }
        LoadState::Failed => {
            error!("asset loading error");
        }
        _ => {}
    };
}

// /// Gets the overall load state of a group of assets from the provided handles.
// ///
// /// This method will only return [`LoadState::Loaded`] if all assets in the
// /// group were loaded successfully.
// fn get_group_load_state(
//     asset: &AssetServer,
//     handles: impl IntoIterator<Item = HandleId>,
// ) -> LoadState {
//     let mut load_state = LoadState::Loaded;
//     for handle_id in handles {
//         match handle_id {
//             HandleId::AssetPathId(id) => match self.get_load_state(id) {
//                 LoadState::Loaded => continue,
//                 LoadState::Loading => {
//                     load_state = LoadState::Loading;
//                 }
//                 LoadState::Failed => return LoadState::Failed,
//                 LoadState::NotLoaded => return LoadState::NotLoaded,
//                 LoadState::Unloaded => return LoadState::Unloaded,
//             },
//             HandleId::Id(_, _) => return LoadState::NotLoaded,
//         }
//     }

//     load_state
// }
