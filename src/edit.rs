use arboard::Clipboard;
use bevy::ecs::event::EventReader;
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

use crate::TextInputNode;
use crate::TextInputStyle;
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

pub fn text_input_edit_system(
    mut shift_pressed: Local<bool>,
    mut command_pressed: Local<bool>,
    time: Res<Time>,
    mut character_events: EventReader<KeyboardInput>,
    mut query: Query<(&mut TextInputNode, &TextInputStyle)>,
    mut text_input_pipeline: ResMut<TextInputPipeline>,
) {
    let mut clipboard = Clipboard::new();
    let mut font_system = &mut text_input_pipeline.font_system;

    let Ok((mut text_input, cursor_style)) = query.get_single_mut() else {
        return;
    };

    text_input.changed = false;

    let mut editor = text_input.editor.borrow_with(&mut font_system);
    let mut flag = false;

    for event in character_events.read() {
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
                                            editor.insert_string(&text, None);
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
                        if let Some(char) = str.chars().next() {
                            println!("{char}");
                            editor.action(Action::Insert(char));
                        }
                    }
                    Key::Space => {
                        editor.action(Action::Insert(' '));
                    }
                    Key::Enter => {
                        editor.action(Action::Enter);
                    }
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

    text_input.changed = flag;
    if cursor_style.blink_interval * 2. <= text_input.cursor_blink_time || text_input.changed {
        text_input.cursor_blink_time = 0.;
    } else {
        text_input.cursor_blink_time += time.delta_secs();
    }
}
