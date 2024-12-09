//! The graph editor.

mod nodes;
mod response;
mod show_connection;
mod show_viewport;
pub mod stages;
mod state;

use egui::{Color32, Id, Stroke, Vec2};

use crate::{
    viewport::{CanvasPos, Grid, Viewport},
    RenderedSocket,
};

pub use nodes::{GraphUi, NodeLayout, NodeResponse, NodeUi};
pub use response::GraphResponse;
pub use show_connection::ConnectionsUi;

use state::EditorState;

/* -------------------------------------------------------------------------- */

/// A node based graph editor.
///
/// The following methods must be called in order:
///
/// - [`new`][Self::new]: Creates the graph editor.
/// - [`show_viewport`][Self::show_viewport]: Render the viewport of the editor.
/// - [`show_nodes`][Self::show_nodes]: Render the nodes.
/// - [`show_connections`][Self::show_connections]: Render the connection between the sockets.
/// - [`finish`][Self::finish]: Returns the [`GraphResponse`].
///
/// ```
/// # #[derive(Clone, PartialEq)]
/// # struct SocketId;
/// fn show_graph_editor(ui: &mut egui::Ui) {
///     let response: nodui::GraphResponse::<SocketId> = nodui::GraphEditor::new("a unique id")
///         .show_viewport(ui)
///         .show_nodes(|ui| {
///             /* This is where you add the nodes */
///         })
///         .show_connections(|ui| {
///             /* This is where you add the connections between sockets */
///         })
///         .finish();
/// }
/// ```
///
pub struct GraphEditor<Stage> {
    /// The id used to store data from one frame to the other.
    id: Id,
    /// The current stage of the editor.
    stage: Stage,
}

impl GraphEditor<stages::Viewport> {
    /// Creates a new [`GraphEditor`].
    #[inline]
    pub fn new(id_salt: impl core::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            stage: stages::Viewport {
                grid_stroke: Stroke::new(0.5, Color32::DARK_GRAY),
                background_color: Color32::BLACK,
                look_at: None,
                width: None,
                height: None,
                view_aspect: None,
                min_size: Vec2::ZERO,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
