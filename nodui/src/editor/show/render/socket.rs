//! Rendering of node's sockets.

use std::sync::Arc;

use egui::{vec2, Color32, Vec2};

use crate::{misc::layout, NodeSide, Socket, SocketShape};

use super::{ROW_HEIGHT, SOCKET_NAME_GAP, SOCKET_WIDTH};

/* -------------------------------------------------------------------------- */

/// The prepared data for a socket.
pub(crate) struct PreparedSocket<SocketId> {
    /// The unique identifier of the socket.
    pub(super) id: SocketId,
    /// The side on which the socket is rendered.
    pub(super) side: NodeSide,
    /// The text name of the socket.
    pub(super) text: Arc<egui::Galley>,
    /// Whether or not this socket's shape should be filled.
    pub(super) filled: bool,
    /// The color of the socket's handle.
    pub(super) color: Color32,
    /// The shape of the socket's handle.
    pub(super) shape: SocketShape,
}

impl<S> PreparedSocket<S> {
    /// Compute the size the socket will occupied.
    pub(super) fn compute_size(&self) -> Vec2 {
        let socket_size = Vec2::splat(SOCKET_WIDTH);
        let socket_text_gap = vec2(SOCKET_NAME_GAP, 0.0);
        let text_size = self.text.size();

        let mut size = layout::stack_horizontally([socket_size, socket_text_gap, text_size]);

        // FIXME: it work will the computed height is lower than `ROW_HEIGHT`.
        size.y = ROW_HEIGHT;

        size
    }
}

/* -------------------------------------------------------------------------- */

/// Do computations to render a socket.
pub(crate) fn prepare<S>(
    socket: Socket<S>,
    visuals: &egui::Visuals,
    fonts: &egui::text::Fonts,
) -> PreparedSocket<S> {
    let Socket {
        id,
        side,
        text,
        mut text_color,
        filled,
        shape,
        mut color,
    } = socket;

    if text_color == Color32::PLACEHOLDER {
        text_color = visuals.strong_text_color();
    }

    if color == Color32::PLACEHOLDER {
        color = text_color;
    }

    let text = fonts.layout_job(egui::text::LayoutJob {
        halign: match side {
            NodeSide::Left => egui::Align::LEFT,
            NodeSide::Right => egui::Align::RIGHT,
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
