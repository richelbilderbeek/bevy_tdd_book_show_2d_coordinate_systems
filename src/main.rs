use crate::app::*;
use bevy::prelude::*;
mod app;

fn main() {
    let mut app = create_app();
    app.add_plugins(DefaultPlugins);
    app.run();
}
