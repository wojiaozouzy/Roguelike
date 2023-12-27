// manager/mod.rs
use bevy::prelude::*;

use crate::actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent};
use crate::graphics::GraphicsWaitEvent;

use crate::player::PlayerActionEvent;
use crate::states::{GameState, MainState, TurnSet};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), game_start)
            .add_systems(OnExit(MainState::Game), game_end)
            .add_systems(
                Update,
                (
                    turn_update_start.run_if(on_event::<PlayerActionEvent>()),
                    turn_update_end.run_if(on_event::<ActionsCompleteEvent>()),
                    turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()),
                    tick.in_set(TurnSet::Tick),
                ),
                
            ).configure_sets(Update,  (TurnSet::Logic, TurnSet::Animation, TurnSet::Tick)
            .chain().run_if(in_state(GameState::TurnUpdate)));
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    info!("触发game_start进入GameState::PlayerInput状态 ");
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    info!("触发game_end进入GameState::None状态 ");
    next_state.set(GameState::None);
}
fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    info!("触发turn_update_start进入TurnUpdate状态 ;进入触发TickEvent");
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}
fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    ev_wait1: EventReader<InvalidPlayerActionEvent>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    info!("tick检查");
    let a = ev_wait.read();
    let a1 = ev_wait1.len();
    if a.len() == 0 && a1 == 0 {
        info!("触发tick进入触发TickEvent");
        ev_tick.send(TickEvent);
    }
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    info!("触发turn_update_end进入PlayerInput");
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    info!("触发turn_update_cancel 进入PlayerInput");
    next_state.set(GameState::PlayerInput);
}
