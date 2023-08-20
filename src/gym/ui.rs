use super::core;
use super::graphics;
use bevy::input::common_conditions::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_camera).add_systems(
            Update,
            add_trajectory_point.run_if(input_just_pressed(MouseButton::Right)),
        );
    }
}

fn update_camera(
    mut cameras: Query<&mut Transform, With<Camera2d>>,
    mouse_input: Res<Input<MouseButton>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let mut camera = cameras.iter_mut().next().unwrap();
    let mut cursor_iter = cursor_moved_events.iter();

    if let (Some(first_cursor), Some(last_cursor)) = (cursor_iter.next(), cursor_iter.last()) {
        if mouse_input.pressed(MouseButton::Left) {
            camera.translation.x -= last_cursor.position.x - first_cursor.position.x;
            camera.translation.y += last_cursor.position.y - first_cursor.position.y;
        }
    }
}

fn add_trajectory_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut robots: Query<&mut core::Robot>,
) {
    let (camera, camera_transform) = cameras.single();
    if let Some(world_position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let mut robot = robots.single_mut();
        robot
            .trajectory
            .push(Vec2::new(world_position.x, world_position.y));

        if robot.trajectory.len() == 1 {
            graphics::draw_point(
                &mut commands,
                &mut meshes,
                &mut materials,
                robot.trajectory[0],
            )
        } else if robot.trajectory.len() > 1 {
            graphics::draw_line(
                &mut commands,
                &mut meshes,
                &mut materials,
                robot.trajectory[robot.trajectory.len() - 2],
                robot.trajectory[robot.trajectory.len() - 1],
            )
        }
    }
}
