//! Stages of [`GraphEditor`](super::GraphEditor).

use crate::Pos;

/// Render the viewport of the graph editor.
pub struct Settings {
    /// The stroke of the editor's grid.
    pub(super) grid_stroke: egui::Stroke,
    /// The color of the background.
    pub(super) background_color: egui::Color32,
    /// Sets the position of the viewport.
    pub(super) look_at: Option<Pos>,
    /// The desired width of the viewport.
    pub(super) width: Option<f32>,
    /// The desired height of the viewport.
    pub(super) height: Option<f32>,
    /// The desired aspect ratio of the viewport.
    pub(super) view_aspect: Option<f32>,
    /// The minimum size of the viewport.
    pub(super) min_size: egui::Vec2,
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

/// The final stage, ready to be [`finished`](super::GraphEditor::finish).
pub struct End<S> {
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
    /// Whether the user create a new connection between two sockets.
    pub(super) connection: Option<(S, S)>,
}
