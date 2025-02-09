use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(InfiniteGridPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_rendering);
    }
}

fn setup_rendering(mut commands: Commands) {
    commands.spawn((
        Camera3d {
            ..Default::default()
        },
        PanOrbitCamera::default(),
        Transform::from_xyz(0.0, 10.0, 20.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
    ));
    commands.spawn(InfiniteGridBundle::default());
}