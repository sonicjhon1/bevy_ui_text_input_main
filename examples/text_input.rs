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
        .with_child(Text::new("Text Input".to_string()))
        .add_child(editor)
        .with_children(|commands| {
            commands
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(4.),
                    ..Default::default()
                })
                .with_children(|commands| {
                    commands
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.),
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .with_child(Text::new("sans"))
                            .observe(
                                move |_: Trigger<Pointer<Click>>,
                                    mut query: Query<&mut TextFont>,
                                    assets: Res<AssetServer>| {
                                    if let Ok(mut text_font) = query.get_mut(editor) {
                                        text_font.font = assets.load("fonts/FiraSans-Bold.ttf");
                                    }
                                },
                            );
                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>,
                                      mut query: Query<&mut TextFont>,
                                      assets: Res<AssetServer>| {
                                    if let Ok(mut text_font) = query.get_mut(editor) {
                                        text_font.font = assets.load("fonts/FiraMono-Medium.ttf");
                                    }
                                },
                            )
                            .with_child(Text::new("mono"));
                        });
                    commands
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.),
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut TextFont>| {
                                    if let Ok(mut text_font) = query.get_mut(editor) {
                                        text_font.font_size = 16.;
                                    }
                                },
                            )
                            .with_child(Text::new("16"));

                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut TextFont>| {
                                    if let Ok(mut text_font) = query.get_mut(editor) {
                                        text_font.font_size = 25.;
                                    }
                                },
                            )
                            .with_child(Text::new("25"));
                        });

                    commands
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.),
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut Node>| {
                                    if let Ok(mut node) = query.get_mut(editor) {
                                        node.height = Val::Px(250.);
                                    }
                                },
                            )
                            .with_child(Text::new("250h"));

                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut Node>| {
                                    if let Ok(mut node) = query.get_mut(editor) {
                                        node.height = Val::Px(400.);
                                    }
                                },
                            )
                            .with_child(Text::new("400h"));
                        });

                    commands
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(4.),
                            ..Default::default()
                        })
                        .with_children(|commands| {
                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut Node>| {
                                    if let Ok(mut node) = query.get_mut(editor) {
                                        node.width = Val::Px(300.);
                                    }
                                },
                            )
                            .with_child(Text::new("300w"));

                            commands
                            .spawn((
                                Node {
                                    border: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(2.)),
                                    ..Default::default()
                                },
                                BorderColor(Color::WHITE),
                            ))
                            .observe(
                                move |_: Trigger<Pointer<Click>>, mut query: Query<&mut Node>| {
                                    if let Ok(mut node) = query.get_mut(editor) {
                                        node.width = Val::Px(500.);
                                    }
                                },
                            )
                            .with_child(Text::new("500w"));
                        });
                });
        });

    //commands.insert_resource(InputFocus::from_entity(editor));
}
