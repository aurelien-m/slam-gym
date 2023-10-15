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
    pub velocity: f32,
    pub angular_velocity: f32,
    pub sensor: Vec<SensorRay>,
    pub sensor_max_length: f32,
}

#[derive(Component)]
pub struct Trajectory(pub Vec<Vec2>);

const ACCELERATION: f32 = 150.0;
const MAX_VELOCITY: f32 = 200.0;
const ANGULAR_ACCELERATION: f32 = PI / 4.0;
const MAX_ANGULAR_VELOCITY: f32 = PI / 2.0;

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
        velocity: 0.0,
        angular_velocity: 0.0,
        sensor_max_length: length,
        sensor,
    });

    commands.spawn(Trajectory(Vec::new()));
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

fn update_orientation(robot: &mut Robot, destination: &Vec2, dt: f32) -> bool {
    let position = robot.position;
    let delta_x = destination.x - position.x;
    let delta_y = destination.y - position.y;
    let target_angle = delta_y.atan2(delta_x);

    let breaking_angle = (robot.angular_velocity.powi(2) / (2.0 * ANGULAR_ACCELERATION)).abs();
    let angle_delta = (target_angle - robot.orientation).abs();

    robot.angular_velocity = if angle_delta < breaking_angle {
        robot.angular_velocity - ANGULAR_ACCELERATION * dt
    } else if robot.angular_velocity < MAX_ANGULAR_VELOCITY {
        robot.angular_velocity + ANGULAR_ACCELERATION * dt
    } else {
        MAX_ANGULAR_VELOCITY
    };

    let travelled_angle = robot.angular_velocity * dt;

    if angle_delta > travelled_angle {
        let new_orientation;

        if (robot.orientation < target_angle && target_angle - robot.orientation <= PI)
            || (robot.orientation > target_angle && robot.orientation - target_angle > PI)
        {
            new_orientation = robot.orientation + travelled_angle;
        } else {
            new_orientation = robot.orientation - travelled_angle;
        }

        let new_orientation = if new_orientation > PI {
            new_orientation - 2.0 * PI
        } else if new_orientation < -PI {
            new_orientation + 2.0 * PI
        } else {
            new_orientation
        };

        robot.orientation = new_orientation;
        return false; // Orientation not reached
    }

    robot.orientation = target_angle;
    robot.angular_velocity = 0.0;
    true // Orientation reached
}

fn update_position(robot: &mut Robot, destination: &Vec2, dt: f32) -> bool {
    let position = robot.position;
    let delta_x = destination.x - position.x;
    let delta_y = destination.y - position.y;
    let target_angle = delta_y.atan2(delta_x);

    let stopping_distance = (robot.velocity.powi(2) / (2.0 * ACCELERATION)).abs();
    let distance = ((destination.x - robot.position.x).powi(2)
        + (destination.y - robot.position.y).powi(2))
    .sqrt();

    robot.velocity = if stopping_distance > distance {
        robot.velocity - ACCELERATION * dt
    } else if robot.velocity < MAX_VELOCITY {
        robot.velocity + ACCELERATION * dt
    } else {
        MAX_VELOCITY
    };

    let travelled_distance = robot.velocity * dt;

    if distance > travelled_distance {
        robot.position.x = robot.position.x + travelled_distance * target_angle.cos();
        robot.position.y = robot.position.y + travelled_distance * target_angle.sin();
    } else {
        robot.position.x = destination.x;
        robot.position.y = destination.y;
        robot.velocity = 0.0;
        return true;
    }

    return false;
}

fn update_robot(
    time: Res<Time>,
    mut robots: Query<&mut Robot>,
    mut trajectory: Query<&mut Trajectory>,
) {
    let mut robot = robots.single_mut();
    let mut trajectory = trajectory.single_mut();

    if let Some(destination) = trajectory.0.first() {
        if !update_orientation(&mut robot, destination, time.delta_seconds()) {
            return; // Orientation not reached
        }

        if update_position(&mut robot, destination, time.delta_seconds()) {
            trajectory.0.remove(0);
        }
    }
}
