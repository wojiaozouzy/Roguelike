use std::any::Any;

// actions/modeks.rs
use bevy::prelude::*;

use crate::{
    board::{components::Position, CurrentBoard},
    pieces::components::{Health, Occupier, Piece},
    vectors::Vector2Int,
};

use super::Action;

pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        if world
            .query_filtered::<&Position, With<Occupier>>()
            .iter(world)
            .any(|p| p.v == self.1)
        {
            return Err(());
        };
        let board = world.get_resource::<CurrentBoard>().ok_or(())?;
        if !board.tiles.contains_key(&self.1) {
            return Err(());
        };

        let mut position = world.get_mut::<Position>(self.0).ok_or(())?;
        position.v = self.1;
        Ok(Vec::new())
    }

    fn name(&self) -> String {
        format!("MeleeHitAction{:?}{:?}", self.0, self.1)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
pub struct MeleeHitAction {
    pub attacker: Entity,
    pub target: Vector2Int,
    pub damage: u32,
}
impl Action for MeleeHitAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        info!("近战行为");
        let attacker_position = world.get::<Position>(self.attacker).ok_or(())?;
        info!(
            "攻击者的位置:{:?}==攻击者的id{:?}",
            attacker_position.v, self.attacker
        );
        info!("玩家的位置:{:?}", self.target);
        if attacker_position.v.manhattan(self.target) > 1 {
            info!("近战行为距离大于1米");
            return Err(());
        };
        info!("近战行为距离小于1米");
        let target_entities = world
            .query_filtered::<(Entity, &Piece, &Position), With<Health>>()
            .iter(world)
            .filter(|(e, e1, p)| {
                info!("所有对象有血量的对象e:{:?}==>类型{:?};p{:?}", e, e1, p);
                p.v == self.target
            })
            .collect::<Vec<_>>();

        if target_entities.len() == 0 {
            return Err(());
        };
        target_entities
            .iter()
            .for_each(|(e, e1, p)| info!("有血量的对象e:{:?}==>类型{:?};p{:?}", e, e1, p));

        let result = target_entities
            .iter()
            .map(|e| Box::new(DamageAction(e.0, self.damage)) as Box<dyn Action>)
            .collect::<Vec<_>>();

        info!("line{};result{:?}", line!(), result);
        Ok(result)
    }
    fn name(&self) -> String {
        format!(
            "MeleeHitAction{:?}{:?}{:?}",
            self.attacker, self.target, self.damage
        )
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
pub struct DamageAction(pub Entity, pub u32);
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.0) else {
            return Err(());
        };
        info!("被攻击的血量{:?}", health.value);
        info!("被攻击的对象{:?}", self.0);
        health.value = health.value.saturating_sub(self.1);
        if health.value == 0 {
            // the unit is killed
            world.despawn(self.0);
        }
        Ok(Vec::new())
    }
    fn name(&self) -> String {
        format!("DamageAction{:?}{:?}", self.0, self.1)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
