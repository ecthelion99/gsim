use bevy::prelude::*;
use bevy::input::keyboard::{KeyboardInput, KeyCode};
#[derive(Component)]
pub struct MainCamera;

pub fn camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut q: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
) {
    let (mut projection, mut transform) = q.single_mut();

    use bevy::input::ButtonState;

    const ZOOM_SPEED: f32 = 0.05;
    const MOVE_SPEED: f32 = 5.0;

    for ev in key_evr.iter() {
        match (ev.state, ev.key_code) {
            (ButtonState::Pressed, Some(KeyCode::PageUp)) => {
                projection.scale *= (1.0 + ZOOM_SPEED);
            },
            (ButtonState::Pressed, Some(KeyCode::PageDown)) => {
                projection.scale *= (1.0 - ZOOM_SPEED);
            },
            (ButtonState::Pressed, Some(KeyCode::Up)) => {
                transform.translation.y += MOVE_SPEED;
            },
            (ButtonState::Pressed, Some(KeyCode::Down)) => {
                transform.translation.y -= MOVE_SPEED;
            },
            (ButtonState::Pressed, Some(KeyCode::Left)) => {
                transform.translation.x -= MOVE_SPEED;
            },
            (ButtonState::Pressed, Some(KeyCode::Right)) => {
                transform.translation.x += MOVE_SPEED;
            },
            _ =>{}

        }
    }

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.01, 5.0);
}