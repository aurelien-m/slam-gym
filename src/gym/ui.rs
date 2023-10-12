use super::core;
use super::graphics;
use bevy::input::common_conditions::*;
use bevy::window::PrimaryWindow;
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UiPlugin;

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ShowFps(bool);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_camera.run_if(input_pressed(MouseButton::Right)),
                    text_update_system,
                    add_trajectory_point.run_if(input_just_pressed(MouseButton::Right)),
                    change_show_fps.run_if(input_just_pressed(KeyCode::F)),
                ),
            )
            .add_plugins(FrameTimeDiagnosticsPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(10.0),
            ..default()
        }),
        FpsText,
    ));
    commands.spawn(ShowFps(false));
}

fn update_camera(
    mut cameras: Query<&mut Transform, With<Camera2d>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let mut camera = cameras.iter_mut().next().unwrap();
    let mut cursor_iter = cursor_moved_events.iter();

    if let (Some(first_cursor), Some(last_cursor)) = (cursor_iter.next(), cursor_iter.last()) {
        camera.translation.x -= last_cursor.position.x - first_cursor.position.x;
        camera.translation.y += last_cursor.position.y - first_cursor.position.y;
    }
}

fn add_trajectory_point(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut trajectory: Query<&mut core::Trajectory>,
) {
    let (camera, camera_transform) = cameras.single();
    if let Some(world_position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let mut trajectory = trajectory.single_mut();
        trajectory
            .0
            .push(Vec2::new(world_position.x, world_position.y));

        if trajectory.0.len() == 1 {
            graphics::draw_point(&mut commands, &mut meshes, &mut materials, trajectory.0[0])
        } else if trajectory.0.len() > 1 {
            graphics::draw_line(
                &mut commands,
                &mut meshes,
                &mut materials,
                trajectory.0[trajectory.0.len() - 2],
                trajectory.0[trajectory.0.len() - 1],
            )
        }
    }
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
    show_fps: Query<&ShowFps>,
) {
    if !show_fps.single().0 {
        return;
    }

    let mut text = query.single_mut();
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            text.sections[0].value = format!("FPS: {value:.2}");
        }
    }
}

fn change_show_fps(mut show_fps: Query<&mut ShowFps>, mut query: Query<&mut Text, With<FpsText>>) {
    let mut show_fps = show_fps.single_mut();
    show_fps.0 = !show_fps.0;

    if !show_fps.0 {
        let mut text = query.single_mut();
        text.sections[0].value = String::new();
    }
}
