use std::{fs, time::Duration};

use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
use bevy_kira_audio::AudioInstance;

use crate::{
    beat::{BeatCreated, BeatHappened, BeatsCleared}, level::LevelIOEvent, playback::{InstanceHandle, PlaybackEvent}, state::{CurrentScreen, SongState}
};
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, (create_ui, check_keyboard_input));
    }
}
fn check_keyboard_input(
    mut song_state: ResMut<SongState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    handle: Res<InstanceHandle>,
    mut beat_writer: EventWriter<BeatCreated>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if let Some(instance) = audio_instances.get_mut(&handle.0) {
            let duration = instance.state().position().unwrap();
            let _ = &song_state.notes.push(Duration::from_secs_f64(duration));
            beat_writer.send(BeatCreated(Duration::from_secs_f64(duration)));
        }
    }
}
fn get_songs() -> Vec<String> {
    let mut songs: Vec<String> = Vec::new();

    let files = fs::read_dir("assets/songs").unwrap();
    files.for_each(|file| songs.push(file.unwrap().file_name().into_string().unwrap()));

    songs
}
fn create_ui(
    mut song_state: ResMut<SongState>,
    mut contexts: EguiContexts,
    mut clear_beats_writer: EventWriter<BeatsCleared>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    handle: Res<InstanceHandle>,
    mut playback_writer: EventWriter<PlaybackEvent>,
    mut beat_writer: EventWriter<BeatCreated>,
    mut beat_reader: EventReader<BeatHappened>,
    mut level_writer: EventWriter<LevelIOEvent>
) {
    let ctx = contexts.ctx_mut();
    if let Some(instance) = audio_instances.get_mut(&handle.0) {
        let duration = instance.state().position().unwrap();
        let mut latest_beat: Option<&BeatHappened> = None;

        for event in beat_reader.read() {
            latest_beat = Some(event);
        }
        egui::TopBottomPanel::top("options_panel")
            .show(ctx, |ui| {
                ui.menu_button("Map", |ui| {
                    ui.button("Open");
                    if ui.button("Save").clicked() {
                        level_writer.send(LevelIOEvent::SAVE);
                    };
            });
    });
        egui::SidePanel::right("music_playback_panel")
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("FRW").clicked() {
                    playback_writer.send(PlaybackEvent::BeginningRequested);
                }

                ui.button("Rewind");
                if ui.button("Play").clicked() {
                    playback_writer.send(PlaybackEvent::PlayRequested);
                }
                if ui.button("Pause").clicked() {
                    playback_writer.send(PlaybackEvent::PauseRequested);
                }
                ui.button("Forward");

            })
        });
        egui::SidePanel::left("song_generation_panel")
            .exact_width(200.)
            .show(ctx, |ui| {
                ui.menu_button("Change Song", |ui| {
                    for song in get_songs() {
                        if ui.button(&song).clicked() {
                            playback_writer.send(PlaybackEvent::ChangeSong(song));
                        }
                    }
                });


                ui.heading("Song Editor");
                ui.label(&song_state.name);
                ui.horizontal(|ui| {
                    ui.label("Song Name");
                    ui.text_edit_singleline(&mut song_state.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Tempo");
                    ui.add(egui::Slider::new(&mut song_state.tempo, 0..=200).text("bpm"));
                });
                ui.heading("Note List");
                ui.horizontal(|ui| {
                    if ui.button("Add New").clicked() {
                        let _ = &song_state.notes.push(Duration::from_secs_f64(duration));
                        beat_writer.send(BeatCreated(Duration::from_secs_f64(duration)));
                    };
                    if ui.button("Reset").clicked() {
                        let _ = &song_state.notes.clear();
                        clear_beats_writer.send(BeatsCleared);

                    }
                });

                for note in &song_state.notes {
                    ui.horizontal(|ui| {
                        if let Some(latest_beat) = latest_beat {
                            if &latest_beat.duration == note {
                                match &latest_beat.accuracy {
                                    -0.3..=0. => {
                                        ui.colored_label(egui::Color32::RED, format!("{:?}", note))
                                    }
                                    _ => ui.colored_label(
                                        egui::Color32::LIGHT_YELLOW,
                                        format!("{:?}", note),
                                    ),
                                };
                                return;
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
                ui.horizontal(|ui| {
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
