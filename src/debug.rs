use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;

use crate::{enemy::SpawnEnemy, items::Item, physics::CollisionGroup, Ground};

pub struct Debug;

impl Plugin for Debug {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::default().filter::<Without<Ground>>())
        .add_system(spawn_item)
        .add_system(spawn_enemy)
        .add_system(toggle_inspector);
    }
}

fn spawn_item(
    mut commands: Commands,
    mut events: EventReader<KeyboardInput>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for event in events.iter() {
        if let KeyboardInput {
            key_code: Some(KeyCode::I),
            state: ElementState::Pressed,
            ..
        } = event
        {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(shape::Box::new(0.1, 0.1, 1.).into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                })
                .insert_bundle((
                    Collider::cuboid(0.25, 0.25, 0.25),
                    RigidBody::Dynamic,
                    Velocity {
                        linvel: [1.0, 10.0, 1.0].into(),
                        angvel: [3.0, 3.0, 3.0].into(),
                    },
                    CollisionGroups::new(
                        CollisionGroup::Item as u32,
                        CollisionGroup::Terrain as u32,
                    ),
                    Item,
                ));
        }
    }
}

fn spawn_enemy(mut events: EventReader<KeyboardInput>, mut spawn: EventWriter<SpawnEnemy>) {
    for event in events.iter() {
        if let KeyboardInput {
            key_code: Some(KeyCode::E),
            state: ElementState::Pressed,
            ..
        } = event
        {
            spawn.send(SpawnEnemy {
                transform: Transform::default(),
            });
        }
    }
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled;
    }
}
