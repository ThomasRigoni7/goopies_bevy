use bevy::input::common_conditions::*;
use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use bevy::render::camera::ScalingMode;
use std::ops::Range;

#[derive(Debug, Resource)]
pub struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub zoom_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            viewport_height: 1000.0,
            zoom_range: 0.1..10.0,
            zoom_speed: 0.1,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraSettings::default())
            .add_systems(Startup, startup)
            .add_systems(Update, zoom)
            .add_systems(Update, handle_drag.run_if(input_pressed(MouseButton::Left)))
            .insert_resource(CameraSettings::default());
    }
}

fn startup(mut commands: Commands, camera_settings: Res<CameraSettings>) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.viewport_height,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    let projection = camera.into_inner();
    if let Projection::Orthographic(ref mut ortho) = projection.into_inner() {
        let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.zoom_speed;
        let multiplicative_zoom = 1. + delta_zoom;
        ortho.scale = (ortho.scale * multiplicative_zoom).clamp(
            camera_settings.zoom_range.start,
            camera_settings.zoom_range.end,
        );
    }
}

fn handle_drag(
    camera: Single<(&mut Transform, &Projection), With<Camera>>,
    window: Single<&Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut last_mouse_position: Local<Vec2>,
) {
    let window = window.into_inner();
    let (mut camera_transform, camera_projection) = camera.into_inner();

    if let Some(current_mouse_position) = window.cursor_position() {
        if let Projection::Orthographic(ref ortho) = camera_projection {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                *last_mouse_position = current_mouse_position; // Reset on new drag
                return;
            }
            let delta = current_mouse_position - *last_mouse_position;
            camera_transform.translation -= Vec3::new(delta.x, -delta.y, 0.0) * ortho.scale;
            *last_mouse_position = current_mouse_position;
        }
    }
}
