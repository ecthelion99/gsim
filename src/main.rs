pub mod utils;
mod acceleration_field_plugin;
mod mass_bodies;
mod constants;
mod newtonian_gravity_plugin;

use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::log::LogPlugin;
use bevy::math::IRect;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;
use crate::acceleration_field_plugin::AccelerationFieldPlugin;
use crate::mass_bodies::*;
use crate::newtonian_gravity_plugin::{GravityAcceleration, update_gravity};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }), CursorInfoPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(AccelerationFieldPlugin{
        //     field_resolution: Vec2::new(5., 5.),
        //     field_size: IRect::new(-64, -36, 65, 37),
        //     arrow_scale: 0.5,
        // })
        .add_event::<SetVelocityEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update,
                     (
                         update_gravity.before(update_bodies_euler),
                         update_bodies_euler, move_user_states,
                              mouse_click, zoom_camera, user_set_velocity))
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
                    translation: Vec3::new(-150., 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, 16.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
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
                    translation: Vec3::new(-100.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, -30.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
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
                    translation: Vec3::new(-400.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, -10.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
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
                    translation: Vec3::new(-200.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, -15.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
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
                    translation: Vec3::new(-250.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, -10.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
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
                    translation: Vec3::new(-80., 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::new(0.0, 50.0, 0.0)),
            acceleration: GravityAcceleration(Vec3::ZERO),
        },
    ));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(5e15),
            radius: Radius(10.),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(Color::rgb(20., 69. / 255., 0.).into()),
                transform: Transform {
                    translation: Vec3::new(20., 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            marker: Body,
            velocity: Velocity(Vec3::ZERO),
            acceleration: GravityAcceleration(Vec3::ZERO),
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

fn move_user_states(
    mut query: Query<(&mut Velocity, &mut Transform), With<UserControlled>>,
    cursor: Res<CursorInfo>,
) {
        if let Some(position) = cursor.position() {
            for (mut velocity,mut transform) in query.iter_mut() {
                transform.translation = position.extend(0.0);
                velocity.0 = Vec3::ZERO;
            }
        }
}