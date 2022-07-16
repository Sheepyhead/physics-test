use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{movement::Grounded, physics::CollisionGroup};

pub struct Enemies;

impl Plugin for Enemies {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemy>().add_system(spawn_enemy);
    }
}

pub struct SpawnEnemy {
    pub transform: Transform,
}

fn spawn_enemy(mut commands: Commands, mut events: EventReader<SpawnEnemy>) {
    for SpawnEnemy { transform } in events.iter() {
        let enemy_height = 1.0;
        commands
            .spawn_bundle(TransformBundle::from_transform(*transform))
            .insert_bundle((
                Collider::capsule_y(enemy_height / 2.0, 1.0),
                Enemy,
                CollisionGroups::new(CollisionGroup::Enemy as u32, u32::MAX),
            ))
            .with_children(|parent| {
                parent.spawn_bundle((
                    Grounded {
                        height_offset: enemy_height / 2.0,
                    },
                    Transform::from_translation(Vec2::ZERO.extend(enemy_height / 2.0).xzy()),
                    GlobalTransform::default(),
                ));
            });
    }
}

#[derive(Component)]
pub struct Enemy;
