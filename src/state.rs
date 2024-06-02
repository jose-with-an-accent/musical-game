use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct SongState {
    pub tempo: u32,
    pub name: String,
    pub notes: Vec<Duration>
}
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Paused,
    MenuScreen,
    Normal,

}

