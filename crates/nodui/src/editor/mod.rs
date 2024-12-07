//! The graph editor.

mod response;
mod show_connection;
mod show_node;
mod show_viewport;
pub mod stages;
mod state;

use egui::{Id, Vec2};

use crate::{
    viewport::{CanvasPos, Grid, Viewport},
    RenderedSocket,
};

pub use response::GraphResponse;
pub use show_connection::ConnectionsUi;
pub use show_node::{GraphUi, NodeLayout, NodeResponse, NodeUi};

use state::EditorState;

/* -------------------------------------------------------------------------- */

/// A node based graph editor.
pub struct GraphEditor<Stage> {
    /// The id used to store data from one frame to the other.
    id: Id,
    /// The current stage of the editor.
    stage: Stage,
}

impl GraphEditor<stages::Viewport> {
    /// Creates a new [`GraphEditor_`].
    #[inline]
    pub fn new(id_salt: impl core::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            stage: stages::Viewport {
                width: None,
                height: None,
                view_aspect: None,
                min_size: Vec2::ZERO,
            },
        }
    }

    // TODO: add methods to defines the viewport size.
}

/* -------------------------------------------------------------------------- */
