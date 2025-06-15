use rand::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::collider::Collider;
use bevy::prelude::*;


const GOOPIE_RADIUS: f32 = 30.0; // Size of the Goopie sprite
const GOOPIE_Z_INDEX: f32 = 1.0; // Z-index for rendering Goopie

#[derive(Component)]
pub struct Goopie;

pub struct GoopiePlugin;

impl Plugin for GoopiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup);
        app.add_systems(Update, move_goopie);
    }
}

fn startup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Example of spawning goopie at a specific position
    let mut rng = rand::rng();
    let angle1 = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    let angle2 = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    spawn_goopie(&mut commands, &scene_assets, angle1, Vec2::new(0.0, 0.0));
    spawn_goopie(&mut commands, &scene_assets, angle2, Vec2::new(500.0, -200.0));
}

pub fn spawn_goopie(
    commands: &mut Commands,
    scene_assets: &Res<SceneAssets>,
    angle: f32,
    translation: Vec2,
) {
    commands.spawn((
        scene_assets.goopie_sprite.clone(),
        Transform {
            translation: translation.extend(GOOPIE_Z_INDEX),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec2::new(GOOPIE_RADIUS, GOOPIE_RADIUS).extend(1.0),
        },
        Goopie,
        Collider,
    ));
}

pub fn move_goopie(query: Query<&mut Transform, With<Goopie>>) {
    for mut transform in query {
        println!("Moving goopie at {:?}", transform.translation);
        // Get the goopie's forward vector by applying the current rotation to the goopie's initial facing
        // vector
        let forward = transform.rotation * Vec3::Y;
        transform.translation += forward;
    }
}
