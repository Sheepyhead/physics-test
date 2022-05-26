use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    movement::{move_towards_destination, Destination},
    physics::{ray_from_screenspace, CollisionGroup},
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
                let destination = commands
                    .spawn_bundle((
                        Collider::cuboid(0.2, 0.2, 0.2),
                        CollisionGroups::new(0, 0),
                        Transform::from_translation(point),
                        GlobalTransform::default(),
                    ))
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
