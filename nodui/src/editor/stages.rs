//! Stages of [`GraphEditor`](super::GraphEditor).

use crate::{misc::viewport::ViewportSize, Pos};

/// Render the viewport of the graph editor.
pub struct Settings {
    /// The stroke of the editor's grid.
    pub(super) grid_stroke: egui::Stroke,
    /// The color of the background.
    pub(super) background_color: egui::Color32,
    /// Sets the position of the viewport.
    pub(super) look_at: Option<Pos>,
    /// Can the user drag a socket to start a new connection.
    pub(super) can_connect_socket: bool,
    /// The size of the viewport
    pub(super) viewport: ViewportSize,
}

/// Render the connections.
pub struct Connections<S> {
    /// The [`egui::Ui`] used to render the editor.
    pub(super) ui: egui::Ui,
    /// The state of the editor.
    pub(super) state: super::EditorState<S>,
    /// The viewport used for coordinates conversions.
    pub(super) viewport: super::Viewport,
    /// The response of the editor.
    pub(super) response: egui::Response,
    /// The sockets that have been rendered.
    pub(super) sockets: Vec<super::RenderedSocket<S>>,
    /// A user interaction with the sockets.
    pub(super) socket_interaction: crate::socket::SocketInteraction<S>,
}
