use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::geometry::{ColliderSet, Ray};
use bevy_rapier2d::rapier::pipeline::QueryFilter;
use bevy_rapier2d::rapier::pipeline::QueryPipeline;
use nalgebra::point;
use nalgebra::vector;

pub struct SensorRay {
    pub orientation: f32,
    pub length: f32,
}

#[derive(Component)]
pub struct Robot {
    pub position: Vec2,
    pub orientation: f32,
    pub trajectory: Vec<Vec2>,
    pub sensor: Vec<SensorRay>,
    pub sensor_max_length: f32,
}

#[derive(Component)]
pub struct World {
    pipeline: QueryPipeline,
    pub colliders: ColliderSet,
}

pub struct GymCorePlugin;

impl Plugin for GymCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_world, setup_robot))
            .add_systems(Update, (update_physics, update_robot));
    }
}

fn setup_world(mut commands: Commands) {
    let collider = ColliderBuilder::cuboid(500.0, 50.0)
        .translation(vector![0.0, -200.0])
        .build();

    let mut collider_set = ColliderSet::new();
    collider_set.insert(collider);

    let rigid_body_set = RigidBodySet::new();
    let mut pipeline = QueryPipeline::new();
    pipeline.update(&rigid_body_set, &collider_set);

    commands.spawn(World {
        pipeline,
        colliders: collider_set,
    });
}

fn setup_robot(mut commands: Commands) {
    let length = 200.0;
    let ray_count = 10;
    let fov = 45.0 * (PI / 180.0);

    let mut sensor = Vec::new();
    for i in 0..ray_count {
        let orientation = (i as f32 / ray_count as f32) * (fov * 2.0) - fov;
        sensor.push(SensorRay {
            orientation,
            length,
        });
    }

    commands.spawn(Robot {
        position: Vec2::new(0.0, 0.0),
        orientation: 0.0,
        trajectory: Vec::new(),
        sensor_max_length: length,
        sensor,
    });
}

fn update_physics(mut robots: Query<&mut Robot>, rapier_handlers: Query<&mut World>) {
    let mut robot = robots.single_mut();
    let rapier_handler = rapier_handlers.single();
    let filter = QueryFilter::default();
    let bodies = RigidBodySet::new();

    let position = robot.position;
    let robot_orientation = robot.orientation;
    let max_toi = robot.sensor_max_length;
    let sensors = &mut robot.sensor;

    for sensor in sensors.iter_mut() {
        let ray = Ray::new(
            point![position.x, position.y],
            vector![
                (sensor.orientation + robot_orientation).cos(),
                (sensor.orientation + robot_orientation).sin()
            ],
        );

        if let Some((_, toi)) = rapier_handler.pipeline.cast_ray(
            &bodies,
            &rapier_handler.colliders,
            &ray,
            max_toi,
            false,
            filter,
        ) {
            sensor.length = toi;
        } else {
            sensor.length = max_toi;
        }
    }
}

fn update_robot(time: Res<Time>, mut robots: Query<&mut Robot>) {
    let mut robot = robots.single_mut();

    if let Some(destination) = robot.trajectory.first() {
        let delta_x = destination.x - robot.position.x;
        let delta_y = destination.y - robot.position.y;

        let distance = (delta_x.powi(2) + delta_y.powi(2)).sqrt();
        let speed = 50.0;

        let delta = speed * time.delta_seconds();
        let angle = delta_y.atan2(delta_x);

        let new_position_x;
        let new_position_y;

        if distance > delta {
            new_position_x = robot.position.x + delta * angle.cos();
            new_position_y = robot.position.y + delta * angle.sin();
        } else {
            new_position_x = destination.x;
            new_position_y = destination.y;
            robot.trajectory.remove(0);
        }

        robot.position.x = new_position_x;
        robot.position.y = new_position_y;
        robot.orientation = angle;
    }
}
