use bevy::{gltf::Gltf, prelude::*};

pub struct PreloadAssets;

impl Plugin for PreloadAssets {
    fn build(&self, app: &mut App) {
        app.add_startup_system(preload)
            .add_system(setup_animations)
            .add_system(inspect_gltf);
    }
}

struct PreloadedAssets(Vec<HandleUntyped>);

#[derive(Debug)]
pub struct Animations {
    pub player: Handle<Gltf>,
}

fn preload(mut commands: Commands, ass: Res<AssetServer>) {
    let mut handles = vec![];

    let path = "Character.gltf";
    handles.push(ass.load_untyped(path));

    handles.push(ass.load_untyped("grass.jpg"));
    commands.insert_resource(PreloadedAssets(handles));
}

fn setup_animations(mut commands: Commands, gltfs: Res<Assets<Gltf>>) {
    if !gltfs.is_changed() {
        return;
    }

    for (handle, _) in gltfs.iter() {
        commands.insert_resource(Animations {
            player: gltfs.get_handle(handle),
        });
    }
}

fn inspect_gltf(mut inspected: Local<bool>, ass: Res<AssetServer>, gltfs: Res<Assets<Gltf>>) {
    if *inspected {
        return;
    }

    if let Some(_gltf) = gltfs.get(ass.load_untyped("Character.gltf")) {
        *inspected = true;

        // info!("{gltf:?}");
    }
}
