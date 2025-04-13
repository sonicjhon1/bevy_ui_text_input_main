mod edit;
mod render;
mod text_input_pipeline;

use bevy::app::{Plugin, PostUpdate};
use bevy::asset::AssetEvents;
use bevy::color::Color;
use bevy::color::palettes::css::{ALICE_BLUE, SKY_BLUE};
use bevy::color::palettes::tailwind::GRAY_400;
use bevy::ecs::component::Component;
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::math::{Rect, Vec2};
use bevy::prelude::ReflectComponent;
use bevy::reflect::{Reflect, std_traits::ReflectDefault};
use bevy::render::{ExtractSchedule, RenderApp};
use bevy::text::TextColor;
use bevy::text::cosmic_text::{Buffer, Edit, Editor, Metrics};
use bevy::text::{GlyphAtlasInfo, TextFont};
use bevy::ui::{Node, RenderUiSystem, UiSystem, extract_text_sections};
use edit::text_input_edit_system;
use render::extract_text_input_nodes;
use text_input_pipeline::{
    TextInputPipeline, remove_dropped_font_atlas_sets_from_text_input_pipeline, text_input_system,
};
pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<TextInputPipeline>().add_systems(
            PostUpdate,
            (
                remove_dropped_font_atlas_sets_from_text_input_pipeline.before(AssetEvents),
                (text_input_edit_system, text_input_system)
                    .chain()
                    .in_set(UiSystem::PostLayout),
            ),
        );

        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_systems(
            ExtractSchedule,
            extract_text_input_nodes
                .in_set(RenderUiSystem::ExtractText)
                .after(extract_text_sections),
        );
    }
}

#[derive(Component, Debug)]
#[require(Node, TextFont, TextInputLayoutInfo, TextInputStyle, TextColor)]
pub struct TextInputNode {
    set_text: Option<String>,
    pub(crate) editor: Editor<'static>,
    needs_update: bool,
    changed: bool,
    max_chars: Option<usize>,
    allow_newline: bool,
    pub(crate) selection_rects: Vec<Rect>,
    pub cursor_blink_time: f32,
}

impl Default for TextInputNode {
    fn default() -> Self {
        Self {
            set_text: None,
            editor: Editor::new(Buffer::new_empty(Metrics::new(20.0, 20.0))),
            needs_update: true,
            changed: false,
            max_chars: None,
            allow_newline: true,
            selection_rects: vec![],
            cursor_blink_time: 0.,
        }
    }
}

/// Styling for a text cursor
#[derive(Component, Copy, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component, Default, Debug, PartialEq)]
pub struct TextInputStyle {
    /// Color of the cursor
    pub cursor_color: Color,
    /// Selection color
    pub selection_color: Color,
    /// Selected text tint, if unset uses the `TextColor`
    pub selected_text_color: Option<Color>,
    /// Width of the cursor
    pub width: TextCursorWidth,
    /// Corner radius in logical pixels
    pub radius: f32,
    /// Normalized height of the cursor relative to the text block's line height.
    pub height: f32,
    /// Time cursor blinks in seconds
    pub blink_interval: f32,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            cursor_color: GRAY_400.into(),
            selection_color: SKY_BLUE.into(),
            selected_text_color: Some(ALICE_BLUE.into()),
            width: TextCursorWidth::Line(3.),
            radius: 0.,
            height: 1.,
            blink_interval: 0.5,
        }
    }
}

impl TextInputNode {
    pub fn get_text(&self) -> String {
        self.editor.with_buffer(|buffer| get_text(buffer))
    }
}

fn get_text(buffer: &Buffer) -> String {
    buffer
        .lines
        .iter()
        .map(|buffer_line| buffer_line.text())
        .fold(String::new(), |mut out, line| {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(line);
            out
        })
}

/// Width of the text cursor
#[derive(Copy, Clone, Debug, PartialEq, Reflect)]
#[reflect(Default, Debug, PartialEq)]
pub enum TextCursorWidth {
    /// Cursor is a block covering the glyph
    Block,
    /// Cursor is a vertical line, the associated value is the line's width in logical pixels
    Line(f32),
}

impl Default for TextCursorWidth {
    fn default() -> Self {
        Self::Line(3.)
    }
}

#[derive(Component, Clone, Default, Debug, Reflect)]
#[reflect(Component, Default, Debug)]
pub struct TextInputLayoutInfo {
    pub glyphs: Vec<TextInputGlyph>,
    pub size: Vec2,
}

#[derive(Debug, Clone, Reflect)]
pub struct TextInputGlyph {
    pub position: Vec2,
    pub size: Vec2,
    pub atlas_info: GlyphAtlasInfo,
    pub span_index: usize,
    pub line_index: usize,
    pub byte_index: usize,
    pub byte_length: usize,
}
