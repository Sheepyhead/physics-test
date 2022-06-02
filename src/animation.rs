use std::fmt;

use bevy::{gltf::Gltf, prelude::*};

use crate::preload_assets::Animations;

pub struct Animation;

impl Plugin for Animation {
    fn build(&self, app: &mut App) {
        app.add_system(setup_animation)
            .add_system(animate.after(setup_animation));
    }
}

fn setup_animation(mut commands: Commands, players: Query<Entity, Added<AnimationPlayer>>) {
    for player in players.iter() {
        commands.entity(player).insert(AnimationId::CombatIdleLoop);
    }
}

fn animate(
    animations: Option<Res<Animations>>,
    gltfs: Res<Assets<Gltf>>,
    mut players: Query<(&mut AnimationPlayer, &AnimationId), Changed<AnimationId>>,
) {
    for (mut player, state) in players.iter_mut() {
        if let Some(animations) = &animations {
            player
                .play(
                    gltfs
                        .get(&animations.player)
                        .and_then(|gltf| gltf.named_animations.get(&state.to_string()))
                        .unwrap()
                        .clone_weak(),
                )
                .repeat();
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Component, Copy)]
pub enum AnimationId {
    LookDown,
    HoldRifleLoop,
    Vault,
    CrouchIdleLoop,
    Slide,
    LookUp,
    AimRifle,
    CrouchLeftLoop,
    TPose,
    LookRightDown,
    RunRightLoop,
    FireRifle,
    LookLeftUp,
    LookLeftDown,
    LookLeft,
    AimFireRifle,
    LookRightUp,
    RunForwardLoop,
    TurnRight,
    CrouchForwardLoop,
    RunLeftLoop,
    TurnLeft,
    SprintLoop,
    LookRight,
    CrouchRightLoop,
    ArmatureAction,
    CombatIdleLoop,
    CrouchBackwardLoop,
}

impl fmt::Display for AnimationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AnimationId::*;
        write!(
            f,
            "{}",
            match self {
                HoldRifleLoop => "HoldRifle-loop".to_string(),
                CrouchIdleLoop => "CrouchIdle-loop".to_string(),
                CrouchLeftLoop => "CrouchLeft-loop".to_string(),
                RunRightLoop => "RunRight-loop".to_string(),
                RunForwardLoop => "RunForward-loop".to_string(),
                CrouchForwardLoop => "CrouchForward-loop".to_string(),
                RunLeftLoop => "RunLeft-loop".to_string(),
                SprintLoop => "Sprint-loop".to_string(),
                CrouchRightLoop => "CrouchRight-loop".to_string(),
                CombatIdleLoop => "CombatIdle-loop".to_string(),
                CrouchBackwardLoop => "CrouchBackward-loop".to_string(),
                other => other.to_string(),
            }
        )
    }
}
