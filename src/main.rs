#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::{
    math::Vec3Swizzles,
    prelude::{shape::Cube, *},
    window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use controls::Controls;
use movement::{Grounded, Movement};
use physics::{CollisionGroup, Physics};

mod controls;
mod movement;
mod physics;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Controls)
        .add_plugin(Movement)
        .add_plugin(Physics)
        .add_startup_system(spawn_camera)
        .add_startup_system(setup)
        .add_system(toggle_inspector)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = PerspectiveCameraBundle::new_3d();

    camera.transform.translation = Vec3::splat(20.0);
    camera.transform.look_at(Vec3::splat(0.0), Vec3::Y);

    commands.spawn_bundle(camera);
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn_bundle((
        Collider::cuboid(100.0, 1.0, 100.0),
        CollisionGroups {
            memberships: CollisionGroup::Terrain as u32,
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Name::new("Ground"),
    ));

    let player_height = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                Cube {
                    size: player_height / 2.0,
                }
                .into(),
            ),
            transform: Transform::from_xyz(0.0, player_height / 2.0, 0.0),
            global_transform: GlobalTransform::default(),
            ..default()
        })
        .insert_bundle((
            Collider::cuboid(1.0, player_height, 1.0),
            Player,
            Name::new("Player"),
        ))
        .with_children(|parent| {
            parent.spawn_bundle((
                Grounded {
                    height_offset: player_height / 2.0,
                },
                Transform::from_translation(Vec2::ZERO.extend(player_height / 2.0).xzy()),
                GlobalTransform::default(),
            ));
        });
}

#[derive(Component)]
struct Player;
