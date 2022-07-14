use bevy::prelude::*;

pub fn approx_equal(a: f32, b: f32) -> bool {
    let margin = f32::EPSILON;
    (a - b).abs() < margin
}


pub fn screen_position_to_rect(pos: Vec2) -> Rect<Val> {
    Rect {
        left: Val::Px(pos.x),
        bottom: Val::Px(pos.y),
        ..Rect::default()
    }
}