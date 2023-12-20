// actions/systems.rs

use super::models::{MeleeHitAction, WalkAction};
use super::{
    Action, ActionsCompleteEvent, ActorQueue, InvalidPlayerActionEvent, NextActorEvent,
    PendingActions,
};
use crate::board::components::Position;
use crate::board::CurrentBoard;
use crate::pieces::components::{Actor, Melee, Occupier, Walk};
use crate::player::Player;
use crate::vectors::{find_path, ORTHO_DIRECTIONS};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const MOVE_SCORE: i32 = 50;
const ATTACK_SCORE: i32 = 100;
const PLAYER_ATTACK_SCORE: i32 = 100;
pub(crate) fn process_action_queue(world: &mut World) {
    if process_pending_actions(world) {
        return;
    }
    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else {
        return;
    };
    let Some(entity) = queue.0.pop_front() else {
        world.send_event(ActionsCompleteEvent);
        return;
    };
    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        return;
    };
    // clear the Actor vec
    let mut possible_actions = actor.0.drain(..).collect::<Vec<(Box<dyn Action>, i32)>>();
    // highest score first
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut success = false;
    for action in possible_actions {
        if let Ok(result) = action.0.execute(world) {
            if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
                info!("result:{:?}",result);
                pending.0 = result
            }
            success = true;
            break;
        }
    }
    if !success && world.get::<Player>(entity).is_some() {
        world.send_event(InvalidPlayerActionEvent);
        return;
    }
    world.send_event(NextActorEvent);
}

pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0.extend(query.iter());
}

pub fn plan_walk(
    mut query: Query<(&Position, &mut Actor), With<Walk>>,
    queue: Res<ActorQueue>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<Occupier>>,
    board: Res<CurrentBoard>,
) {
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((position, mut actor)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    // get all possible move targets
    let positions = ORTHO_DIRECTIONS
        .iter()
        .map(|d| *d + position.v)
        .collect::<Vec<_>>();
    let path_to_player = find_path(
        position.v,
        player_position.v,
        &board.tiles.keys().cloned().collect(),
        &occupier_query.iter().map(|p| p.v).collect(),
    );
    let mut rng = thread_rng();
    let actions = positions
        .iter()
        .map(|v| {
            // randomize movement choices
            let mut d = rng.gen_range(-10..0);
            if let Some(path) = &path_to_player {
                // however prioritze a movement if it leads to the player
                if path.contains(v) {
                    d = 5
                }
            }
            (
                Box::new(WalkAction(*entity, *v)) as Box<dyn super::Action>,
                MOVE_SCORE + d,
            )
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}

pub fn plan_melee(
    mut query: Query<(&mut Actor, &Melee)>,
    player_query: Query<&Position, With<Player>>,
    queue: Res<ActorQueue>,
) {
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((mut actor, melee)) = query.get_mut(*entity) else {
        return;
    };
    let Ok(player_position) = player_query.get_single() else {
        return;
    };
    let action = Box::new(MeleeHitAction {
        attacker: *entity,
        target: player_position.v,
        damage: melee.damage,
    });
    actor
        .0
        .push((action, PLAYER_ATTACK_SCORE + melee.damage as i32))
}

fn process_pending_actions(world: &mut World) -> bool {
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false,
    };
    info!("pending:{}",pending.len());
    let mut next = Vec::new();
    let mut success = false;
    for action in pending {
        if let Ok(result) = action.execute(world) {
            next.extend(result);
            success = true;
        }
    }
    // if there are any new actions assign them back to the resource
    // should be safe to unwrap as we confirmed the resource at the beginning
    let mut res = world.get_resource_mut::<PendingActions>().unwrap();
    res.0 = next;
    success
}
