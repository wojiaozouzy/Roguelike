use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component, Debug)]
pub struct Piece {
    pub kind: String,
}
#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct Melee {
    // melee attack behaviour for the npcs
    pub damage: u32,
}

#[derive(Component)]
// there can be only a single occupier piece on the same tile
pub struct Occupier;

#[derive(Component)]
// movement behaviour for non-player pieces
pub struct Walk;
