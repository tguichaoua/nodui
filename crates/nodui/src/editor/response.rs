//! Graph editor's response.

use crate::{Pos, RenderedSocket, Viewport};

use super::{stages, GraphEditor};

/* -------------------------------------------------------------------------- */

/// The result of rendering a [`GraphEditor`].
pub struct GraphResponse<S> {
    /// The viewport of the editor.
    ///
    /// Use it for coordinates conversion.
    pub viewport: Viewport,
    /// The [`Response`][egui::Response] of the editor.
    pub response: egui::Response,
    /// The sockets that have been rendered.
    pub sockets: Vec<RenderedSocket<S>>,
    /// Whether the user create a new connection.
    pub connection: Option<(S, S)>,
    /// The position of the viewport.
    pub position: Pos,
}

impl<S> GraphEditor<stages::End<S>>
where
    S: Send + Sync + Clone + 'static,
{
    /// End the rendering of the graph editor and returns the [`GraphResponse`].
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

        let position = viewport.grid.canvas_to_graph(state.viewport_position);

        state.store(ui.ctx(), id);

        GraphResponse {
            viewport,
            response,
            sockets,
            connection,
            position,
        }
    }
}

/* -------------------------------------------------------------------------- */
