//! text input example

use bevy::{
    color::palettes::css::{NAVY, RED, YELLOW},
    prelude::*,
};
use bevy_ui_text_input::{TextInputNode, TextInputPlugin, TextInputStyle};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2d);

    let editor = commands
        .spawn((
            TextInputNode::default(),
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.,
                ..Default::default()
            },
            TextColor(YELLOW.into()),
            Node {
                width: Val::Px(500.),
                height: Val::Px(250.),
                ..default()
            },
            TextInputStyle {
                selected_text_color: Some(RED.into()),
                ..default()
            },
            BackgroundColor(NAVY.into()),
        ))
        .id();
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            ..Default::default()
        })
        .add_child(editor)
        .with_child((
            Text::new("abcde".to_string()),
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.,
                ..Default::default()
            },
        ));
}
