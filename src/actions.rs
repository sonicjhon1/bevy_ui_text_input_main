use bevy::ecs::entity::Entity;
use bevy::ecs::resource::Resource;
use bevy::text::cosmic_text::Action;
use bevy::text::cosmic_text::BorrowedWithFontSystem;
use bevy::text::cosmic_text::Edit;
use bevy::text::cosmic_text::Editor;
use bevy::text::cosmic_text::Motion;
use bevy::text::cosmic_text::Selection;
use std::collections::VecDeque;

use crate::TextInputMode;
use crate::clipboard::ClipboardRead;
use crate::edit::apply_action;
use crate::edit::apply_motion;
use crate::edit::buffer_len;
use crate::edit::cursor_at_line_end;
use crate::edit::filter_text;

/// An action to perform on a [`TextInputEditor`]
#[derive(Debug)]
pub enum TextInputEdit {
    /// Move the cursor with some motion
    Motion(Motion, bool),
    /// Escape, clears selection
    Escape,
    /// Insert character at cursor
    Insert(char, bool),
    /// Create new line
    Enter,
    /// Delete text behind cursor
    Backspace,
    /// Delete text in front of cursor
    Delete,
    // Indent text (typically Tab)
    Indent,
    // Unindent text (typically Shift+Tab)
    Unindent,
    /// Mouse click at specified position
    Click {
        x: i32,
        y: i32,
    },
    /// Mouse double click at specified position
    DoubleClick {
        x: i32,
        y: i32,
    },
    /// Mouse triple click at specified position
    TripleClick {
        x: i32,
        y: i32,
    },
    /// Mouse drag to specified position
    Drag {
        x: i32,
        y: i32,
    },
    /// Scroll specified number of lines
    Scroll {
        lines: i32,
    },
    Paste(String),
    Undo,
    Redo,
    SelectAll,
    ScrollUp,
    ScrollDown,
}

pub enum TextInputAction {
    Submit,
    Copy,
    Paste(ClipboardRead),
    Edit(TextInputEdit),
}

#[derive(Resource, Debug, Default)]
pub struct TextInputActionsQueue(VecDeque<(Entity, TextInputEdit)>);

impl TextInputActionsQueue {
    pub fn push(&mut self, entity: Entity, action: TextInputEdit) {
        self.0.push_back((entity, action));
    }

    pub fn pop(&mut self) -> Option<(Entity, TextInputEdit)> {
        self.0.pop_front()
    }
}

pub fn apply_edit(
    edit: TextInputEdit,
    editor: &mut BorrowedWithFontSystem<'_, Editor<'static>>,
    changes: &mut cosmic_undo_2::Commands<bevy::text::cosmic_text::Change>,
    max_chars: Option<usize>,
    input_mode: &TextInputMode,
) {
    editor.start_change();

    match edit {
        TextInputEdit::Motion(motion, with_select) => {
            apply_motion(editor, with_select, motion);
        }
        TextInputEdit::Escape => {
            editor.action(Action::Escape);
        }
        TextInputEdit::Insert(ch, overwrite) => {
            if editor.selection() != Selection::None {
                editor.action(Action::Insert(ch));
            } else if overwrite && !cursor_at_line_end(editor) {
                editor.action(Action::Delete);
                editor.action(Action::Insert(ch));
            } else if max_chars.is_none_or(|max_chars| editor.with_buffer(buffer_len) < max_chars) {
                editor.action(Action::Insert(ch));
                if let Some(regex) = input_mode.regex() {
                    let text = editor.with_buffer(crate::get_text);
                    if !regex.is_match(&text) {
                        editor.action(Action::Backspace);
                    }
                }
            }
        }
        TextInputEdit::Backspace => {
            if editor.delete_selection() {
                editor.set_redraw(true);
            } else {
                editor.action(Action::Backspace);
            }
        }
        TextInputEdit::Delete => {
            if editor.delete_selection() {
                editor.set_redraw(true);
            } else {
                editor.action(Action::Delete);
            }
        }
        TextInputEdit::Indent => {
            editor.action(Action::Indent);
        }
        TextInputEdit::Unindent => {
            editor.action(Action::Indent);
        }
        TextInputEdit::Click { x, y } => {
            editor.action(Action::Click { x, y });
        }
        TextInputEdit::DoubleClick { x, y } => {
            editor.action(Action::DoubleClick { x, y });
        }
        TextInputEdit::TripleClick { x, y } => {
            editor.action(Action::DoubleClick { x, y });
        }
        TextInputEdit::Drag { x, y } => {
            editor.action(Action::Drag { x, y });
        }
        TextInputEdit::Scroll { lines } => {
            editor.action(Action::Scroll { lines });
        }
        TextInputEdit::Paste(text) => {
            if max_chars.is_none_or(|max| editor.with_buffer(buffer_len) + text.len() <= max) {
                if filter_text(*input_mode, &text) {
                    editor.insert_string(&text, None);
                }
            }
        }
        TextInputEdit::Undo => {
            for action in changes.undo() {
                apply_action(editor, action);
            }
        }
        TextInputEdit::Redo => {
            for action in changes.redo() {
                apply_action(editor, action);
            }
        }
        TextInputEdit::SelectAll => {
            editor.action(Action::Motion(Motion::BufferStart));
            let cursor = editor.cursor();
            editor.set_selection(Selection::Normal(cursor));
            editor.action(Action::Motion(Motion::BufferEnd));
        }
        TextInputEdit::ScrollUp => {
            editor.action(Action::Scroll { lines: -1 });
        }
        TextInputEdit::ScrollDown => {
            editor.action(Action::Scroll { lines: 1 });
        }
        TextInputEdit::Enter => {
            editor.action(Action::Enter);
        }
    }

    if let Some(change) = editor.finish_change() {
        if !change.items.is_empty() {
            changes.push(change);
            editor.set_redraw(true);
        }
    }
}
