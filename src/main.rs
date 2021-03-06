#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::must_use_candidate,
    clippy::enum_glob_use
)]

use animation::Animation;
use bevy::{
    input::system::exit_on_esc_system,
    math::Vec3Swizzles,
    prelude::{shape::Plane, *},
    window::PresentMode,
};
use bevy_rapier3d::prelude::*;
use bevy_tweening::TweeningPlugin;
use camera::Camera;
use controls::Controls;
use enemy::Enemies;
use items::Items;
use movement::{Grounded, Movement};
use physics::{CollisionGroup, Physics};
use preload_assets::PreloadAssets;
use rendering::Rendering;
use ui_overlay::UiOverlay;

mod animation;
mod camera;
mod controls;
mod debug;
mod enemy;
mod items;
mod movement;
mod physics;
mod preload_assets;
mod rendering;
mod ui_overlay;
pub mod util;

pub const CLEAR: Color = Color::BLACK;
pub const HEIGHT: f32 = 600.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const PLAYER_SPAWN: [f32; 3] = [0.0, 0.5, 0.0];

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            title: "Bevy Template".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system)
        .add_plugin(TweeningPlugin)
        .add_plugin(Camera)
        .add_plugin(Animation)
        .add_plugin(Controls)
        .add_plugin(Enemies)
        .add_plugin(Items)
        .add_plugin(Movement)
        .add_plugin(Physics)
        .add_plugin(PreloadAssets)
        .add_plugin(Rendering)
        .add_plugin(UiOverlay)
        .add_startup_system(setup);

    #[cfg(debug_assertions)]
    app.add_plugin(debug::Debug);

    app.run();
}

#[allow(dead_code)]
fn slow_down() {
    std::thread::sleep(std::time::Duration::from_secs_f32(1.000));
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        ..default()
    });
    for x in -10..10 {
        for z in -10..10 {
            commands
                .spawn_bundle((
                    Collider::cuboid(5.0, 0.01, 5.0),
                    CollisionGroups {
                        memberships: CollisionGroup::Terrain as u32,
                        ..default()
                    },
                    Name::new("Ground"),
                    Ground,
                ))
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Plane { size: 10.0 }.into()),
                    material: materials.add(ass.load("grass.jpg").into()),
                    transform: Transform::from_xyz(10.0 * x as f32, 0.0, 10.0 * z as f32),
                    ..default()
                });
        }
    }
    let player_height = 1.0;
    commands
        .spawn_bundle(TransformBundle {
            local: Transform::from_translation(PLAYER_SPAWN.into()),
            ..default()
        })
        .insert_bundle((
            Collider::capsule_y(player_height / 2.0, 1.0),
            Player,
            Name::new("Player"),
            CollisionGroups::new(0, u32::MAX),
        ))
        .with_children(|parent| {
            parent.spawn_bundle((
                Grounded {
                    height_offset: player_height / 2.0,
                },
                Transform::from_translation(Vec2::ZERO.extend(player_height / 2.0).xzy()),
                GlobalTransform::default(),
            ));

            parent.spawn_scene(ass.load("Character.gltf#Scene0"));
        });
}

#[derive(Component)]
struct Player;
