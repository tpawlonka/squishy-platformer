use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                ..default()
            }),
            ..Default::default()
        }));
}
