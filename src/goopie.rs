use crate::{asset_loader::SceneAssets, GRAVITY};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

const GOOPIE_SPAWN_AREA: f32 = 1000.0; // Area in which Goopie can spawn
const GOOPIE_RADIUS: f32 = 30.0; // Size of the Goopie sprite
const GOOPIE_VISION_RADIUS: f32 = 200.0; // Vision radius for Goopie
const GOOPIE_Z_INDEX: f32 = 1.0; // Z-index for rendering Goopie
const GOOPIE_DENSITY: f32 = 1.1; // Density of Goopie for physics simulation

#[derive(Component, Debug, Clone, Copy)]
pub struct Goopie;

pub struct GoopiePlugin;

impl Plugin for GoopiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup);
        app.add_systems(Update, apply_forward_impulse);
        app.add_systems(Update, print_mass);
    }
}

fn startup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Example of spawning goopie at a specific position
    // let mut rng = rand::rng();
    let angle1 = 3.0 / 2.0 * std::f32::consts::PI; // 270 degrees in radians
    let angle2 = std::f32::consts::PI / 2.0;
    spawn_goopie(
        &mut commands,
        &scene_assets,
        angle1,
        Vec2::new(10.0, 10.0),
        Vec2::new(0.0, 0.0),
    );
    spawn_goopie(
        &mut commands,
        &scene_assets,
        angle2,
        Vec2::new(300.0, -15.0),
        Vec2::new(0.0, 0.0),
    );

    random_spawn_goopie(&mut commands, &scene_assets, true);
    random_spawn_goopie(&mut commands, &scene_assets, true);
    random_spawn_goopie(&mut commands, &scene_assets, true);
    random_spawn_goopie(&mut commands, &scene_assets, true);
}

pub fn random_spawn_goopie(
    commands: &mut Commands,
    scene_assets: &Res<SceneAssets>,
    with_speed: bool,
) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    let translation: Vec2 = Vec2::new(
        rng.random_range(-GOOPIE_SPAWN_AREA..GOOPIE_SPAWN_AREA),
        rng.random_range(-GOOPIE_SPAWN_AREA..GOOPIE_SPAWN_AREA),
    );
    if with_speed {
        let speed = Vec2::new(rng.random_range(-10.0..10.0), rng.random_range(-10.0..10.0));
        spawn_goopie(commands, scene_assets, angle, translation, speed);
        return;
    }

    spawn_goopie(
        commands,
        scene_assets,
        angle,
        translation,
        Vec2::new(0.0, 0.0),
    );
}

pub fn spawn_goopie(
    commands: &mut Commands,
    scene_assets: &Res<SceneAssets>,
    angle: f32,
    translation: Vec2,
    speed: Vec2,
) {
    commands
        .spawn((
            RigidBody::Dynamic,
            scene_assets.goopie_sprite.clone(),
            Collider::ball(1.0),
            ColliderMassProperties::Density(GOOPIE_DENSITY),
            Transform {
                translation: translation.extend(GOOPIE_Z_INDEX),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec2::new(GOOPIE_RADIUS, GOOPIE_RADIUS).extend(1.0),
            },
            Velocity {
                linvel: speed,
                ..default()
            },
            Sleeping::disabled(),
            Damping {
                linear_damping: 0.3,
                angular_damping: 0.3,
            },
            GravityScale(GRAVITY),
            ExternalImpulse::default(),
            Goopie,
        ))
        .with_child((Collider::ball(GOOPIE_VISION_RADIUS), Sensor, ColliderMassProperties::Mass(0.0),));
}


pub fn apply_forward_impulse(
    mut query: Query<(&mut ExternalImpulse, &Transform), With<Goopie>>,
) {
    for (mut ext_impulse, transform) in query.iter_mut() {
        let forward = transform.rotation * Vec3::X;
        let impulse = forward * 100.0; // Adjust the impulse strength as needed
        // println!("Applying impulse: {:?}", impulse);
        ext_impulse.impulse = impulse.truncate();
    }
}

pub fn print_mass(
    mut query: Query<&mut ReadMassProperties>,
){
    for mass_properties in query.iter_mut() {
        println!("Goopie mass: {}", mass_properties.mass);
    }
}