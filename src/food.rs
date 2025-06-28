use bevy::prelude::*;
use rand::prelude::*;
use crate::collider::Collider;
use crate::asset_loader::SceneAssets;
use crate::moving_object::{MovingObjectBundle, Velocity};

const FOOD_RADIUS: f32 = 10.0; // Size of the Food sprite
const FOOD_Z_INDEX: f32 = 0.0; // Z-index for rendering Food

#[derive(Component, Debug, Clone, Copy)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup);
    }
}

fn startup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Example of spawning food at a specific position
    spawn_food(&mut commands, &scene_assets, Vec2::new(-100.0, 0.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-100.0, -50.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-100.0, -100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-50.0, -100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(0.0, -100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(50.0, -100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(100.0, -100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(100.0, -50.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(100.0, 0.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(100.0, 50.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(100.0, 100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(50.0, 100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(0.0, 100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-50.0, 100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-100.0, 100.0));
    spawn_food(&mut commands, &scene_assets, Vec2::new(-100.0, 50.0));
}

pub fn spawn_food(commands: &mut Commands, scene_assets: &Res<SceneAssets>, translation: Vec2) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    commands.spawn((
        MovingObjectBundle {
            sprite: scene_assets.food_sprite.clone(),
            transform: Transform {
                translation: translation.extend(FOOD_Z_INDEX),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec2::new(FOOD_RADIUS, FOOD_RADIUS).extend(1.0),
            },
            collider: Collider,
            velocity: Velocity {
                speed: Vec2::new(0.0, 0.0), // Food starts stationary
            },
        },
        Food,
    ));
}