use bevy::prelude::*;
use bevy_cursor::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
use crate::newtonian_gravity_plugin::{GravityAcceleration};

#[derive(Component)]
pub struct Body;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct UserControlled;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Arrow;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Trail {
    pub points: Vec<Vec2>,
}

impl Trail {
    pub fn new(n: usize) -> Self {
        Trail {
            points: Vec::with_capacity(n),
        }
    }
}

#[derive(Bundle)]
pub struct MassBodyBundle<M: Material2d> {
    pub mass: Mass,
    pub radius: Radius,
    pub marker: Body,
    pub meshbundle: MaterialMesh2dBundle<M>,
    pub velocity: Velocity,
    pub acceleration: GravityAcceleration,
}



pub fn mouse_click(
    mouse_botton_input: Res<Input<MouseButton>>,
    cursor: Res<CursorInfo>,
    mut commands: Commands,
    query: Query<(Entity,&Radius, &Transform), (With<Body>, Without<Halted>)>,
    q_halted: Query<(Entity, (With<Halted>))>,
    mut ev_set_velocity: EventWriter<SetVelocityEvent>,
) {
    if let Some(position) = cursor.position() {

        if mouse_botton_input.just_pressed(MouseButton::Left) {
            for (entity, radius, transform) in query.iter()
                .filter(|(_, radius, transform)| (position - transform.translation.truncate()).length() < radius.0){
                    commands.entity(entity)
                        .insert(UserControlled);
            }
        }

        if mouse_botton_input.just_pressed(MouseButton::Right) {
            for (entity, radius, transform) in query.iter()
                .filter(|(_, radius, transform)| (position - transform.translation.truncate()).length() < radius.0){
                commands.entity(entity)
                    .insert(Halted);
            }
        }

        if mouse_botton_input.just_released(MouseButton::Right) {
            for (entity, _) in q_halted.iter() {
                commands.entity(entity)
                    .remove::<Halted>();
                ev_set_velocity.send(SetVelocityEvent { e: entity });
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

#[derive(Event)]
pub struct SetVelocityEvent{
    pub e: Entity,
}

#[derive(Component)]
pub struct Halted;

const VELOCITY_SCALE: f32 = 0.1;
pub fn user_set_velocity(
    mut ev_set_velocity: EventReader<SetVelocityEvent>,
    cursor: Res<CursorInfo>,
    mut query: Query<(&mut Velocity, &Transform), With<Body>>,
) {
    if let Some(position) = cursor.position() {
        for ev in ev_set_velocity.iter() {
            if let Ok((mut velocity, transform)) = query.get_mut(ev.e) {
                velocity.0 = (position.extend(0.0) - transform.translation)*VELOCITY_SCALE;
            }
        }
    }
}

pub fn update_bodies_euler(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, & GravityAcceleration, &Mass, &mut Transform), (With<Body>, Without<UserControlled>, Without<Halted>)>,
) {
    for (mut velocity, acceleration, mass, mut transform) in query.iter_mut() {
        velocity.0 += acceleration.0 * time.delta_seconds();
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub fn update_trails(
    mut query: Query<(&mut Trail, &Transform), (With<Body>, Without<UserControlled>, Without<Halted>)>,
) {
    for (mut trail, transform) in query.iter_mut() {
        if trail.points.len() == trail.points.capacity() {
            trail.points.remove(0);
        }
        trail.points.push(transform.translation.truncate());
    }
}

pub fn draw_trails(
    mut gizmos: Gizmos,
    materials: Res<Assets<ColorMaterial>>,
    query: Query<(&Trail, &Handle<ColorMaterial>), With<Body>>) {
    for (trail, color_handle) in query.iter() {
        for window in trail.points.windows(2) {
            let color =  match materials.get(color_handle) {
                Some(color_material) => color_material.color,
                None => Color::WHITE,
            };
            gizmos.line_2d(window[0], window[1], color);
        }
    }
}

pub fn move_user_controlled_bodies(
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