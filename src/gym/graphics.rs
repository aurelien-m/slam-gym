use super::core;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::rapier::geometry::TypedShape::Cuboid as RapierCuboid;

pub struct GymGraphicsPlugin;

const DARK_BLUE: Color = Color::rgb(0.15, 0.27, 0.33);
const LIGHT_BLUE: Color = Color::rgb(0.16, 0.62, 0.56);
const LIGHT_ORANGE: Color = Color::rgb(0.91, 0.77, 0.42);
const MEDIUM_ORANGE: Color = Color::rgb(0.96, 0.64, 0.38);
const DARK_ORANGE: Color = Color::rgb(0.91, 0.44, 0.32);

const BOTTOM_LEVEL: f32 = 0.0;
const MEDIUM_LEVEL: f32 = 1.0;
const TOP_LEVEL: f32 = 2.0;

#[derive(Component)]
struct Robot;

#[derive(Component)]
struct Sensor;

impl Plugin for GymGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(DARK_BLUE))
            .add_systems(PostStartup, (setup_world, setup_robot))
            .add_systems(Update, (draw_robot, draw_sensors));
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Query<&mut core::World>,
) {
    let world = world.single();
    for (_handle, collider) in world.colliders.iter() {
        match collider.shape().as_typed_shape() {
            RapierCuboid(cuboid) => {
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            shape::Quad::new(Vec2::new(
                                cuboid.half_extents.x * 2.0,
                                cuboid.half_extents.y * 2.0,
                            ))
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(DARK_ORANGE)),
                    transform: Transform::from_translation(Vec3::new(
                        collider.position().translation.x,
                        collider.position().translation.y,
                        BOTTOM_LEVEL,
                    )),
                    ..default()
                });
            }
            _ => {}
        }
    }
}

fn setup_robot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    robot: Query<&mut core::Robot>,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0., 0., 0.],
            [1., 0.5, 0.],
            [0., 1., 0.],
            [-1., 1., 0.],
            [-1., -1., 0.],
            [0., -1., 0.],
            [1., -0.5, 0.],
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1,
    ])));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(LIGHT_ORANGE)),
            transform: Transform::from_translation(Vec3::new(0., 0., TOP_LEVEL))
                .with_scale(Vec3::splat(16.)),
            ..Default::default()
        },
        Robot,
    ));

    let robot = robot.single();
    for sensor in robot.sensor.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                material: materials.add(ColorMaterial::from(MEDIUM_ORANGE)),
                transform: Transform::from_translation(Vec3::new(
                    sensor.orientation.cos() * sensor.length,
                    sensor.orientation.sin() * sensor.length,
                    TOP_LEVEL,
                )),
                ..default()
            },
            Sensor,
        ));
    }
}

fn draw_robot(robots: Query<&mut core::Robot>, mut transform: Query<&mut Transform, With<Robot>>) {
    let robot = robots.single();
    let transform = &mut transform.single_mut();
    transform.translation.x = robot.position.x;
    transform.translation.y = robot.position.y;
    transform.rotation = Quat::from_rotation_z(robot.orientation);
}

fn draw_sensors(robot: Query<&mut core::Robot>, mut query: Query<&mut Transform, With<Sensor>>) {
    let robot = robot.single();
    for i in 0..robot.sensor.len() {
        if let Some(sensor) = &mut query.iter_mut().nth(i) {
            let length = robot.sensor[i].length;
            let orientation = robot.sensor[i].orientation + robot.orientation;
            sensor.translation.x = robot.position.x + length * orientation.cos();
            sensor.translation.y = robot.position.y + length * orientation.sin();
        }
    }
}

pub fn draw_line(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    point_a: Vec2,
    point_b: Vec2,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [point_a.x, point_a.y, MEDIUM_LEVEL],
            [point_b.x, point_b.y, MEDIUM_LEVEL],
        ],
    );
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        material: materials.add(ColorMaterial::from(LIGHT_BLUE)),
        ..Default::default()
    });
    draw_point(commands, meshes, materials, point_b);
}

pub fn draw_point(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    point: Vec2,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(LIGHT_BLUE)),
        transform: Transform::from_translation(Vec3::new(point.x, point.y, MEDIUM_LEVEL)),
        ..default()
    });
}

pub fn draw_square(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    point: Vec2,
    size: f32,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(size, size)).into())
            .into(),
        material: materials.add(ColorMaterial::from(DARK_ORANGE)),
        transform: Transform::from_translation(Vec3::new(point.x, point.y, MEDIUM_LEVEL)),
        ..default()
    });
}
