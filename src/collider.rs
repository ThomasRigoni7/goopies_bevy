use bevy::{
    math::bounding::{BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::moving_object::Velocity;

#[derive(Component, Default)]
pub struct Collider;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions);
    }
}

pub struct Collision {
    // collision direction going from obj1 to obj2
    direction: Vec2,
}

// Check collisions between entities with Collider components.
// Build a BoundingCircle for each entity and check for intersections.
pub fn check_collisions(mut query: Query<(&Transform, &mut Velocity), With<Collider>>) {
    // Collect mutable references into a Vec to avoid aliasing issues
    let mut transforms: Vec<_> = query.iter_mut().collect();

    for i in 0..transforms.len() {
        // Split the slice at i + 1 to get two non-overlapping mutable slices
        let (left, right) = transforms.split_at_mut(i + 1);
        let (transform1, velocity1) = &mut left[i];
        for j in 0..right.len() {
            let (transform2, velocity2) = &mut right[j];

            assert_eq!(
                transform1.scale.x, transform1.scale.y,
                "Colliders must be circles. Found non-uniform scale on entity {}.",
                i
            );
            assert_eq!(
                transform2.scale.x,
                transform2.scale.y,
                "Colliders must be circles. Found non-uniform scale on entity {}.",
                j + i + 1
            );

            let circle1 = BoundingCircle {
                center: transform1.translation.truncate(),
                circle: Circle {
                    radius: transform1.scale.x,
                },
            };
            let circle2 = BoundingCircle {
                center: transform2.translation.truncate(),
                circle: Circle {
                    radius: transform2.scale.x,
                },
            };

            if let Some(collision) = collision(circle1, circle2) {
                println!(
                    "Collision detected between entity {} and entity {}: {:?}",
                    i,
                    j + i + 1,
                    collision.direction
                );
                velocity1.speed -= collision.direction;
                velocity2.speed += collision.direction;
            }
        }
    }
}

pub fn collision(obj1: BoundingCircle, obj2: BoundingCircle) -> Option<Collision> {
    if !obj1.intersects(&obj2) {
        return None;
    }

    let direction = obj2.center - obj1.center;

    if direction == Vec2::ZERO {
        // If the centers are the same, we can't determine a direction
        return None;
    }

    Some(Collision {
        direction: direction
            * ((obj1.circle.radius + obj2.circle.radius) / direction.length())
            * 0.01,
    })
}
