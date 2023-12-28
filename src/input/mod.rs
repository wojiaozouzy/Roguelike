// input/mod.rs
use bevy::prelude::*;

use crate::board::components::Position;

use crate::player::{Deck, DeckEvent, DeckEventKind, Player};
use crate::states::GameState;
use crate::vectors::Vector2Int;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_input.run_if(in_state(GameState::PlayerInput)),
        );
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 4] = [
    (KeyCode::W, Vector2Int::UP),
    (KeyCode::S, Vector2Int::DOWN),
    (KeyCode::A, Vector2Int::LEFT),
    (KeyCode::D, Vector2Int::RIGHT),
];

fn player_input(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<&Position, With<Player>>,
    // deck: Res<Deck>,
    mut ev_deck: EventWriter<DeckEvent>,
) {
    let Ok(position) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        info!("键盘输入player_input");
        ev_deck.send(DeckEvent(DeckEventKind::UseCard(Some(position.v + dir))));
    }
}
