// ui/mod.rs

use bevy::prelude::*;
use std::collections::HashMap;

use crate::states::GameState;

mod assets;
mod deck;
mod helpers;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ReloadUiEvent>()
            .add_systems(Startup, assets::load_assets)
            .add_systems(
                Update,
                (
                    helpers::button_click_animation,
                    deck::draw_deck.run_if(on_event::<ReloadUiEvent>()),
                    deck::card_click.run_if(in_state(GameState::PlayerInput)),
                ),
            )
            .add_systems(OnEnter(GameState::PlayerInput), player_input_start);
    }
}

#[derive(Event)]
pub struct ReloadUiEvent;

fn player_input_start(mut ev_ui: EventWriter<ReloadUiEvent>) {
    ev_ui.send(ReloadUiEvent);
}

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub textures: HashMap<&'static str, Handle<Image>>,
}
