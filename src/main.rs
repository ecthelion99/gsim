pub mod utils;
mod acceleration_field_plugin;
mod mass_bodies;
mod constants;
mod newtonian_gravity_plugin;
mod camera;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;
use crate::mass_bodies::*;
use crate::newtonian_gravity_plugin::{GravityAcceleration, update_gravity};
use crate::camera::{MainCamera, camera};

const TRAIL_LENGTH: usize = 1800;
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
                         update_bodies_euler,
                         update_trails.before(draw_trails),
                         draw_trails,
                         move_user_controlled_bodies,
                              mouse_click, camera, user_set_velocity))
        .run();
}

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
                material: materials.add(Color::YELLOW_GREEN.into()),
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
        Trail::new(600),
    ));
    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::PURPLE.into()),
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
        Trail::new(TRAIL_LENGTH),
    ));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::YELLOW.into()),
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
        Trail::new(TRAIL_LENGTH),
    ));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::GREEN.into()),
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
        Trail::new(TRAIL_LENGTH),
    ));

    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::MAROON.into()),
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
        Trail::new(TRAIL_LENGTH),
    ));


    commands.spawn((
        MassBodyBundle {
            mass: Mass(1e13),
            radius: Radius(2.5),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(Color::TURQUOISE.into()),
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
        Trail::new(TRAIL_LENGTH),
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
        Trail::new(TRAIL_LENGTH),
    ));
}


