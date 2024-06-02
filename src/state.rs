use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SongState {
    tempo: u32,
    name: String,
    notes: Vec<Duration>
}