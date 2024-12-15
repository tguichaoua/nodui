//! Rendering of node's header.

use std::sync::Arc;

use egui::{
    epaint::{RectShape, TextShape},
    vec2, Color32, FontSelection, Pos2, Rect, Rounding, Vec2,
};

use crate::{Header, TitleHeader};

/* -------------------------------------------------------------------------- */

/// The prepared data to render the header of the node.
pub(crate) struct PreparedHeader {
    /// The prepared header content.
    content: HeaderContent,
    /// The size required to render the header.
    size: Vec2,
}

impl PreparedHeader {
    /// The total size of the header.
    pub(crate) fn size(&self) -> Vec2 {
        self.size
    }

    /// Whether or not the header has content to render.
    pub(crate) fn has_content(&self) -> bool {
        !matches!(self.content, HeaderContent::None)
    }
}

/// The prepared content of the node header.
enum HeaderContent {
    /// No header.
    None,
    /// A [`TitleHeader`].
    Title(TitleHeaderContent),
}

/// The prepared content for a [`TitleHeader`].
struct TitleHeaderContent {
    /// The title text of the header.
    title: Arc<egui::Galley>,
    /// The padding of the header.
    padding: egui::Margin,
    /// The background color of the header.
    background: Color32,
}

/* -------------------------------------------------------------------------- */

/// Do computations to render the header.
pub(crate) fn prepare(ui: &egui::Ui, header: Header, body_color: Color32) -> PreparedHeader {
    match header {
        Header::None => PreparedHeader {
            content: HeaderContent::None,
            size: Vec2::ZERO,
        },
        Header::Title(TitleHeader {
            text,
            background_color: background,
        }) => {
            // TODO: allow user to customize this value ?
            let padding = egui::Margin::same(5.0);

            let background = if background == Color32::PLACEHOLDER {
                body_color
            } else {
                background
            };

            let title = text.into_galley(ui, None, f32::INFINITY, FontSelection::Default);

            let size = padding.sum() + title.rect.size();

            PreparedHeader {
                content: HeaderContent::Title(TitleHeaderContent {
                    title,
                    padding,
                    background,
                }),
                size,
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

impl PreparedHeader {
    /// Render the header.
    pub(in crate::editor) fn show(
        self,
        ui: &egui::Ui,
        pos: Pos2,
        node_size: Vec2,
        rounding: Rounding,
    ) {
        let Self { content, size } = self;

        match content {
            HeaderContent::None => {}
            HeaderContent::Title(TitleHeaderContent {
                title,
                padding,
                background,
            }) => {
                let rect = Rect::from_min_size(pos, vec2(node_size.x, size.y));
                ui.painter()
                    .add(RectShape::filled(rect, rounding, background));

                // TODO: use `title.job` for correct positioning (e.g. halign).
                ui.painter().add(TextShape::new(
                    pos + padding.left_top(),
                    title,
                    ui.visuals().text_color(),
                ));
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
