use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let initial_camera_scale = 1.0;
    let initial_player_position = Vec2::new(320.0, 240.0);
    let initial_player_size = Vec2::new(64.0, 32.0);
    let mut app = create_app(
        initial_camera_scale,
        initial_player_position,
        initial_player_size,
    );
    app.add_plugins(DefaultPlugins);
    app.run();
}
