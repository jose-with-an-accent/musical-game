use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy_kira_audio::AudioInstance;

use crate::{beat::{BeatCreated, BeatHappened}, playback::{InstanceHandle, PlaybackEvent}, state::SongState};
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, create_ui);
    }
}
fn create_ui(mut song_state: ResMut<SongState>, mut contexts: EguiContexts, mut audio_instances: ResMut<Assets<AudioInstance>>,  handle: Res<InstanceHandle>, mut playback_writer: EventWriter<PlaybackEvent>, mut beat_writer: EventWriter<BeatCreated>, mut beat_reader: EventReader<BeatHappened>) {
    let ctx = contexts.ctx_mut();
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        let duration = instance.state().position().unwrap();
    let mut latest_beat:Option<Duration> = None;

    for event in beat_reader.read() {
        latest_beat = Some(event.0);
    }

    egui::SidePanel::left("song_generation_panel").exact_width(200.).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.button("Rewind");
            if ui.button("Play").clicked() {
                playback_writer.send(PlaybackEvent::PlayRequested);
            }
            if ui.button("Pause").clicked() {
                playback_writer.send(PlaybackEvent::PauseRequested);
            }
            ui.button("Forward");
        });
        ui.horizontal(|ui| {
            if ui.button("Beginning").clicked() {
                playback_writer.send(PlaybackEvent::BeginningRequested);
            }
        });
        ui.heading("Song Editor");

        ui.horizontal(|ui| {
            ui.label("Song Name");
            ui.text_edit_singleline(&mut song_state.name);
        });
        ui.horizontal(|ui| {
            ui.label("Tempo");
            ui.add(egui::Slider::new(&mut song_state.tempo, 0..=200).text("bpm"));
        });
        ui.heading("Note List");
        for note in &song_state.notes {
            ui.horizontal(|ui| {
                if let Some(latest_beat) = latest_beat {
                    if &latest_beat == note {
                        ui.colored_label(egui::Color32::RED, format!("{:?}", note));
                    } else {
                        ui.label(format!("{:?}", note));
                    }

                } else {
                    ui.label(format!("{:?}", note));
                }
                if ui.button("Remove").clicked() {
                    // song_state.notes.remove(note.);
                }
            });
        }
        if ui.button("Add New").clicked() {
            let _ = &song_state.notes.push(Duration::from_secs_f64(duration));
            beat_writer.send(BeatCreated(Duration::from_secs_f64(duration)));
        };
        ui.horizontal(|ui| {
            ui.button("Save");
            ui.button("Test");
        });
    });
}
    // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
    //     // The top panel is often a good place for a menu bar:
    //     egui::menu::bar(ui, |ui| {
    //         egui::menu::menu_button(ui, "File", |ui| {
    //             if ui.button("Quit").clicked() {
    //                 std::process::exit(0);
    //             }
    //         });
    //     });
    // });
}