use bevy::prelude::*;

pub struct Items;

impl Plugin for Items {
    fn build(&self, _: &mut App) {}
}

#[derive(Component)]
pub struct Item;
