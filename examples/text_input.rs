//! text input example

use bevy::{color::palettes::css::LIGHT_GOLDENROD_YELLOW, prelude::*};
use bevy_ui_text_input::{
    SubmitTextEvent, TextInputBuffer, TextInputNode, TextInputPlugin, TextInputPrompt,
    TextInputStyle, TextSubmittedEvent,
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
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(5.),
            ..Default::default()
        },))
        .with_children(|commands| {
            commands
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(2.)),
                        padding: UiRect::all(Val::Px(2.)),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        ..Default::default()
                    },
                    BorderColor(Color::WHITE),
                    BackgroundColor(Color::BLACK),
                ))
                .with_child((
                    Node {
                        width: Val::Px(500.),
                        height: Val::Px(500.),
                        max_height: Val::Px(500.),
                        min_height: Val::Px(500.),
                        max_width: Val::Px(500.),
                        min_width: Val::Px(500.),
                        ..Default::default()
                    },
                    Text::new("Nothing submitted."),
                    OutputMarker,
                ));
        })
        .id();

    let editor = commands
        .spawn((
            TextInputNode {
                clear_on_submit: true,
                ..Default::default()
            },
            TextInputPrompt {
                text: "The TextInputPrompt is shown when the input is empty..".to_string(),
                color: Some(Color::srgb(0.3, 0.3, 0.3)),
                ..Default::default()
            },
            TextInputBuffer::default(),
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 25.,
                ..Default::default()
            },
            TextColor(LIGHT_GOLDENROD_YELLOW.into()),
            Node {
                width: Val::Px(500.),
                height: Val::Px(500.),
                ..default()
            },
            TextInputStyle::default(),
            BackgroundColor(Color::srgb(0., 0., 0.2)),
        ))
        .id();

    let submit_button = commands
        .spawn((
            Node {
                align_self: AlignSelf::Start,
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderColor(Color::WHITE),
        ))
        .with_child(Text::new("Submit"))
        .observe(
            move |_: Trigger<Pointer<Click>>, mut submit_writer: EventWriter<SubmitTextEvent>| {
                submit_writer.send(SubmitTextEvent { entity: editor });
            },
        )
        .id();

    let control_panel = commands
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },))
        .add_child(submit_button)
        .with_children(|commands| {
            commands
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_content: AlignContent::Start,
                    width: Val::Auto,
                    height: Val::Auto,
                    flex_grow: 0.0,
                    flex_shrink: 0.0,
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
                                    node.height = Val::Px(500.);
                                }
                            },
                        )
                        .with_child(Text::new("500h"));
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
                                    node.width = Val::Px(250.);
                                }
                            },
                        )
                        .with_child(Text::new("250w"));

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

    let editor_panel = commands
        .spawn((
            Node {
                border: UiRect::all(Val::Px(2.)),
                padding: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            BorderColor(Color::WHITE),
            BackgroundColor(Color::BLACK),
        ))
        .add_child(editor)
        .id();

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(10.)),
            row_gap: Val::Px(10.),
            column_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_child(Text::new("Text Input Example".to_string()))
        .with_children(|commands| {
            commands
                .spawn(Node {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                    column_gap: Val::Px(10.),
                    row_gap: Val::Px(10.),
                    ..Default::default()
                })
                .add_child(editor_panel)
                .add_child(output)
                .add_child(control_panel)
                .with_child(Text::new(
                    "Press Shift + Enter or click the button to submit",
                ));
        });
}

fn submit(
    mut events: EventReader<TextSubmittedEvent>,
    mut query: Query<&mut Text, With<OutputMarker>>,
) {
    for event in events.read() {
        for mut text in query.iter_mut() {
            text.0 = event.text.clone();
        }
    }
}
