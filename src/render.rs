use bevy::asset::AssetId;
use bevy::asset::Assets;
use bevy::color::Alpha;
use bevy::color::LinearRgba;
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::math::Mat4;
use bevy::math::Rect;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::render::Extract;
use bevy::render::sync_world::RenderEntity;
use bevy::render::sync_world::TemporaryRenderEntity;
use bevy::render::view::ViewVisibility;
use bevy::sprite::BorderRect;
use bevy::sprite::TextureAtlasLayout;
use bevy::text::TextColor;
use bevy::text::cosmic_text::Edit;
use bevy::transform::components::GlobalTransform;
use bevy::ui::CalculatedClip;
use bevy::ui::ComputedNode;
use bevy::ui::DefaultUiCamera;
use bevy::ui::ExtractedGlyph;
use bevy::ui::ExtractedUiItem;
use bevy::ui::ExtractedUiNode;
use bevy::ui::ExtractedUiNodes;
use bevy::ui::NodeType;
use bevy::ui::ResolvedBorderRadius;
use bevy::ui::TargetCamera;

use crate::TextCursorWidth;
use crate::TextInputBuffer;
use crate::TextInputGlyph;
use crate::TextInputLayoutInfo;
use crate::TextInputStyle;

pub fn extract_text_input_nodes(
    mut commands: Commands,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &GlobalTransform,
            &ViewVisibility,
            Option<&CalculatedClip>,
            Option<&TargetCamera>,
            &TextInputLayoutInfo,
            &TextColor,
            &TextInputStyle,
            &TextInputBuffer,
        )>,
    >,
    mapping: Extract<Query<&RenderEntity>>,
    default_ui_camera: Extract<DefaultUiCamera>,
) {
    let mut start = extracted_uinodes.glyphs.len();
    let mut end = start + 1;

    let default_ui_camera = default_ui_camera.get();
    for (
        entity,
        uinode,
        global_transform,
        view_visibility,
        clip,
        camera,
        text_layout_info,
        text_color,
        style,
        input,
    ) in &uinode_query
    {
        let Some(camera_entity) = camera.map(TargetCamera::entity).or(default_ui_camera) else {
            continue;
        };

        // Skip if not visible or if size is set to zero (e.g. when a parent is set to `Display::None`)
        if !view_visibility.get() || uinode.is_empty() {
            continue;
        }

        let Ok(&render_camera_entity) = mapping.get(camera_entity) else {
            continue;
        };

        let color = text_color.0.to_linear();
        let selection_color = style
            .selected_text_color
            .map(|selection_color| selection_color.to_linear())
            .unwrap_or(color);

        let sx = input
            .editor
            .with_buffer(|buffer| buffer.scroll().horizontal);

        let transform = global_transform.affine()
            * bevy::math::Affine3A::from_translation(
                (-0.5 * uinode.size() - sx * Vec2::X).extend(0.),
            );

        let node_rect = Rect::from_center_size(
            global_transform.translation().truncate(),
            uinode.size() * global_transform.scale().truncate(),
        );

        let clip = Some(
            clip.map(|clip| clip.clip.intersect(node_rect))
                .unwrap_or(node_rect),
        );

        let line_height = input
            .editor
            .with_buffer(|buffer| buffer.metrics().line_height);

        for rect in &input.selection_rects {
            let id = commands.spawn(TemporaryRenderEntity).id();
            extracted_uinodes.uinodes.insert(
                id,
                ExtractedUiNode {
                    stack_index: uinode.stack_index(),
                    color: LinearRgba::from(style.selection_color),
                    image: AssetId::default(),
                    clip,
                    camera_entity: render_camera_entity.id(),
                    rect: Rect {
                        min: Vec2::ZERO,
                        max: Vec2::new(rect.width(), rect.height()),
                    },
                    item: ExtractedUiItem::Node {
                        atlas_scaling: None,
                        flip_x: false,
                        flip_y: false,
                        border_radius: ResolvedBorderRadius {
                            top_left: 5.,
                            top_right: 5.,
                            bottom_left: 5.,
                            bottom_right: 5.,
                        },
                        border: BorderRect::ZERO,
                        node_type: NodeType::Rect,
                        transform: transform * Mat4::from_translation(rect.center().extend(0.)),
                    },
                    main_entity: entity.into(),
                },
            );
        }

        let selection = input.editor.selection_bounds();

        for TextInputGlyph {
            position,
            atlas_info,

            line_index,
            byte_index,
            ..
        } in text_layout_info.glyphs.iter()
        {
            let color_out = if let Some((s0, s1)) = selection {
                if (s0.line < *line_index || (*line_index == s0.line && s0.index <= *byte_index))
                    && (*line_index < s1.line || (*line_index == s1.line && *byte_index < s1.index))
                {
                    selection_color
                } else {
                    color
                }
            } else {
                color
            };
            let rect = texture_atlases
                .get(&atlas_info.texture_atlas)
                .unwrap()
                .textures[atlas_info.location.glyph_index]
                .as_rect();
            extracted_uinodes.glyphs.push(ExtractedGlyph {
                transform: transform * Mat4::from_translation(position.extend(0.)),
                rect,
            });
            extracted_uinodes.uinodes.insert(
                commands.spawn(TemporaryRenderEntity).id(),
                ExtractedUiNode {
                    stack_index: uinode.stack_index(),
                    color: color_out,
                    image: atlas_info.texture.id(),
                    clip,
                    rect,
                    item: ExtractedUiItem::Glyphs {
                        range: start..end,
                        atlas_scaling: Vec2::ONE,
                    },
                    main_entity: entity.into(),
                    camera_entity: render_camera_entity.id(),
                },
            );

            start = end;
            end += 1;
        }

        if style.blink_interval < input.cursor_blink_time {
            continue;
        }

        if style.cursor_color.is_fully_transparent() {
            continue;
        }

        let cursor_height = line_height * style.cursor_height;

        let Some((x, y)) = input.editor.cursor_position() else {
            continue;
        };

        let scale_factor = uinode.inverse_scale_factor().recip();
        let width = match style.cursor_width {
            TextCursorWidth::Block => 3. * scale_factor,
            TextCursorWidth::Line(width) => width * scale_factor,
        };

        let id = commands.spawn(TemporaryRenderEntity).id();

        extracted_uinodes.uinodes.insert(
            id,
            ExtractedUiNode {
                stack_index: uinode.stack_index(),
                color,
                image: AssetId::default(),
                clip,
                camera_entity: render_camera_entity.id(),
                rect: Rect {
                    min: Vec2::ZERO,
                    max: Vec2::new(width, cursor_height),
                },
                item: ExtractedUiItem::Node {
                    atlas_scaling: None,
                    flip_x: false,
                    flip_y: false,
                    border_radius: ResolvedBorderRadius::ZERO,
                    border: BorderRect::ZERO,
                    node_type: NodeType::Rect,
                    transform: transform
                        * Mat4::from_translation(Vec3::new(
                            x as f32,
                            y as f32 + line_height * 0.5,
                            0.,
                        )),
                },
                main_entity: entity.into(),
            },
        );

        // extracted_uinodes.uinodes.push(ExtractedUiNode {
        //     render_entity: commands.spawn(TemporaryRenderEntity).id(),
        //     stack_index: uinode.stack_index,
        //     color: LinearRgba::from(style.cursor_color),
        //     image: AssetId::default(),
        //     clip,
        //     extracted_camera_entity,
        //     rect: Rect {
        //         min: Vec2::ZERO,
        //         max: Vec2::new(width, cursor_height),
        //     },
        //     item: ExtractedUiItem::Node {
        //         atlas_scaling: None,
        //         flip_x: false,
        //         flip_y: false,
        //         border_radius: ResolvedBorderRadius {
        //             top_left: style.radius * target.scale_factor,
        //             top_right: style.radius * target.scale_factor,
        //             bottom_left: style.radius * target.scale_factor,
        //             bottom_right: style.radius * target.scale_factor,
        //         },
        //         border: BorderRect::ZERO,
        //         node_type: NodeType::Rect,
        //         transform: transform
        //             * Mat4::from_translation(Vec3::new(x as f32, y as f32 + line_height * 0.5, 0.)),
        //     },
        //     main_entity: entity.into(),
        // });
    }
}
