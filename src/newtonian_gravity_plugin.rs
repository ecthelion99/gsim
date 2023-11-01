use bevy::prelude::*;
use crate::mass_bodies::*;

#[derive(Component)]
pub struct GravityAcceleration(pub Vec3);

const G: f32 = 6.6734e-11;
const ETA: f32 = 1.0;

pub fn update_gravity(
    mut query: Query<(&mut GravityAcceleration, &Mass, &Transform), (With<Body>)>,
) {
    // Collect data into separate vectors
    // Update accelerations
    let mut queries: Vec<(Mut<GravityAcceleration>, &Mass, &Transform)> = Vec::new();
    {
        for (acceleration, mass, transform) in query.iter_mut() {
            queries.push((acceleration, mass, transform));
        }
    }

    for i in 0..queries.len() {
        let mut acceleration = Vec3::ZERO;
        for j in 0..queries.len() {
            if i != j {
                acceleration += calc_acceleration(
                    queries[j].2.translation, queries[j].1.0, queries[i].2.translation);
            }
            queries[i].0.0 = acceleration;
        }
    }
}

pub fn calc_acceleration(point: Vec3, mass: f32, position: Vec3) -> Vec3 {
    let distance = position - point;
    let magnitude = G*mass/(distance.length() + ETA).powi(2);
    -magnitude*distance.normalize()
}