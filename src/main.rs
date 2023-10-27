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
use crate::mass_bodies::{MassBodyBundle, Mass, Radius, PassiveBody, UserControlled, State};
use bevy::input::mouse::MouseWheel;
use bevy::utils::hashbrown::HashMap;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (update_passivebody_states, move_user_states,
                              update_view, mouse_click, zoom_camera))
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
                mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                transform: Transform {
                    translation: Vec3::ZERO,
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: PassiveBody,
            state: State {
                position: Vec3::ZERO,
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO,
            },
        },
    ));
    let separation = 50.0;
    for i in (-1..2).step_by(2){
        for j in (-1..2).step_by(2) {
            let radius = if i == -1 && j == -1 {2.5} else {1.5};
            commands.spawn((
                MassBodyBundle {
                    mass: Mass(if i == -1 && j == -1 {1e12} else {1e11}),
                    meshbundle: MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                        material: materials.add(Color::rgb(0., 1., 0.).into()),
                        transform: Transform {
                            translation: Vec3::new(
                                (i as f32) * separation,
                                (j as f32) * separation,
                                0.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    radius: Radius(radius),
                    marker: PassiveBody,
                    state: State {
                        position: Vec3::new(
                            (i as f32) * separation,
                            (j as f32) * separation,
                            0.0,
                        ),
                        velocity: Vec3::ZERO,
                        acceleration: Vec3::ZERO,
                    },
                },
            ));
        }
    }

}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                debug!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                debug!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}

fn mouse_pos_logging(cursor: Res<CursorInfo>) {
    if let Some(position) = cursor.position() {
        debug!("Mouse position: {:?}", position);
    }
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

fn update_passivebody_states(
    time: Res<Time>,
    mut query: Query<(&mut State, &Mass, Entity), (With<PassiveBody>, Without<UserControlled>)>,
) {
    // Collect data into separate vectors
    let mut states: Vec<Mut<State>> = Vec::new();
    let mut masses: Vec<&Mass> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();

    for (mut state, mass, entity) in query.iter_mut() {
        states.push(state);
        masses.push(mass);
        entities.push(entity);
    }

    // Update accelerations
    for i in 0..states.len() {
        states[i].acceleration = Vec3::ZERO;
        for j in 0..states.len() {
            if i != j {
                let acceleration = utils::acceleration(states[j].position, masses[j].0, states[i].position);
                states[i].acceleration += acceleration;
            }
        }
    }

    // Update positions and velocities
    for i in 0..states.len() {
        let state = &mut *states[i];
        state.velocity += state.acceleration * time.delta_seconds();
        state.position += state.velocity * time.delta_seconds();
        debug!("Entity: {:?}, velocity: {:?}", entities[i], state.velocity);
    }
}

fn update_view(
    mut query: Query<(&mut Transform, &State)>,
) {
    for (mut transform, state) in query.iter_mut() {
        transform.translation = state.position;
    }
}


fn move_user_states(
    mut query: Query<(&mut State), (With<PassiveBody>, With<UserControlled>)>,
    cursor: Res<CursorInfo>,
) {
        if let Some(position) = cursor.position() {
            for mut state in query.iter_mut() {
                state.position = position.extend(0.0);
                state.velocity = Vec3::ZERO;
                state.acceleration = Vec3::ZERO;
            }
        }
}

fn mouse_click(
    mouse_botton_input: Res<Input<MouseButton>>,
    cursor: Res<CursorInfo>,
    mut commands: Commands,
    query: Query<(Entity,&Radius, &Transform), With<PassiveBody>>,
) {
    if let Some(position) = cursor.position() {
            if mouse_botton_input.just_pressed(MouseButton::Left) {
                for (entity, radius, transform) in query.iter() {
                    if (position - transform.translation.truncate()).length() < radius.0 {
                        commands.entity(entity)
                            .insert(UserControlled);
                    }
                }
            }

            if mouse_botton_input.just_released(MouseButton::Left) {
                for (entity, radius, transform) in query.iter() {
                        commands.entity(entity)
                            .remove::<UserControlled>();
                }
            }
    }
}