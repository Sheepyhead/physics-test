use bevy::{prelude::*, render::camera::Camera3d};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

use crate::{util::screen_position_to_rect, items::Item};

pub struct UiOverlay;

impl Plugin for UiOverlay {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new().filter::<With<ItemTitleLabelUI>>())
            .add_system(spawn)
            .add_system(refresh_position_on_item_move);
    }
}

fn spawn(
    mut commands: Commands,
    ass: Res<AssetServer>,
    windows: Res<Windows>,
    imgs: Res<Assets<Image>>,
    items: Query<(Entity, &GlobalTransform), (Without<ItemTitleLabel>, With<Item>)>,
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
) {
    let (pos, camera) = camera.single();
    for (item, item_trans) in items.iter() {
        if let Some(screen_pos) =
            camera.world_to_screen(&windows, &imgs, pos, item_trans.translation)
        {
            let label = commands
                .spawn_bundle(NodeBundle {
                    color: Color::BLACK.into(),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: screen_position_to_rect(screen_pos),
                        ..default()
                    },
                    ..default()
                })
                .insert(ItemTitleLabelUI(item))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Short Stick",
                            TextStyle {
                                font: ass.load("AvQest.ttf"),
                                font_size: 20.0,
                                color: Color::ANTIQUE_WHITE,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Top,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..default()
                    });
                })
                .id();

            commands.entity(item).insert(ItemTitleLabel(label));
        }
    }
}

fn refresh_position_on_item_move(
    windows: Res<Windows>,
    imgs: Res<Assets<Image>>,
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    items: Query<(&Transform, &ItemTitleLabel), Changed<Transform>>,
    mut labels: Query<&mut Style>,
) {
    let (pos, camera) = camera.single();

    for (item_trans, label) in items.iter() {
        if let Some(screen_pos) =
            camera.world_to_screen(&windows, &imgs, pos, item_trans.translation)
        {
            if let Ok(mut label) = labels.get_mut(**label) {
                label.position = screen_position_to_rect(screen_pos);
            }
        }
    }
}

#[derive(Component, Deref, DerefMut, Inspectable)]
struct ItemTitleLabel(Entity);

#[derive(Component, Deref, DerefMut, Inspectable)]
struct ItemTitleLabelUI(Entity);
