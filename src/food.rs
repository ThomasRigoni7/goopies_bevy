use bevy::prelude::*;
use rand::prelude::*;
use crate::collider::Collider;
use crate::asset_loader::SceneAssets;

const FOOD_RADIUS: f32 = 10.0; // Size of the Food sprite
const FOOD_Z_INDEX: f32 = 0.0; // Z-index for rendering Food

#[derive(Component)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, startup);
    }
}

fn startup(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // Example of spawning food at a specific position
    spawn_food(&mut commands, &scene_assets, Vec2::new(-150.0, 100.0));
}

pub fn spawn_food(commands: &mut Commands, scene_assets: &Res<SceneAssets>, translation: Vec2) {
    let mut rng = rand::rng();
    let angle = rng.random_range(0.0..std::f32::consts::PI * 2.0);
    commands.spawn((
        scene_assets.food_sprite.clone(),
        Transform {
            translation: translation.extend(FOOD_Z_INDEX),
            rotation: Quat::from_rotation_z(angle),
            scale: Vec2::new(FOOD_RADIUS, FOOD_RADIUS).extend(1.0),
        },
        Collider,
        Food,
    ));
}