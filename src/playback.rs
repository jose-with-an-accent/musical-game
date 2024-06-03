use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance};
use bevy_kira_audio::prelude::*;
#[derive(Resource)]
pub struct InstanceHandle(pub Handle<AudioInstance>);

pub struct PlaybackPlugin;
impl Plugin for PlaybackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_audio);
        app.add_systems(Update, update_playback);
    }
}
fn create_audio(mut commands: Commands, audio: Res<Audio>, asset_server: Res<AssetServer>) {
    let handle = audio.play(asset_server.load("songs/01.ogg")).handle();
    commands.insert_resource(InstanceHandle(handle));
}
fn update_playback(mut audio_instances: ResMut<Assets<AudioInstance>>, audio: Res<Audio>, handle: Res<InstanceHandle>, mut playback_events: EventReader<PlaybackEvent>) {
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        for event in playback_events.read() {
            match event {
                PlaybackEvent::PauseRequested => instance.pause(AudioTween::default()),
                PlaybackEvent::PlayRequested => instance.resume(AudioTween::default()),
                PlaybackEvent::BeginningRequested => instance.seek_to(0.),
                PlaybackEvent::ChangeSong(file_name ) => todo!("Unimplemented")
            };
        };
    };
}
#[derive(Event, Debug)]
pub enum PlaybackEvent {
    PauseRequested,
    PlayRequested,
    BeginningRequested,
    ChangeSong(String)
}