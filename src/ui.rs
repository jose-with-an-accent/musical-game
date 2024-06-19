use bevy::prelude::*;
use sickle_ui::{
    ui_builder::{UiBuilderExt, UiRoot}, ui_commands::SetTextExt, ui_style::{
        SetBackgroundColorExt, SetNodeHeightExt, SetNodePositionTypeExt, SetNodeRightExt,
        SetNodeTopExt,
    }, widgets::{column::UiColumnExt, label::{LabelConfig, UiLabelExt}}
};
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_ui);
    }
}
fn create_ui(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|column| {
        column
            .style()
            .position_type(PositionType::Absolute)
            .height(Val::Auto)
            .top(Val::Vw(50.))
            .right(Val::Vw(50.))
            .background_color(Color::RED);
        column
            .label(LabelConfig::default())
            .entity_commands()
            // We can use the set_text method to set the text of a label.
            .set_text("This is label 1.", None);

        column
            .label(LabelConfig::default())
            .entity_commands()
            .set_text("This is another label.", None);
    });
}
