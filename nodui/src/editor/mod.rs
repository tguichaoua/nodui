//! The graph editor.

mod connections;
mod response;
mod show;
pub mod stages;
mod state;

use egui::Id;

use crate::{
    misc::viewport::ViewportSize,
    viewport::{CanvasPos, Grid, Viewport},
    Pos, RenderedSocket,
};

pub use connections::ConnectionsUi;
pub use response::GraphResponse;
pub use show::{GraphUi, NodeResponse, NodeUi};

use state::EditorState;

/* -------------------------------------------------------------------------- */

/// A node based graph editor.
///
/// The following methods must be called in order:
///
/// - [`new`][Self::new]: Creates the graph editor.
/// - [`show`][Self::show]: Allocates the space in the ui and renders the viewport and the nodes.
/// - [`show_connections`][Self::show_connections]: Renders the connections between the sockets and returns the [`GraphResponse`].
///
/// ```
/// # #[derive(Clone, PartialEq)]
/// # struct SocketId;
/// fn show_graph_editor(ui: &mut egui::Ui) {
///     let response: nodui::GraphResponse::<SocketId> = nodui::GraphEditor::new("a unique id")
///         .show(ui, |ui| {
///             /* This is where you add the nodes */
///         })
///         .show_connections(|ui| {
///             /* This is where you add the connections between sockets */
///         });
/// }
/// ```
pub struct GraphEditor<Stage> {
    /// The id used to store data from one frame to the other.
    id: Id,
    /// The current stage of the editor.
    stage: Stage,
}

impl GraphEditor<stages::Settings> {
    /// Creates a new [`GraphEditor`].
    #[inline]
    pub fn new(id_salt: impl core::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            stage: stages::Settings {
                show_grid: true,
                look_at: None,
                can_connect_socket: true,
                viewport: ViewportSize::default(),
            },
        }
    }

    /// Move the viewport to make `pos` on the center of the viewport.
    #[inline]
    #[must_use]
    pub fn look_at(mut self, pos: Pos) -> Self {
        self.stage.look_at = Some(pos);
        self
    }

    /// Should show the grid.
    ///
    /// Default to `true`.
    #[inline]
    #[must_use]
    pub fn show_grid(mut self, show_grid: bool) -> Self {
        self.stage.show_grid = show_grid;
        self
    }

    /// If `true` the user can drag-n-drop a socket to create a connection.
    ///
    /// Can be useful to prevent user to edit the graph.
    ///
    /// Default to `true`.
    #[inline]
    #[must_use]
    pub fn can_connect_socket(mut self, can_connect_socket: bool) -> Self {
        self.stage.can_connect_socket = can_connect_socket;
        self
    }

    /// `width / height` ratio of the editor region.
    ///
    /// By default no fixed aspect ratio is set (and width/height will fill the ui it is in).
    #[inline]
    #[must_use]
    pub fn view_aspect(mut self, view_aspect: f32) -> Self {
        self.stage.viewport = self.stage.viewport.view_aspect(view_aspect);
        self
    }

    /// Width of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the width can be calculated from the height.
    #[inline]
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.stage.viewport = self.stage.viewport.width(width);
        self
    }

    /// Height of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the height can be calculated from the width.
    #[inline]
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.stage.viewport = self.stage.viewport.height(height);
        self
    }
}

/* -------------------------------------------------------------------------- */
