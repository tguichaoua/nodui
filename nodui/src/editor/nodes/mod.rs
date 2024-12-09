//! The rendering of the nodes.

mod header;
mod node;
mod render;

use egui::{Id, Vec2};

use crate::misc::collector::Collector;

use super::{RenderedSocket, Viewport};

pub use node::{NodeResponse, NodeUi};

/* -------------------------------------------------------------------------- */

/// This is what you use to render the nodes.
///
/// See [`GraphEditor::show_nodes`].
pub struct GraphUi<S> {
    /// The id of the graph editor.
    pub(super) graph_id: Id,
    /// The id and delta position of the node being dragged, id any.
    pub(super) dragged_node: Option<(Id, Vec2)>,
    /// The viewport of the editor.
    pub(super) viewport: Viewport,
    /// The [`egui::Ui`] used to render the editor.
    pub(super) ui: egui::Ui,
    /// The rendered sockets.
    pub(super) rendered_sockets: Collector<RenderedSocket<S>>,
}

/* -------------------------------------------------------------------------- */
