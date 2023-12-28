use bevy::prelude::*;
use bevy::window::WindowMode;
// use my_bevy_roguelike::GamePlugin; 

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            // GamePlugin,
        ))
        .run()
}
