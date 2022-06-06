use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::physics::CollisionGroup;

pub struct Movement;

impl Plugin for Movement {
    fn build(&self, app: &mut App) {
        app.add_system(move_towards_destination)
            .add_system(snap_to_ground);
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct Destination(pub Entity);

#[derive(Component)]
pub struct Grounded {
    pub height_offset: f32,
}

pub fn move_towards_destination(
    mut commands: Commands,
    time: Res<Time>,
    mut destinations: Query<(Entity, &mut Transform, &Destination)>,
    targets: Query<&GlobalTransform, Without<Destination>>,
) {
    for (entity, mut moving, destination) in destinations.iter_mut() {
        if let Ok(target) = targets.get(**destination) {
            let target = target.translation.xz();
            let full_movement = target - moving.translation.xz();
            let mut movement = full_movement.normalize_or_zero() * time.delta_seconds() * 20.0;
            movement = movement.clamp_length_max(full_movement.length());
            if movement == Vec2::ZERO {
                // Reached destination, remove everything
                commands.entity(entity).remove::<Destination>();
                commands.entity(**destination).despawn_recursive();
            } else {
                moving.translation += movement.extend(0.0).xzy();
            }
        } else {
            // Destination does not exist or does not work as a destination, remove everything
            commands.entity(entity).remove::<Destination>();
            commands.entity(**destination).despawn_recursive();
        }
    }
}

fn snap_to_ground(
    mut commands: Commands,
    context: Res<RapierContext>,
    grounded: Query<(Entity, &GlobalTransform, &Grounded, &Parent), Changed<GlobalTransform>>,
    mut mover: Query<&mut Transform, Without<Grounded>>,
) {
    for (entity, grounded_transform, Grounded { height_offset }, grounded_parent) in grounded.iter()
    {
        if let Ok(mut mover_transform) = mover.get_mut(**grounded_parent) {
            if let Some((_, RayIntersection { point, .. })) = context.cast_ray_and_get_normal(
                grounded_transform.translation,
                -Vec3::Y,
                Real::MAX,
                false,
                InteractionGroups::all().with_filter(CollisionGroup::Terrain as u32),
                None,
            ) {
                mover_transform.translation.y = point.y + height_offset;
            }
        } else {
            // Grounded doesn't have valid parent, remove
            commands.entity(entity).despawn_recursive();
        }
    }
}
