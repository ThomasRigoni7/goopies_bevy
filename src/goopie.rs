use rand::prelude::*;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collider::Collider;
use crate::moving_object::{MovingObjectBundle, Velocity};


const GOOPIE_RADIUS: f32 = 30.0; // Size of the Goopie sprite
const GOOPIE_Z_INDEX: f32 = 1.0; // Z-index for rendering Goopie

#[derive(Component, Debug, Clone, Copy)]
pub struct Goopie;

pub struct GoopiePlugin;

impl Plugin for GoopiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup);
    }
}

fn startup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Example of spawning goopie at a specific position
    // let mut rng = rand::rng();
    let angle1 = 3.0/2.0 * std::f32::consts::PI; // 270 degrees in radians
    let angle2 = std::f32::consts::PI / 2.0;
    spawn_goopie(&mut commands, &scene_assets, angle1, Vec2::new(0.0, 0.0), Vec2::new(0.2, 0.0));
    spawn_goopie(&mut commands, &scene_assets, angle2, Vec2::new(300.0, 0.0), Vec2::new(-0.3, 0.0));
}

pub fn spawn_goopie(
    commands: &mut Commands,
    scene_assets: &Res<SceneAssets>,
    angle: f32,
    translation: Vec2,
    speed: Vec2
) {
commands.spawn((
    MovingObjectBundle {
        sprite: scene_assets.goopie_sprite.clone(),
        transform: Transform {
            translation: translation.extend(GOOPIE_Z_INDEX),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec2::new(GOOPIE_RADIUS, GOOPIE_RADIUS).extend(1.0),
        },
        collider: Collider,
        velocity: Velocity {
            speed: speed, // Goopie starts stationary
        },
    },
    Goopie,
));
}

