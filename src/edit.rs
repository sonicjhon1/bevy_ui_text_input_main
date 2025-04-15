use arboard::Clipboard;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventReader;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::Local;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::input::ButtonState;
use bevy::input::keyboard::Key;
use bevy::input::keyboard::KeyboardInput;
use bevy::text::cosmic_text::Action;
use bevy::text::cosmic_text::BorrowedWithFontSystem;
use bevy::text::cosmic_text::Edit;
use bevy::text::cosmic_text::Editor;
use bevy::text::cosmic_text::Motion;
use bevy::text::cosmic_text::Selection;
use bevy::time::Time;

use crate::TextInputBuffer;
use crate::TextInputMode;
use crate::TextInputNode;
use crate::TextInputStyle;
use crate::TextInputSubmitEvent;
use crate::text_input_pipeline::TextInputPipeline;

fn apply_motion<'a>(
    editor: &mut BorrowedWithFontSystem<Editor<'a>>,
    shift_pressed: bool,
    motion: Motion,
) {
    if shift_pressed {
        if editor.selection() == Selection::None {
            let cursor = editor.cursor();
            editor.set_selection(Selection::Normal(cursor));
        }
    } else {
        editor.action(Action::Escape);
    }
    editor.action(Action::Motion(motion));
}

fn filter_char_input(mode: TextInputMode, ch: char) -> bool {
    match mode {
        TextInputMode::TextSingleLine => ch != '\n',
        TextInputMode::Text { .. } => {
            // Allow all characters for text mode
            true
        }
        TextInputMode::Number => {
            // Allow only numeric characters
            ch.is_ascii_digit() || ch == '-'
        }
        TextInputMode::Hex => {
            // Allow hexadecimal characters (0-9, a-f, A-F)
            ch.is_ascii_hexdigit()
        }
        TextInputMode::Decimal => {
            // Allow numeric characters and a single decimal point
            ch.is_ascii_digit() || ch == '.' || ch == '-'
        }
    }
}

fn filter_text(mode: TextInputMode, text: &str) -> bool {
    matches!(mode, TextInputMode::Text { .. }) || text.chars().all(|ch| filter_char_input(mode, ch))
}

pub fn text_input_edit_system(
    mut shift_pressed: Local<bool>,
    mut command_pressed: Local<bool>,
    mut keyboard_events_reader: EventReader<KeyboardInput>,
    mut query: Query<(Entity, &TextInputNode, &mut TextInputBuffer)>,
    mut text_input_pipeline: ResMut<TextInputPipeline>,
    mut submit_event: EventWriter<TextInputSubmitEvent>,
) {
    let mut clipboard = Clipboard::new();
    let keyboard_events: Vec<_> = keyboard_events_reader.read().collect();

    let mut font_system = &mut text_input_pipeline.font_system;

    for (entity, input, mut buffer) in query.iter_mut() {
        buffer.changed = false;
        if !input.is_active {
            continue;
        }

        let mut flag = false;
        let mut editor = buffer.editor.borrow_with(&mut font_system);
        for event in &keyboard_events {
            match event.logical_key {
                Key::Shift => {
                    *shift_pressed = event.state == ButtonState::Pressed;
                    continue;
                }
                Key::Control => {
                    *command_pressed = event.state == ButtonState::Pressed;
                    continue;
                }
                #[cfg(target_os = "macos")]
                Key::Super => {
                    *command_pressed = event.state == ButtonState::Pressed;
                    continue;
                }
                _ => {}
            };

            let mut changed = true;
            if event.state.is_pressed() {
                if *command_pressed {
                    match &event.logical_key {
                        Key::Character(str) => {
                            if let Some(char) = str.chars().next() {
                                match char {
                                    'c' => {
                                        // copy
                                        if let Ok(ref mut clipboard) = clipboard {
                                            if let Some(text) = editor.copy_selection() {
                                                let _ = clipboard.set_text(text);
                                            }
                                        }
                                    }
                                    'x' => {
                                        // cut
                                        if let Ok(ref mut clipboard) = clipboard {
                                            if let Some(text) = editor.copy_selection() {
                                                let _ = clipboard.set_text(text);
                                            }
                                        }
                                        editor.delete_selection();
                                    }
                                    'v' => {
                                        // paste
                                        if let Ok(ref mut clipboard) = clipboard {
                                            if let Ok(text) = clipboard.get_text() {
                                                if filter_text(input.mode, &text) {
                                                    editor.insert_string(&text, None);
                                                }
                                            }
                                        }
                                    }
                                    'z' => {
                                        // undo
                                    }
                                    'y' | 'Z' => {
                                        // redo
                                    }
                                    'a' => {
                                        // select all
                                        editor.action(Action::Motion(Motion::BufferStart));
                                        let cursor = editor.cursor();
                                        editor.set_selection(Selection::Normal(cursor));
                                        editor.action(Action::Motion(Motion::BufferEnd));
                                    }
                                    _ => {
                                        // not recognised, ignore
                                    }
                                }
                            }
                        }
                        Key::ArrowLeft => {
                            apply_motion(&mut editor, *shift_pressed, Motion::PreviousWord);
                        }
                        Key::ArrowRight => {
                            apply_motion(&mut editor, *shift_pressed, Motion::NextWord);
                        }
                        Key::ArrowUp => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Up);
                        }
                        Key::ArrowDown => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Down);
                        }
                        Key::Home => {
                            apply_motion(&mut editor, *shift_pressed, Motion::BufferStart);
                        }
                        Key::End => {
                            apply_motion(&mut editor, *shift_pressed, Motion::BufferEnd);
                        }
                        _ => {
                            // not recognised, ignore
                        }
                    }
                } else {
                    match &event.logical_key {
                        Key::Character(str) => {
                            if let Some(char) = str
                                .chars()
                                .next()
                                .filter(|char| filter_char_input(input.mode, *char))
                            {
                                editor.action(Action::Insert(char));
                            }
                        }
                        Key::Space => {
                            editor.action(Action::Insert(' '));
                        }
                        Key::Enter => match (*shift_pressed, input.mode) {
                            (false, TextInputMode::Text { .. }) => {
                                editor.action(Action::Enter);
                            }
                            _ => {
                                let text = editor.with_buffer(crate::get_text);
                                submit_event.send(TextInputSubmitEvent {
                                    text_input_id: entity,
                                    text,
                                });
                            }
                        },
                        Key::Backspace => {
                            editor.action(Action::Backspace);
                        }
                        Key::Delete => {
                            if *shift_pressed {
                                // cut
                                if let Ok(ref mut clipboard) = clipboard {
                                    if let Some(text) = editor.copy_selection() {
                                        let _ = clipboard.set_text(text);
                                    }
                                }
                                editor.delete_selection();
                            } else {
                                editor.action(Action::Delete);
                            }
                        }
                        Key::PageUp => {
                            apply_motion(&mut editor, *shift_pressed, Motion::PageUp);
                        }
                        Key::PageDown => {
                            apply_motion(&mut editor, *shift_pressed, Motion::PageDown);
                        }
                        Key::ArrowLeft => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Left);
                        }
                        Key::ArrowRight => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Right);
                        }
                        Key::ArrowUp => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Up);
                        }
                        Key::ArrowDown => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Down);
                        }
                        Key::Home => {
                            apply_motion(&mut editor, *shift_pressed, Motion::Home);
                        }
                        Key::End => {
                            apply_motion(&mut editor, *shift_pressed, Motion::End);
                        }
                        Key::Escape => {
                            editor.action(Action::Escape);
                        }
                        Key::Tab => {
                            if *shift_pressed {
                                editor.action(Action::Unindent);
                            } else {
                                editor.action(Action::Indent);
                            }
                        }
                        _ => {
                            changed = false;
                        }
                    }
                }
            }
            if changed {
                flag = true;
            }
        }

        buffer.changed = flag;
    }
}

pub fn update_cursor_blink_timers(
    time: Res<Time>,
    mut query: Query<(&TextInputNode, &mut TextInputBuffer, &TextInputStyle)>,
) {
    for (input, mut buffer, style) in query.iter_mut() {
        if !input.is_active
            || style.blink_interval * 2. <= buffer.cursor_blink_time
            || buffer.changed
        {
            buffer.cursor_blink_time = 0.;
        } else {
            buffer.cursor_blink_time += time.delta_secs();
        }
    }
}
