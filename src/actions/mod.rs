use bevy::prelude::*;
use std::collections::VecDeque;

use crate::states::GameState;
pub mod models;
mod systems;
pub struct ActionsPlugin;
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .configure_sets(
                Update,
                (
                    ActionSet::Planning.run_if(on_event::<NextActorEvent>()),
                    ActionSet::Planning.before(ActionSet::Late),
                ),
            )
            .add_systems(
                Update,
                (
                    systems::process_action_queue
                        .run_if(on_event::<TickEvent>())
                        .in_set(ActionSet::Late),
                    (systems::plan_walk, systems::plan_melee).in_set(ActionSet::Planning),
                ),
            )
            .add_systems(
                OnExit(GameState::PlayerInput),
                systems::populate_actor_queue,
            );
    }
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
}
impl std::fmt::Debug for dyn Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 在这里实现输出的格式
    
        write!(f, "{:?}", self as *const _)
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    Planning,
    Late,
}
#[derive(Default, Resource)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);
 
#[derive(Default, Resource)]
pub struct ActorQueue(pub VecDeque<Entity>);
#[derive(Event)]
pub struct TickEvent;
#[derive(Event)]
pub struct NextActorEvent;
#[derive(Event)]
pub struct ActionsCompleteEvent;
#[derive(Event)]
pub struct InvalidPlayerActionEvent;
