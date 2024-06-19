use std::time::Duration;

use bevy::{
    math::{primitives::Sphere, Vec3A},
    prelude::*,
};
use bevy_reflect::{prelude::*};
use bevy_kira_audio::AudioInstance;

use crate::playback::InstanceHandle;
#[derive(Component, Debug, Reflect)]
pub struct Timed {
    start: Duration,
}
#[derive(Component, Reflect)]
pub struct Metadata {
    title: String,
    map_author: String,
    artist: String,
    path: String,
    bpm: String,
}
#[derive(Component, Reflect)]
pub enum Difficulty {
    EASY,
    NORMAL,
    HARD,
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
pub struct BeatHappened {
    pub duration: Duration,
    pub accuracy: f64,
}
#[derive(Event, Debug)]
pub struct BeatCreated(pub Duration);
#[derive(Event, Debug)]
pub struct BeatsCleared;
pub struct BeatPlugin;
#[derive(Component)]
pub enum Transition {
    PRE,
    POST,
}
#[derive(Bundle, Reflect)]
pub struct Beat {
    timed: Timed,
}
impl Plugin for BeatPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(On, systems)
        app.add_systems(FixedUpdate, (watch_beat_changes, check_on_beats));
        app.add_systems(Startup, (create_beats));
    }
}
fn create_beats(mut commands: Commands) {
    // commands.spawn(PbrBundle {
    //     mesh: Sphere::new(25.)
    // });
}
fn check_on_beats(
    mut commands: Commands,
    mut query: Query<&Timed>,
    mut query2: Query<Entity, With<Timed>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    handle: Res<InstanceHandle>,
    mut beat_writer: EventWriter<BeatHappened>,
) {
    for (mut beat, id) in query.iter().zip(query2.iter()) {
        if let Some(instance) = audio_instances.get_mut(&handle.0) {
            if let Some(playback_time) = instance.state().position() {
                let difference = playback_time - beat.start.as_secs_f64();
                match difference {
                    -1.3..=-0.3 => {
                        commands.entity(id).insert(Transition::PRE);
                    }
                    -0.3..=0.3 => {
                        // println!("Beat just happened! {} difference from {:?} beat", &difference, item.start);
                        beat_writer.send(BeatHappened {
                            duration: beat.start,
                            accuracy: difference,
                        });
                    }
                    0.3..=1.3 => {
                        commands.entity(id).remove::<Transition>();
                    }
                    _ => (),
                }
            }
        }
    }
}
fn watch_beat_changes(
    mut beat_reader: EventReader<BeatCreated>,
    mut commands: Commands,
    mut clear_beats_reader: EventReader<BeatsCleared>,
    entity_query: Query<Entity, With<Timed>>,
) {
    for event in clear_beats_reader.read() {
        for id in entity_query.iter() {
            commands.entity(id).remove::<(Timed, Transition)>();
        }
        println!("Beats Cleared!");
    }
    for event in beat_reader.read() {
        println!("Beat created. Updating... {:?}", event);
        commands.spawn((
            Beat {
                timed: Timed { start: event.0 },
            },
            Transition::PRE,
            Transform {
                translation: Vec3::new(5., 5., 5.),
                rotation: Quat::default(),
                scale: Vec3::ONE,
            },
        ));
    }
}
