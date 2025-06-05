## Changelog

### 0.5.0
* Text edits are no longer applied immediately to the text editor buffer immediately on interactions such as key presses or mouse clicks. Instead they are stored in the active text input entity's `TextEditQueue` component and applied all at once in `process_text_edit_queues`. This is to ensure that edits are always applied in order to the currently active text input. Before it was theoretically possible (though unlikely because frames are too short) for actions to be applied to the wrong buffer, or for pasted text to appear after characters entered after the paste.
* Fixed compilation on wasm - thanks to [fallible-algebra](https://github.com/fallible-algebra)
* New resource `TextInputGlobalState` that tracks overwrite mode and the state of the modifier keys.

### 0.4.0
* Improved performance. Text input layouts should only be regenerated after edits now. 
* The `ActiveTextInput` resource is removed. Use `InputFocus` to set the active text input.
* Fixed command binds so that they work when capslock is on.
* `TextInputNode`s are unfocused when despawned.
* Selections are cleared when a `TextInputNode` is unfocused.
* Added clipboard support for wasm32.
* Double-click to select a word.
* Triple-click to select a paragraph.
co-authored by [databasedav](https://github.com/databasedav)

### 0.3.0
* Bevy 0.16 support.

### 0.2.0
* New `line_height` parameter on `TextInputNode`. 