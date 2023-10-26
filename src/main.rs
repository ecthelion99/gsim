pub mod arrows;
mod acceleration_field_plugin;
mod mass_bodies;

use bevy::log::LogPlugin;
use bevy::math::IRect;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;
use crate::acceleration_field_plugin::AccelerationFieldPlugin;
use crate::mass_bodies::{MassBodyBundle, Mass, Radius, Velocity, PassiveBody, UserControlled};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }), CursorInfoPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AccelerationFieldPlugin{
            field_resolution: Vec2::new(25.0, 25.0),
            field_size: IRect::new(-24, -14, 25, 15)
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_free_bodies, move_user_bodies, mouse_click))
        .run();
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let masses = vec![5.0, 10.0, 4.0, 25.0];
    for i in -2..2 {
        commands.spawn((
            MassBodyBundle {
                mass: Mass(masses[(i+2) as usize]),
                radius: Radius(masses[(i+2) as usize]),
                velocity: Velocity(Vec3::ZERO),
                meshbundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(masses[(i+2) as usize]).into()).into(),
                    material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                    transform: Transform {
                        translation: Vec3::new((i as f32)*100., 0., 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                marker: PassiveBody,
            },
        ));
    }

}

fn move_free_bodies(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), (With<PassiveBody>, Without<UserControlled>)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn move_user_bodies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<PassiveBody>, With<UserControlled>)>,
    cursor: Res<CursorInfo>,
) {
        if let Some(position) = cursor.position() {
            for (mut transform, mut velocity) in query.iter_mut() {
                velocity.0 = (position.extend(0.0) - transform.translation)/time.delta_seconds();
                transform.translation = position.extend(0.0)
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
                    if (position - transform.translation.truncate()).length() < radius.0 {
                        commands.entity(entity)
                            .remove::<UserControlled>();
                    }
                }
            }
    }
}