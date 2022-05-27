use bevy::{gltf::Gltf, prelude::*};

pub struct PreloadAssets;

impl Plugin for PreloadAssets {
    fn build(&self, app: &mut App) {
        app.add_startup_system(preload).add_system(inspect_gltf);
    }
}

struct PreloadedAssets(Vec<HandleUntyped>);

fn preload(mut commands: Commands, ass: Res<AssetServer>) {
    let mut handles = vec![];
    for path in ["anim_idle.glb", "anim_run.glb", "ybot.glb"] {
        handles.push(ass.load_untyped(path));
    }
    commands.insert_resource(PreloadedAssets(handles));
}

fn inspect_gltf(mut inspected: Local<bool>, ass: Res<AssetServer>, gltfs: Res<Assets<Gltf>>) {
    if *inspected {
        return;
    }

    if let Some(gltf) = gltfs.get(ass.load_untyped("anim_idle.glb")) {
        *inspected = true;

        info!("{gltf:?}");
    }
    
}
