//! text input example

use bevy::{
    color::palettes::css::{BROWN, NAVY, YELLOW},
    prelude::*,
};
use bevy_ui_text_input::{
    TextInputBuffer, TextInputNode, TextInputPlugin, TextInputPrompt, TextInputStyle,
    TextInputSubmitEvent,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TextInputPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, submit)
        .run();
}

#[derive(Component)]
struct OutputMarker;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // UI camera
    commands.spawn(Camera2d);

    let output = commands
        .spawn(Node {
            width: Val::Px(500.),
            max_width: Val::Px(500.),
            min_width: Val::Px(500.),
            ..Default::default()
        })
        .with_child((
            Node {
                width: Val::Px(500.),
                max_width: Val::Px(500.),
                min_width: Val::Px(500.),
                ..Default::default()
            },
            Text::new("Nothing submitted."),
            BackgroundColor(Color::BLACK),
            OutputMarker,
        ))
        .id();

    let editor = commands
        .spawn((
            TextInputNode {
                clear_on_submit: true,
                ..Default::default()
            },
            TextInputPrompt {
                text: "This text from TextInputPrompt is displayed when the input is empty."
                    .to_string(),
                color: Some(BROWN.into()),
                ..Default::default()
            },
            TextInputBuffer::default(),
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
            TextInputStyle::default(),
            BackgroundColor(NAVY.into()),
        ))
        .id();

    let editor_panel = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: Val::Px(500.),
            ..Default::default()
        })
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
        })
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
        .with_child(Text::new("Text Input Example".to_string()))
        .with_children(|commands| {
            commands
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.),
                    ..Default::default()
                })
                .add_child(editor_panel)
                .add_child(output);
        });
}

fn submit(
    mut events: EventReader<TextInputSubmitEvent>,
    mut query: Query<&mut Text, With<OutputMarker>>,
) {
    for event in events.read() {
        println!("submitted: {}", event.text);
        for mut text in query.iter_mut() {
            text.0 = event.text.clone();
        }
    }
}
