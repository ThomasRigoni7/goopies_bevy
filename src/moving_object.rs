use bevy::prelude::*;
use crate::collider::Collider;

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub speed: Vec2,
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub collider: Collider,
    pub velocity: Velocity,
}
pub struct MovingObjectPlugin;

impl Plugin for MovingObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_object);
    }
}

pub fn move_object(query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query {
        transform.translation += velocity.speed.extend(0.0);
    }
}
