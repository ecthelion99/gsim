pub mod utils;
mod acceleration_field_plugin;
mod mass_bodies;
mod constants;

use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::log::LogPlugin;
use bevy::math::IRect;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;
use crate::acceleration_field_plugin::AccelerationFieldPlugin;
use crate::mass_bodies::*;
use bevy::input::mouse::MouseWheel;
use bevy::utils::hashbrown::HashMap;
use rand::random;
use crate::constants::SOFTENING_DISTANCE;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }), CursorInfoPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AccelerationFieldPlugin{
            field_resolution: Vec2::new(2.0, 2.0),
            field_size: IRect::new(-64, -36, 65, 37)
        })
        .add_event::<SetVelocityEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, (update_body_states, move_user_states,
                              mouse_click, zoom_camera, set_velocity))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.25;
    commands.spawn((camera, MainCamera));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                transform: Transform {
                    translation: Vec3::new(85.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            state: BodyState {
                velocity: Vec3::new(0.0, 5.0, 0.0),
                acceleration: Vec3::ZERO,
            },
        },
    ));
    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                transform: Transform {
                    translation: Vec3::new(115.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            state: BodyState {
                velocity: Vec3::new(0.0, 3., 0.0),
                acceleration: Vec3::ZERO,
            },
        },
    ));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e14),
            radius: Radius(10.),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                transform: Transform {
                    translation: Vec3::new(-50., 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            state: BodyState {
                velocity: Vec3::new(0.0, 0.0, 0.0),
                acceleration: Vec3::ZERO,
            },
        },
    ));
}


fn zoom_camera(
    mut key_evr: EventReader<KeyboardInput>,
    mut q: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let mut projection = q.single_mut();

    use bevy::input::ButtonState;

    const ZOOM_SPEED: f32 = 0.05;

    for ev in key_evr.iter() {
        match (ev.state, ev.key_code) {
            (ButtonState::Pressed, Some(KeyCode::Up)) => {
                projection.scale *= (1.0 - ZOOM_SPEED);
            },
            (ButtonState::Pressed, Some(KeyCode::Down)) => {
                projection.scale *= (1.0 + ZOOM_SPEED);
            },
            _ => {}
        }
    }

    // always ensure you end up with sane values
    // (pick an upper and lower bound for your application)
    projection.scale = projection.scale.clamp(0.01, 5.0);
}

fn update_body_states(
    time: Res<Time>,
    mut query: Query<(&mut BodyState, &Mass, &mut Transform, Entity), (With<Body>, Without<UserControlled>, Without<Halted>)>,
) {
    // Collect data into separate vectors
    let mut states: Vec<Mut<BodyState>> = Vec::new();
    let mut masses: Vec<&Mass> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();
    let mut transforms: Vec<Mut<Transform>> = Vec::new();

    for (mut state, mass, transform, entity) in query.iter_mut() {
        states.push(state);
        masses.push(mass);
        entities.push(entity);
        transforms.push(transform);
    }

    // Update accelerations
    for i in 0..states.len() {
        states[i].acceleration = Vec3::ZERO;
        for j in 0..states.len() {
            if i != j {
                if (transforms[i].translation - transforms[j].translation).length() <  SOFTENING_DISTANCE{
                    continue;
                }
                let acceleration = utils::acceleration(transforms[j].translation, masses[j].0, transforms[i].translation);
                states[i].acceleration += acceleration;
            }
        }
    }

    // Update positions and velocities
    for i in 0..states.len() {
        let state = &mut *states[i];
        state.velocity += state.acceleration * time.delta_seconds();
        transforms[i].translation += state.velocity * time.delta_seconds();
    }
}


fn move_user_states(
    mut query: Query<(&mut BodyState, &mut Transform), With<UserControlled>>,
    cursor: Res<CursorInfo>,
) {
        if let Some(position) = cursor.position() {
            for (mut state,mut transform) in query.iter_mut() {
                transform.translation = position.extend(0.0);
                state.velocity = Vec3::ZERO;
                state.acceleration = Vec3::ZERO;
            }
        }
}