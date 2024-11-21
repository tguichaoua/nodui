use std::sync::Arc;

use egui::{
    epaint::{RectShape, TextShape},
    text::{Fonts, LayoutJob},
    vec2, Align, Color32, FontId, Galley, Margin, Pos2, Rect, Rounding, Ui, Vec2,
};
use nodui_core::ui::{NodeHeader, TitleHeader};

use super::IntoEgui;

/// The prepared data to render the header of the node.
pub(super) struct PreparedHeader {
    /// The prepared header content.
    content: HeaderContent,
    /// The size required to render the header.
    size: Vec2,
}

impl PreparedHeader {
    pub(super) fn size(&self) -> Vec2 {
        self.size
    }

    pub(super) fn has_content(&self) -> bool {
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
    title: Arc<Galley>,
    /// The padding of the header.
    padding: Margin,
    /// The background color of the header.
    background: Color32,
}

/// Prepare the header for its rendering.
pub(crate) fn prepare(fonts: &Fonts, header: NodeHeader) -> PreparedHeader {
    match header {
        NodeHeader::None => PreparedHeader {
            content: HeaderContent::None,
            size: Vec2::ZERO,
        },
        NodeHeader::Title(TitleHeader { title, background }) => {
            // TODO: allow user to customize this value ?
            let padding = Margin::same(5.0);

            let background = background.into_egui();

            let title = fonts.layout_job(LayoutJob {
                halign: Align::LEFT,
                ..LayoutJob::simple_singleline(
                    title.text,
                    FontId::monospace(12.0),
                    title.color.into_egui().unwrap_or(Color32::WHITE),
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

impl PreparedHeader {
    pub(super) fn show(self, ui: &Ui, pos: Pos2, node_size: Vec2, rounding: Rounding) {
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
