use std::collections::VecDeque;

use bevy::ecs::entity::Entity;
use bevy::ecs::resource::Resource;
use bevy::text::cosmic_text::Motion;

use crate::clipboard::ClipboardRead;

/// An action to perform on a [`TextInputEditor`]
#[derive(Debug)]
pub enum TextInputAction {
    /// Move the cursor with some motion
    Motion(Motion),
    /// Escape, clears selection
    Escape,
    /// Insert character at cursor
    Insert(char),
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
    Copy,
    Paste(ClipboardRead),
    Undo,
    Redo,
    SelectAll,
    ScrollUp,
    ScrollDown,
}

#[derive(Resource, Debug, Default)]
pub struct TextInputActionsQueue(VecDeque<(Entity, TextInputAction)>);

impl TextInputActionsQueue {
    pub fn push(&mut self, entity: Entity, action: TextInputAction) {
        self.0.push_back((entity, action));
    }

    pub fn pop(&mut self) -> Option<(Entity, TextInputAction)> {
        self.0.pop_front()
    }
}
