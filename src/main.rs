pub mod arrows;
mod acceleration_field_plugin;
mod mass_bodies;

use bevy::input::keyboard::{KeyboardInput, KeyCode};
use bevy::log::LogPlugin;
use bevy::math::IRect;
use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;
use crate::acceleration_field_plugin::AccelerationFieldPlugin;
use crate::mass_bodies::{MassBodyBundle, Mass, Radius, Velocity, PassiveBody, UserControlled};
use bevy::input::mouse::MouseWheel;

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
        .add_systems(Update, (move_free_bodies, move_user_bodies, mouse_click,
                              zoom_camera, mouse_pos_logging))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let masses = vec![1e20, 1.2e20, 1.5e20, 2e20];
    let radius = vec![1.0, 1.2, 1.5, 2.0];
    for i in -2..2 {
        commands.spawn((
            MassBodyBundle {
                mass: Mass(masses[(i+2) as usize]),
                radius: Radius(radius[(i+2) as usize]),
                velocity: Velocity(Vec3::ZERO),
                meshbundle: MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius[(i+2) as usize]).into()).into(),
                    material: materials.add(Color::rgb(1., 69. / 255., 0.).into()),
                    transform: Transform {
                        translation: Vec3::new((i as f32)*20.0, 0., 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                marker: PassiveBody,
            },
        ));
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
                        commands.entity(entity)
                            .remove::<UserControlled>();
                }
            }
    }
}