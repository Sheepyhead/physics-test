use std::{f32::consts::PI, time::Duration};

use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_tweening::{lens::TransformRotationLens, Animator, EaseFunction, Tween};

use crate::{
    movement::{move_towards_destination, Destination},
    physics::{ray_from_screenspace, CollisionGroup},
    rendering::Pyramid,
    Player,
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
    windows: Res<Windows>,
    context: Res<RapierContext>,
    camera: Query<(&Camera, &GlobalTransform)>,
    player: Query<(Entity, Option<&Destination>), With<Player>>,
) {
    if input.pressed(MouseButton::Left) {
        if let Some(cursor_pos_screen) = windows
            .get_primary()
            .and_then(|window| window.cursor_position())
        {
            let (camera, camera_transform) = camera.single();
            let (from, to) =
                ray_from_screenspace(cursor_pos_screen, &windows, camera, camera_transform, 100.0);

            if let Some((_, RayIntersection { point, .. })) = context.cast_ray_and_get_normal(
                from,
                to,
                Real::MAX,
                false,
                InteractionGroups::all().with_filter(CollisionGroup::Terrain as u32),
                None,
            ) {
                let mut mat = StandardMaterial::from(Color::YELLOW);
                mat.unlit = true;
                let transform = Transform::from_translation(point.extend(1.0).xwz())
                    .with_rotation(Quat::from_rotation_x(PI));
                let angles = transform.rotation.to_euler(EulerRot::XYZ);
                let end_rotation =
                    Quat::from_euler(EulerRot::XYZ, angles.0, angles.1 + PI, angles.2);
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
}
