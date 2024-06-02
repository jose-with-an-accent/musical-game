use bevy::prelude::*;
mod camera;
mod movement;
mod ui;
mod player;
mod state;
use state::SongState;
use ui::UiPlugin;
use player::PlayerPlugin;
use camera::CameraPlugin;
fn main() {
    let app = App::new()
    .insert_resource(ClearColor(Color::ORANGE_RED))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 43.
    })
    .init_resource::<SongState>()
    .add_plugins(DefaultPlugins)
    .add_plugins(UiPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(PlayerPlugin)
    .run();
}
