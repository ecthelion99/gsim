use bevy::prelude::*;
use bevy_cursor::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};

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
pub struct BodyState {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Bundle)]
pub struct MassBodyBundle<M: Material2d> {
    pub mass: Mass,
    pub radius: Radius,
    pub marker: Body,
    pub meshbundle: MaterialMesh2dBundle<M>,
    pub state: BodyState,
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
pub fn set_velocity(
    mut ev_set_velocity: EventReader<SetVelocityEvent>,
    cursor: Res<CursorInfo>,
    mut query: Query<(&mut BodyState, &Transform), With<Body>>,
) {
    if let Some(position) = cursor.position() {
        for ev in ev_set_velocity.iter() {
            if let Ok((mut state, transform)) = query.get_mut(ev.e) {
                state.velocity = (position.extend(0.0) - transform.translation)*VELOCITY_SCALE;
                debug!("velocity of entity {:?} set to: {:?}", ev.e, state.velocity);
            }
        }
    }
}