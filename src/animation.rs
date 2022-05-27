use bevy::prelude::*;

pub struct Animation;

impl Plugin for Animation {
    fn build(&self, app: &mut App) {
        app.add_system(setup_animation).add_system(animate);
    }
}

#[derive(Component)]
pub enum AnimationState {
    Idle,
}

impl AnimationState {
    fn path(&self) -> &str {
        match self {
            AnimationState::Idle => "anim_run.glb#Animation0",
        }
    }
}

fn setup_animation(mut commands: Commands, players: Query<Entity, Added<AnimationPlayer>>) {
    for player in players.iter() {
        commands.entity(player).insert(AnimationState::Idle);
    }
}

fn animate(
    ass: Res<AssetServer>,
    mut players: Query<(&mut AnimationPlayer, &AnimationState), Changed<AnimationState>>,
) {
    for (mut player, state) in players.iter_mut() {
        let handle = ass.load(state.path());

        player.play(handle.clone_weak()).repeat();
    }
}
