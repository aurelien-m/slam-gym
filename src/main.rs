use bevy::prelude::*;

mod gym;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, gym::GymPlugin))
        .add_systems(Startup, setup_bevy)
        .add_systems(Update, update_camera)
        .run();
}

fn setup_bevy(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
