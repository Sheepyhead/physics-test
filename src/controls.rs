use std::{f32::consts::PI, time::Duration};

use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_tweening::{lens::TransformRotationLens, Animator, EaseFunction, Tween};

use crate::{
    camera::UnderCursor,
    movement::{move_towards_destination, Destination},
    rendering::Pyramid,
    Ground, Player,
};

pub struct Controls;

impl Plugin for Controls {
    fn build(&self, app: &mut App) {
        app.add_system(click_ground.after(move_towards_destination));
    }
}

fn click_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
    input: Res<Input<MouseButton>>,
    under_cursor: Res<UnderCursor>,
    player: Query<(Entity, Option<&Destination>), With<Player>>,
    ground: Query<(), With<Ground>>,
) {
    if input.pressed(MouseButton::Left) {
        if let Some(under_cursor) = &under_cursor.0 {
            if ground.get(under_cursor.hit).is_err() {
                return;
            }

            let mut mat = StandardMaterial::from(Color::YELLOW);
            mat.unlit = true;
            let transform =
                Transform::from_translation(under_cursor.intersection.extend(1.0).xwz())
                    .with_rotation(Quat::from_rotation_x(PI));
            let angles = transform.rotation.to_euler(EulerRot::XYZ);
            let end_rotation = Quat::from_euler(EulerRot::XYZ, angles.0, angles.1 + PI, angles.2);
            let destination = commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(
                        Pyramid {
                            base_side_length: 0.5,
                            height: 0.5,
                        }
                        .into(),
                    ),
                    material: mats.add(mat),
                    transform,
                    ..default()
                })
                .insert(Animator::new(Tween::new(
                    EaseFunction::CircularInOut,
                    bevy_tweening::TweeningType::PingPong,
                    Duration::from_secs_f32(0.5),
                    TransformRotationLens {
                        start: transform.rotation,
                        end: end_rotation,
                    },
                )))
                .id();

            let (player, old_destination) = player.single();
            if let Some(old_destination) = old_destination {
                commands.entity(**old_destination).despawn();
            }
            commands.entity(player).insert(Destination(destination));
        }
    }
}
