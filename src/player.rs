use bevy::prelude::*;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        // app.add_systems(Update, update_player);
    }
}
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("songs/01.ogg"),
        settings: PlaybackSettings {
            spatial: true,
            ..default()
        }
    });
}
