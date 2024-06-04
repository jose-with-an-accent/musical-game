use std::time::Duration;

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
        app.add_systems(FixedUpdate, update_player_input);
    }
}
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
}
fn update_player_input(keyboard_inputs: Res<ButtonInput<KeyCode>>, mut score_writer: EventWriter<PlayerScored>, mut beat_reader: EventReader<BeatHappened>) {
    let mut accuracy: Option<f64> = None;
    for event in beat_reader.read() {
        accuracy = Some(event.accuracy);
    }
    if keyboard_inputs.just_pressed(KeyCode::Space) && accuracy.is_some() {
        let num = accuracy.unwrap();
        match num.is_sign_positive() {
            true => println!("LATE w/ accuracy {:?}", Duration::from_secs_f64(num.abs())),
            false => println!("EARLY w/ accuracy {:?}", Duration::from_secs_f64(num.abs()))
        }
        score_writer.send(PlayerScored::GOOD);
    }
}
#[derive(Event, Debug)]
pub enum PlayerScored {
    BAD, GOOD
}