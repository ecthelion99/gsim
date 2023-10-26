use std::cmp::max;
use bevy::math::IRect;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::arrows::arrow;
use crate::mass_bodies::*;

pub const G: f32 = 6.6734e-11;
pub struct AccelerationFieldPlugin {
    pub(crate) field_resolution: Vec2,
    pub(crate) field_size: IRect,
}

impl Plugin for AccelerationFieldPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AccelerationFieldInfo {
            field_resolution: self.field_resolution,
            field_size: self.field_size,
            max: 0.0,
            min: f32::INFINITY,
            })
            .add_systems(Startup, setup)
            .add_systems(Update, (update_acceleration_field,
                                  update_acceleration_field_arrows));
    }
}

#[derive(Resource)]
struct AccelerationFieldInfo {
    pub field_resolution: Vec2,
    pub field_size: IRect,
    pub max : f32,
    pub min : f32,
}

#[derive(Component)]
struct AccelerationFieldPoint {
    pub position: Vec3,
    pub acceleration: Vec3,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    acceleration_field_info: ResMut<AccelerationFieldInfo>,
) {
        debug!("Setting up acceleration field");
        for i in acceleration_field_info.field_size.min.x ..
            acceleration_field_info.field_size.max.x {
            for j in acceleration_field_info.field_size.min.y ..
                acceleration_field_info.field_size.max.y {
                let position = Vec3::new((i as f32)*acceleration_field_info.field_resolution.x,
                                         (j as f32)*acceleration_field_info.field_resolution.y, -1.);
                commands.spawn((
                    MaterialMesh2dBundle{
                        mesh: meshes.add(arrow(0.5, 0.1, 0.3).into()).into(),
                        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                        transform: Transform {
                            translation: position,
                            scale: acceleration_field_info.field_resolution.extend(0.0)/2.,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    AccelerationFieldPoint {
                        position,
                        acceleration: Vec3::new(0.0, 0.0, 0.0),
                    },
                ));
            }
        }
}

fn update_acceleration_field(
    mut field_info: ResMut<AccelerationFieldInfo>,
    mut query: Query<&mut AccelerationFieldPoint>,
    mass_query: Query<(&Mass, &Transform, &Radius)>,
) {
    let mut max_acceleration = f32::NEG_INFINITY;
    let mut min_acceleration = f32::INFINITY;
    for mut field_point in query.iter_mut() {
        let mut total_acceleration = Vec3::ZERO;
        for (mass, transform, radius) in mass_query.iter() {
            if (transform.translation - field_point.position).length() >  radius.0{
                total_acceleration += acceleration(transform.translation, mass.0, field_point.position);
            }
        }
        max_acceleration = max_acceleration.max(total_acceleration.length());
        min_acceleration = min_acceleration.min(total_acceleration.length());
        field_point.acceleration = total_acceleration;
    }
    // bevy::log::debug!("Max acceleration: {}", field_info.max);
    // bevy::log::debug!("Min acceleration: {}", field_info.min);
    field_info.max = max_acceleration;
    field_info.min = min_acceleration;

}

fn acceleration(point: Vec3, mass: f32, position: Vec3) -> Vec3 {
    let distance = position - point;
    let magnitude = G*mass/(distance.length_squared());
    -magnitude*distance.normalize()
}

fn update_acceleration_field_arrows(
    field_info: Res<AccelerationFieldInfo>,
    mut query: Query<(&mut Transform, &AccelerationFieldPoint)>,
) {

    for (mut transform, field_point) in query.iter_mut() {
        let mut scale = (field_point.acceleration.length() - field_info.min)/(field_info.max - field_info.min)*field_info.field_resolution.x;
        transform.scale = Vec3::from_array([scale, scale, 0.]);
        transform.rotation = Quat::from_rotation_z(field_point.acceleration.y.atan2(field_point.acceleration.x));
    }
}