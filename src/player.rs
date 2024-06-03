use bevy::{input::keyboard::KeyboardInput, math::Vec3A, prelude::*, render::primitives::Sphere};
use bevy_kira_audio::Audio;

use crate::beat::BeatHappened;
#[derive(Component)]
pub struct Player {

}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, update_player_input);
    }
}
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
}
fn update_player_input(keyboard_inputs: Res<ButtonInput<KeyCode>>, mut score_writer: EventWriter<PlayerScored>, mut beat_reader: EventReader<BeatHappened>) {
    let mut happened = false;
    for event in beat_reader.read() {
        happened = true;
    }
    if keyboard_inputs.pressed(KeyCode::Space) && happened {
        println!("Space pressed!");
        score_writer.send(PlayerScored::GOOD);
    }
}
#[derive(Event, Debug)]
pub enum PlayerScored {
    BAD, GOOD
}