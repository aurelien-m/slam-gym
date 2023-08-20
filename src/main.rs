use bevy::prelude::*;

mod gym;

fn main() {
    App::new()
        .add_systems(Startup, setup_bevy)
        .add_plugins((DefaultPlugins, gym::GymPlugin))
        .run();
}

fn setup_bevy(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
