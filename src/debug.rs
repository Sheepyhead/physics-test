use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

pub struct Debug;

impl Plugin for Debug {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_item);
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
                    Item,
                ));
        }
    }
}

#[derive(Component)]
pub struct Item;
