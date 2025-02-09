use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Squishy Game".to_string(),
                present_mode: PresentMode::AutoNoVsync,
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..Default::default()
        }))
        .add_plugins(squishy_lib::simulation::SimulationPlugin)
        .add_plugins(squishy_lib::rendering::RenderPlugin)
        .add_plugins(squishy_lib::gui::GuiPlugin)
        .run();
}
