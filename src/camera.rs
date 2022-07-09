use bevy::{prelude::*, render::camera::Camera3d};
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::{
    math::Real,
    plugin::RapierContext,
    prelude::{InteractionGroups, RayIntersection},
};

use crate::{
    physics::{ray_from_screenspace, CollisionGroup},
    Player, PLAYER_SPAWN,
};

pub struct Camera;

#[derive(Deref, DerefMut)]
pub struct CameraOffset(Vec3);

impl Plugin for Camera {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraOffset(Vec3::new(15.0, 15.0, 15.0)))
            .insert_resource(UnderCursor::default())
            .add_startup_system(spawn_camera)
            .add_system(follow_player)
            .add_system(update_under_cursor);
    }
}

fn spawn_camera(mut commands: Commands, offset: Res<CameraOffset>) {
    let mut camera = PerspectiveCameraBundle::new_3d();

    camera.transform.translation = Vec3::from(PLAYER_SPAWN) + **offset;
    camera.transform.look_at(PLAYER_SPAWN.into(), Vec3::Y);
    camera.perspective_projection.fov = 0.5;

    commands.spawn_bundle(camera).insert(Name::new("Camera"));
}

fn follow_player(
    offset: Res<CameraOffset>,
    player: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    if let Ok(player_pos) = player.get_single() {
        let mut camera_pos = camera.single_mut();
        camera_pos.translation = player_pos.translation + **offset;
    }
}

#[derive(Debug, Default, Deref, DerefMut, Inspectable)]
pub struct UnderCursor(pub Option<Collision>);

#[derive(Debug, Inspectable)]
pub struct Collision {
    pub hit: Entity,
    pub intersection: Vec3,
}

fn update_under_cursor(
    windows: Res<Windows>,
    context: Res<RapierContext>,
    mut under_cursor: ResMut<UnderCursor>,
    camera: Query<(&bevy::prelude::Camera, &GlobalTransform)>,
) {
    if let Some(cursor_pos_screen) = windows.get_primary().and_then(Window::cursor_position) {
        let (camera, camera_transform) = camera.single();
        let (from, to) =
            ray_from_screenspace(cursor_pos_screen, &windows, camera, camera_transform, 100.0);

        if let Some((hit, RayIntersection { point, .. })) = context.cast_ray_and_get_normal(
            from,
            to,
            Real::MAX,
            false,
            InteractionGroups::all().with_filter(CollisionGroup::Terrain as u32),
            None,
        ) {
            **under_cursor = Some(Collision {
                hit,
                intersection: point,
            });
        } else {
            **under_cursor = None;
        }
    }
}
