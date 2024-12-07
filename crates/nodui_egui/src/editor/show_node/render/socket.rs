//! Rendering of node's sockets.

use std::sync::Arc;

use egui::{vec2, Vec2};

use crate::{misc::layout, Socket};

use super::{ROW_HEIGHT, SOCKET_NAME_FIELD_GAP, SOCKET_NAME_GAP, SOCKET_WIDTH};

/* -------------------------------------------------------------------------- */

/// The prepared data for a socket.
pub(crate) struct PreparedSocket<SocketId> {
    /// The unique identifier of the socket.
    pub(super) id: SocketId,
    /// The side on which the socket is rendered.
    pub(super) side: nodui_core::ui::NodeSide,
    /// The text name of the socket.
    pub(super) text: Arc<egui::Galley>,
    /// Whether or not this socket's shape should be filled.
    pub(super) filled: bool,
    /// The color of the socket's handle.
    pub(super) color: egui::Color32,
    /// The shape of the socket's handle.
    pub(super) shape: nodui_core::ui::SocketShape,
}

impl<S> PreparedSocket<S> {
    /// Compute the size the socket will occupied.
    pub(super) fn compute_size(&self) -> Vec2 {
        let socket_size = Vec2::splat(SOCKET_WIDTH);
        let socket_text_gap = vec2(SOCKET_NAME_GAP, 0.0);
        let text_size = self.text.size();
        let text_field_gap = vec2(SOCKET_NAME_FIELD_GAP, 0.0);

        let mut size =
            layout::stack_horizontally([socket_size, socket_text_gap, text_size, text_field_gap]);

        // FIXME: it work will the computed height is lower than `ROW_HEIGHT`.
        size.y = ROW_HEIGHT;

        size
    }
}

/* -------------------------------------------------------------------------- */

/// Do computations to render a socket.
pub(crate) fn prepare<S>(socket: Socket<S>, fonts: &egui::text::Fonts) -> PreparedSocket<S> {
    let Socket {
        id,
        side,
        text,
        text_color,
        filled,
        shape,
        color,
    } = socket;

    let text = fonts.layout_job(egui::text::LayoutJob {
        halign: match side {
            nodui_core::ui::NodeSide::Left => egui::Align::LEFT,
            nodui_core::ui::NodeSide::Right => egui::Align::RIGHT,
        },
        ..egui::text::LayoutJob::simple_singleline(text, egui::FontId::monospace(12.0), text_color)
    });

    PreparedSocket {
        id,
        side,
        text,
        filled,
        color,
        shape,
    }
}

/* -------------------------------------------------------------------------- */
