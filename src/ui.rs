use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiContexts, EguiPlugin};
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SongState>();
        app.add_plugins(EguiPlugin);
        app.add_systems(Update, create_ui);
    }
}
fn create_ui(mut song_state: ResMut<SongState>, mut contexts: EguiContexts, time: Res<Time>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("song_generation_panel").exact_width(200.).show(ctx, |ui| {
        ui.heading("Song Editor");
        ui.horizontal(|ui| {
            ui.button("Rewind");
            ui.button("Play");
            ui.button("Pause");
            ui.button("Forward");
        });
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
            ui.label(format!("{:?}", note));
        }
        if ui.button("Add New").clicked() {
            &song_state.notes.push(time.elapsed());
        };
        ui.horizontal(|ui| {
            ui.button("Save");
            ui.button("Test");
        });
    });
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