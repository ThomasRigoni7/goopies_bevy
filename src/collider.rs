use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume}, prelude::*
};

#[derive(Component, Default)]
pub struct Collider;


pub struct Collision {
    // collision direction going from obj1 to obj2
    direction: Vec2
}

pub fn collision(obj1: BoundingCircle, obj2: BoundingCircle) -> Option<Collision> {
    if !obj1.intersects(&obj2) {
        return None;
    }

    let direction = obj2.center - obj1.center;

    Some(Collision {
        direction: direction.normalize_or_zero()
    })
}
