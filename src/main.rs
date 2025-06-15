use bevy::prelude::*;
mod collider;
mod goopie;
mod food;
mod asset_loader;

#[derive(Component)]
struct CameraMarker;

fn startup(mut commands: Commands){
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(goopie::GoopiePlugin)
        .add_plugins(food::FoodPlugin)
        .add_systems(Startup, startup)
        .run();
}

