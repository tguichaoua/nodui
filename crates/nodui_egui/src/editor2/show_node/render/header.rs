use std::sync::Arc;

use egui::{
    epaint::{RectShape, TextShape},
    vec2, Color32, Pos2, Rect, Rounding, Vec2,
};

use crate::editor2::header::{Header, TitleHeader};

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

pub(crate) fn prepare(header: Header, fonts: &egui::text::Fonts) -> PreparedHeader {
    match header {
        Header::None => PreparedHeader {
            content: HeaderContent::None,
            size: Vec2::ZERO,
        },
        Header::Title(TitleHeader {
            text,
            text_color,
            background,
        }) => {
            // TODO: allow user to customize this value ?
            let padding = egui::Margin::same(5.0);

            let title = fonts.layout_job(egui::text::LayoutJob {
                halign: egui::Align::LEFT,
                ..egui::text::LayoutJob::simple_singleline(
                    text,
                    egui::FontId::monospace(12.0),
                    text_color,
                )
            });

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
    pub(in crate::editor2) fn show(
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

                ui.painter().add(TextShape::new(
                    pos + padding.left_top(),
                    title,
                    Color32::WHITE,
                ));
            }
        }
    }
}

/* -------------------------------------------------------------------------- */
