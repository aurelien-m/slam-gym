use bevy::prelude::*;
use bevy_rapier2d::rapier::dynamics::RigidBodySet;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use bevy_rapier2d::rapier::geometry::{ColliderSet, Ray};
use bevy_rapier2d::rapier::pipeline::QueryFilter;
use bevy_rapier2d::rapier::pipeline::QueryPipeline;
use nalgebra::point;
use nalgebra::vector;

#[derive(Component)]
pub struct Robot {
    pub position: Vec2,
    pub orientation: f32,
    pub trajectory: Vec<Vec2>,
}

#[derive(Component)]
pub struct World {
    pipeline: QueryPipeline,
    pub collider_set: ColliderSet,
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
        collider_set,
    });
}

fn setup_robot(mut commands: Commands) {
    commands.spawn(Robot {
        position: Vec2::new(0.0, 0.0),
        orientation: 0.0,
        trajectory: Vec::new(),
    });
}

fn update_physics(robots: Query<&mut Robot>, mut rapier_handlers: Query<&mut World>) {
    let robot = robots.single();
    let x = robot.position.x;
    let y = robot.position.y;

    let rapier_handler = rapier_handlers.iter_mut().next().unwrap();
    let ray = Ray::new(point![x, y], vector![0.0, -1.0]);
    let filter = QueryFilter::default();
    let bodies = RigidBodySet::new();

    if let Some((_handle, toi)) = rapier_handler.pipeline.cast_ray(
        &bodies,
        &rapier_handler.collider_set,
        &ray,
        250.0,
        false,
        filter,
    ) {
        let _hit_point = ray.point_at(toi);
    }
}

fn update_robot(time: Res<Time>, mut robots: Query<&mut Robot>) {
    let mut robot = robots.single_mut();
    // robot.position.y -= time.delta_seconds() * 10.0;
    robot.orientation += time.delta_seconds() * 1.0;
}
