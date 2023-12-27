// actions/systems.rs

use super::models::{MeleeHitAction, WalkAction};
use super::{
    Action, ActionExecutedEvent, ActionsCompleteEvent, ActorQueue, InvalidPlayerActionEvent,
    NextActorEvent, PendingActions,
};

use crate::board::components::Position;
use crate::board::CurrentBoard;
use crate::pieces::components::{Actor, Melee, Occupier, Walk};
use crate::player::Player;
use crate::vectors::{find_path, Vector2Int, ORTHO_DIRECTIONS};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

const MOVE_SCORE: i32 = 50;
const ATTACK_SCORE: i32 = 100;
const PLAYER_ATTACK_SCORE: i32 = 100;
pub(crate) fn process_action_queue(world: &mut World) {
    info!("进入process_action_queue");
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
        // this can mean that the current actor
        // has been removed from the world since creating the queue
        // cue the next one
        world.send_event(NextActorEvent);
        return;
    };
    info!("行动对象entity:{:?}", entity);
    // clear the Actor vec
    let mut possible_actions = actor.0.drain(..).collect::<Vec<(Box<dyn Action>, i32)>>();
    // highest score first
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    possible_actions
        .iter()
        .for_each(|(a, b)| info!("实现的action名字:{:?}==>独占系统查询优先级index:{}", a, b));
    let mut success = false;
    for action in possible_actions {
        success = success || execute_action(action.0, world);
        if success {
            break;
        }
    }
    info!(
        "success{}:玩家对象:{:?}",
        success,
        world.get::<Player>(entity).is_some()
    );
    if !success && world.get::<Player>(entity).is_some() {
        info!("触发一下InvalidPlayerActionEvent");
        world.send_event(InvalidPlayerActionEvent);
        info!("触发一下InvalidPlayerActionEvent 完成");
        return;
    }
    info!("触发一下NextActorEvent");
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
    info!("plan_walk");
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
    info!("plan_melee");
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
    info!("进入缓存的process_pending_actions");
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false,
    };
    let mut success = false;
    for action in pending {
        success = success || execute_action(action, world);
    }
    success
}

fn execute_action(action: Box<dyn super::Action>, world: &mut World) -> bool {
    if let Ok(result) = action.execute(world) {
        if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
            pending.0.extend(result);
        }
        world.send_event(ActionExecutedEvent(action));
        return true;
    }
    false
}
