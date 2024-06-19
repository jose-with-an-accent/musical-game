use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SongState {
    pub tempo: u32,
    pub name: String,
    pub notes: Vec<Duration>
}
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Paused,
    Normal,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentScreen {
    MENU, PLAY, SELECTION, EDITOR
}