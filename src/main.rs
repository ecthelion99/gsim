use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_cursor::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CursorInfoPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_free_bodies, move_user_bodies, mouse_click))
        .run();
}

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct PassiveBody;

#[derive(Component)]
struct Mass(f32);

#[derive(Component)]
struct UserControlled;

#[derive(Component)]
struct Radius(f32);

#[derive(Resource)]
struct MousePosition(Option<Vec2>);

#[derive(Bundle)]
struct MassBodyBundle<M: Material2d> {
    mass: Mass,
    velocity: Velocity,
    radius: Radius,
    marker: PassiveBody,
    meshbundle: MaterialMesh2dBundle<M>,
}



fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    //Circle
    commands.spawn((
        MassBodyBundle {
            mass: Mass(1.0),
            radius: Radius(25.),
            velocity: Velocity(Vec3::new(25., 0.0, 0.0)),
            meshbundle: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(25.).into()).into(),
                material: materials.add(Color::rgb(1.,69./255.,0.).into()),
                ..Default::default()
            },
            marker: PassiveBody,
        },

    ));
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