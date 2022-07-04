use bevy::{prelude::*, render::camera::Camera3d};

use crate::{Player, PLAYER_SPAWN};

pub struct Camera;

#[derive(Deref, DerefMut)]
pub struct CameraOffset(Vec3);

impl Plugin for Camera {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraOffset(Vec3::new(15.0, 15.0, 15.0)))
            .add_startup_system(spawn_camera)
            .add_system(follow_player);
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
