use std::ops::Range;

use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

#[derive(Debug, Resource)]
pub struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            orthographic_viewport_height: 100.0,
            orthographic_zoom_range: 0.1..10.0,
            orthographic_zoom_speed: 0.1,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Startup, startup)
            .add_systems(Update, zoom);
    }
}

fn startup(mut commands: Commands) {
    commands.insert_resource(CameraSettings {
        orthographic_viewport_height: 100.0,
        orthographic_zoom_range: 0.1..10.0,
        orthographic_zoom_speed: 0.1,
    });

    commands.spawn(Camera2d);
}

pub fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    let projection = camera.into_inner();
    if let Projection::Orthographic(ref mut ortho) = projection.into_inner() {
        let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
        let multiplicative_zoom = 1. + delta_zoom;
        ortho.scale = (ortho.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );

    }

}