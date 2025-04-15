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

fn buffer_len(buffer: &bevy::text::cosmic_text::Buffer) -> usize {
    buffer
        .lines
        .iter()
        .map(|line| line.text().chars().count())
        .sum()
}

fn cursor_at_buffer_end(editor: &mut BorrowedWithFontSystem<Editor<'_>>) -> bool {
    let cursor = editor.cursor();
    editor.with_buffer(|buffer| {
        cursor.line == buffer.lines.len() - 1
            && buffer
                .lines
                .get(cursor.line)
                .map(|line| cursor.index == line.text().len())
                .unwrap_or(false)
    })
}

pub fn text_input_edit_system(
    mut shift_pressed: Local<bool>,
    mut command_pressed: Local<bool>,
    mut keyboard_events_reader: EventReader<KeyboardInput>,
    mut query: Query<(
        Entity,
        &TextInputNode,
        &mut TextInputBuffer,
        &TextInputStyle,
    )>,
    mut text_input_pipeline: ResMut<TextInputPipeline>,
    mut submit_event: EventWriter<TextInputSubmitEvent>,
    time: Res<Time>,
) {
    let mut clipboard = Clipboard::new();
    let keyboard_events: Vec<_> = keyboard_events_reader.read().collect();

    let mut font_system = &mut text_input_pipeline.font_system;

    for (entity, input, mut buffer, style) in query.iter_mut() {
        if !input.is_active {
            buffer.cursor_blink_time = f32::MAX;
            continue;
        }

        buffer.cursor_blink_time = if keyboard_events.is_empty() {
            (buffer.cursor_blink_time + time.delta_secs()).rem_euclid(style.blink_interval * 2.)
        } else {
            0.
        };

        let TextInputBuffer {
            editor,
            overwrite_mode,
            ..
        } = &mut *buffer;

        let mut editor = editor.borrow_with(&mut font_system);

        if editor.with_buffer(|buffer| buffer.wrap() != input.mode.wrap()) {
            apply_motion(&mut editor, *shift_pressed, Motion::BufferStart);
            editor.action(Action::Escape);

            editor.with_buffer_mut(|buffer| {
                buffer.set_wrap(input.mode.wrap());
            });
        }

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
                                                if input.max_chars.is_none_or(|max| {
                                                    editor.with_buffer(buffer_len) + text.len()
                                                        <= max
                                                }) {
                                                    if filter_text(input.mode, &text) {
                                                        editor.insert_string(&text, None);
                                                    }
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
                    let mut key = event.logical_key.clone();
                    if event.logical_key == Key::Space {
                        key = Key::Character(" ".into());
                    }
                    match key {
                        Key::Character(str) => {
                            if let Some(char) = str
                                .chars()
                                .next()
                                .filter(|ch| filter_char_input(input.mode, *ch))
                            {
                                if *overwrite_mode {
                                    if editor.selection() != Selection::None {
                                        editor.action(Action::Insert(char));
                                    } else if !(cursor_at_buffer_end(&mut editor)
                                        && input.max_chars.is_some_and(|max_chars| {
                                            max_chars <= editor.with_buffer(buffer_len)
                                        }))
                                    {
                                        editor.action(Action::Delete);
                                        editor.action(Action::Insert(char));
                                    }
                                } else {
                                    if input.max_chars.is_none_or(|max_chars| {
                                        editor.with_buffer(buffer_len) < max_chars
                                    }) {
                                        editor.action(Action::Insert(char));
                                    }
                                }
                            }
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

                                if input.clear_on_submit {
                                    editor.action(Action::Motion(Motion::BufferStart));
                                    let cursor = editor.cursor();
                                    editor.set_selection(Selection::Normal(cursor));
                                    editor.action(Action::Motion(Motion::BufferEnd));
                                    editor.action(Action::Delete);
                                }
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
                        Key::Insert => {
                            *overwrite_mode = !*overwrite_mode;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
