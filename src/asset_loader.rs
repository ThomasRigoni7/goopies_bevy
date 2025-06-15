use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub goopie_sprite: Sprite,
    pub food_sprite: Sprite,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(asset_server: Res<AssetServer>, mut assets: ResMut<SceneAssets>) {
    let goopie_handle = asset_server.load("goopie.png");
    let food_handle = asset_server.load("food.png");

    assets.goopie_sprite = Sprite {
        image: goopie_handle,
        custom_size: Some(Vec2::new(2.0, 2.0)), // Set the size of the sprite itself to be 1 unit
        ..default()
    };
    assets.food_sprite = Sprite {
        image: food_handle,
        custom_size: Some(Vec2::new(2.0, 2.0)), // Set the size of the sprite itself to be 1 unit
        ..default()
    };
}