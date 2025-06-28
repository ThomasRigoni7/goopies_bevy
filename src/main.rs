use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use iyes_perf_ui::prelude::*;
mod asset_loader;
mod food;
mod goopie;
mod camera;

const GRAVITY: f32 = 0.0;

fn startup(mut commands: Commands) {
    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiDefaultEntries::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: "Goopie Game".to_string(),
                present_mode: bevy::window::PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(asset_loader::AssetLoaderPlugin)
        .add_plugins(goopie::GoopiePlugin)
        .add_plugins(food::FoodPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, startup)
        .run();
}
