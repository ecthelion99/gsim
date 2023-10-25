use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(MousePosition(None))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_free_bodies, move_user_bodies, cursor_position, mouse_click))
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

#[derive(Component)]
struct MainCamera;


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
    commands.spawn((Camera2dBundle::default(), MainCamera));

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

fn move_free_bodies(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity, &Mass), (With<PassiveBody>, Without<UserControlled>)>) {
    for (mut transform, velocity, mass) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn move_user_bodies(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), (With<PassiveBody>, With<UserControlled>)>,
    mouse_position: Res<MousePosition>,
) {
    match mouse_position.0 {
        Some(position) => {
            for (mut transform, mut velocity) in query.iter_mut() {
                velocity.0 = (position.extend(0.0) - transform.translation)/time.delta_seconds();
                transform.translation = position.extend(0.0)
            }
        }
        None => {}
    }
}

fn cursor_position(
    mut mycoords: ResMut<MousePosition>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();
    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = Some(world_position);
    }
}


fn mouse_click(
    mouse_botton_input: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut commands: Commands,
    query: Query<(Entity,&Radius, &Transform), With<PassiveBody>>,
) {
    if let Some(position) = mouse_position.0 {
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