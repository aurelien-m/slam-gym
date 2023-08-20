use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::rapier::geometry::TypedShape::Cuboid as RapierCuboid;

use super::core;

pub struct GymGraphicsPlugin;

#[derive(Component)]
struct Robot;

impl Plugin for GymGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_graphics)
            .add_systems(PostStartup, setup_world)
            .add_systems(Update, draw_robot);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world: Query<&mut core::World>,
) {
    let world = world.single();
    for (_handle, collider) in world.collider_set.iter() {
        match collider.shape().as_typed_shape() {
            RapierCuboid(cuboid) => {
                commands.spawn(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(
                            shape::Quad::new(Vec2::new(
                                cuboid.half_extents.x,
                                cuboid.half_extents.y,
                            ))
                            .into(),
                        )
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
                    transform: Transform::from_translation(Vec3::new(
                        collider.position().translation.x,
                        collider.position().translation.y,
                        0.,
                    )),
                    ..default()
                });
            }
            _ => {}
        }
    }
}

fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[0., 0.761, 1., 1.]; 7]);
    mesh.set_indices(Some(Indices::U32(vec![
        0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5, 0, 5, 6, 0, 6, 1,
    ])));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::splat(16.)),
            ..Default::default()
        },
        Robot,
    ));
}

fn draw_robot(robots: Query<&mut core::Robot>, mut transform: Query<&mut Transform, With<Robot>>) {
    let robot = robots.single();
    let transform = &mut transform.single_mut();
    transform.translation.x = robot.position.x;
    transform.translation.y = robot.position.y;
    transform.rotation = Quat::from_rotation_z(robot.orientation);
}