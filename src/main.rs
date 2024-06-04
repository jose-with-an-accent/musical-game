use beat::{BeatCreated, BeatHappened, BeatPlugin, BeatsCleared};
use bevy::prelude::*;
mod camera;
mod movement;
mod editor;
mod player;
mod state;
mod score;
mod playback;
mod beat;
use bevy_kira_audio::AudioPlugin;
use playback::{PlaybackEvent, PlaybackPlugin};
use state::SongState;
use editor::EditorPlugin;
use player::{PlayerPlugin, PlayerScored};
use camera::CameraPlugin;
fn main() {
    App::new()
    .insert_resource(Time::<Fixed>::from_hz(60.))
    .add_event::<PlaybackEvent>()
    .add_event::<BeatCreated>()
    .add_event::<BeatHappened>()
    .add_event::<PlayerScored>()
    .add_event::<BeatsCleared>()
    .insert_resource(ClearColor(Color::ORANGE_RED))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 43.
    })
    .init_resource::<SongState>()
    .add_plugins(DefaultPlugins)
    .add_plugins(BeatPlugin)
    .add_plugins(AudioPlugin)
    .add_plugins(EditorPlugin)
    .add_plugins(PlaybackPlugin)
    .add_plugins(CameraPlugin)
    .add_plugins(PlayerPlugin)
    .run();
}
