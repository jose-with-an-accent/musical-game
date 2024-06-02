use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;

use crate::playback::InstanceHandle;
#[derive(Component, Debug)]
pub struct Timed {
    start: Duration,   
}
#[derive(Component)] 
pub struct Metadata {
    title: String,
    map_author: String,
    artist: String,
    path: String,
    bpm: String
}
#[derive(Component)]
pub enum Difficulty {
    EASY, NORMAL, HARD
}
// #[derive(Bundle)]
// pub struct Song {
//     pub beats: Vec<Beat>
//     pub metadata: Metadata
// }

// commands.spawn((
//     Timed {
//         from: Duration::from_secs(31),
//         to: Duration::from_secs(33)
//     },
//     BeatDifficulty::EASY,

// ))
#[derive(Event, Debug)]
pub struct BeatHappened(pub Duration);
#[derive(Event, Debug)]
pub struct BeatCreated(pub Duration);
pub struct BeatPlugin;
impl Plugin for BeatPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(On, systems)
        app.add_systems(Update, (watch_beat_changes, check_on_beats));
    }
}
fn check_on_beats(mut query: Query<&Timed>, mut audio_instances: ResMut<Assets<AudioInstance>>,  handle: Res<InstanceHandle>, mut beat_writer: EventWriter<BeatHappened>) {
    for item in query.iter() {
        if let Some(instance) = audio_instances.get_mut(&handle.0) {
            if let Some(time) = instance.state().position() {
                let difference = time - item.start.as_secs_f64();
                match difference {
                    -1.3..=-0.3 => {

                    },
                    -0.3..=0.3 => {
                        println!("Beat just happened! {:?} {:?}", time, item.start);
                        beat_writer.send(BeatHappened(item.start));

                    },
                    0.3..=1.3 => {

                    },
                    _ => ()
                }
                // if 0. < time - item.start.as_secs_f64() && time - item.start.as_secs_f64() <= 0.03 {
                //     println!("Beat just happened! {:?} {:?}", time, item.start);
                //     beat_writer.send(BeatHappened(item.start));
                //     return
                // }
            }

        }
    }
}
fn watch_beat_changes(mut beat_reader: EventReader<BeatCreated>, mut commands: Commands) {
    for event in beat_reader.read() {
        println!("Beat created. Updating... {:?}", event);
        commands.spawn((
            Timed {
                start: event.0
            },
            Difficulty::EASY
        ));
    }
}

