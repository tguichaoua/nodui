#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

mod header;
mod show_connection;
mod show_node;
mod show_viewport;
mod socket;
pub mod stages;

use egui::{Id, Response, Vec2};

use crate::viewport::{CanvasPos, Grid, Viewport};

pub use show_connection::ConnectionsUi;
pub use show_node::{GraphUi, NodeResponse, NodeUi};
pub use socket::{ConnectionInProgress, RenderedSocket, Socket};

/* -------------------------------------------------------------------------- */

pub struct GraphEditor<Stage> {
    id: Id,
    stage: Stage,
}

pub struct GraphResponse<S> {
    pub viewport: Viewport,
    pub response: Response,
    pub sockets: Vec<RenderedSocket<S>>,
    pub connection: Option<(S, S)>,
}

impl<S> GraphEditor<stages::End<S>>
where
    S: Send + Sync + Clone + 'static,
{
    #[inline]
    pub fn finish(self) -> GraphResponse<S> {
        let Self {
            id,
            stage:
                stages::End {
                    ui,
                    state,
                    viewport,
                    response,
                    sockets,
                    connection,
                },
        } = self;

        state.store(ui.ctx(), id);

        GraphResponse {
            viewport,
            response,
            sockets,
            connection,
        }
    }
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

/// The state of the editor saved from on frame to another.
#[derive(Clone)]
struct EditorState<S> {
    /// The current viewport position.
    viewport_position: CanvasPos,
    /// The grid of the editor.
    grid: Grid,

    /// The node currently being dragged and the delta position form it's current position.
    dragged_node: Option<(Id, Vec2)>,
    /// The socket currently being dragged.
    dragged_socket: Option<S>,
}

impl<S> Default for EditorState<S> {
    fn default() -> Self {
        Self {
            viewport_position: CanvasPos::ZERO,
            grid: Grid { size: 10.0 },
            dragged_node: None,
            dragged_socket: None,
        }
    }
}

impl<S> EditorState<S>
where
    Self: Clone + Send + Sync + 'static,
{
    /// Loads the editor state.
    fn load(ctx: &egui::Context, id: Id) -> Self {
        ctx.data(|data| data.get_temp(id).unwrap_or_default())
    }

    /// Store the editor state.
    fn store(self, ctx: &egui::Context, id: Id) {
        ctx.data_mut(|data| data.insert_temp(id, self));
    }
}

/* -------------------------------------------------------------------------- */
