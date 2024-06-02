use bevy::prelude::*;

use crate::player::PlayerScored;

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, create_score);
        app.add_systems(Update, update_score);
    }
}
fn update_score(mut event_reader: EventReader<PlayerScored>) {
    for event in event_reader.read() {
        println!("{:?}", event);
    }
}