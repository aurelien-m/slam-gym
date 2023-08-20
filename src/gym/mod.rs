use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod core;
mod graphics;

pub struct GymPlugin;

impl Plugin for GymPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(core::GymCorePlugin)
            .add_plugins(graphics::GymGraphicsPlugin);
    }
}
