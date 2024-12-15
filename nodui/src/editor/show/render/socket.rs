//! Rendering of node's sockets.

use std::sync::Arc;

use egui::{vec2, Color32, FontSelection, Vec2};

use crate::{misc::layout, NodeSide, Socket, SocketShape};

use super::{SOCKET_NAME_GAP, SOCKET_WIDTH};

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

        layout::stack_horizontally([socket_size, socket_text_gap, text_size])
    }
}

/* -------------------------------------------------------------------------- */

/// Do computations to render a socket.
pub(crate) fn prepare<S>(ui: &egui::Ui, socket: Socket<S>) -> PreparedSocket<S> {
    let Socket {
        id,
        side,
        text,
        filled,
        shape,
        mut color,
    } = socket;

    if color == Color32::PLACEHOLDER {
        color = ui.visuals().strong_text_color();
    }

    let layout_job = text.into_layout_job(ui.style(), FontSelection::Default, ui.text_valign());

    let text = ui.fonts(|fonts| {
        fonts.layout_job(egui::text::LayoutJob {
            halign: match side {
                NodeSide::Left => egui::Align::LEFT,
                NodeSide::Right => egui::Align::RIGHT,
            },
            ..layout_job
        })
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
