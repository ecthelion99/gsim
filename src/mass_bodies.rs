use bevy::prelude::*;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};

#[derive(Component)]
pub struct Velocity( pub Vec3);

#[derive(Component)]
pub struct PassiveBody;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct UserControlled;

#[derive(Component)]
pub struct Radius(pub f32);

#[derive(Component)]
pub struct Arrow;

#[derive(Bundle)]
pub struct MassBodyBundle<M: Material2d> {
    pub mass: Mass,
    pub velocity: Velocity,
    pub radius: Radius,
    pub marker: PassiveBody,
    pub meshbundle: MaterialMesh2dBundle<M>,
}