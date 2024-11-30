use nodui_core::Pos;

use crate::{RenderedSocket, Viewport};

use super::{stages, GraphEditor};

/* -------------------------------------------------------------------------- */

pub struct GraphResponse<S> {
    pub viewport: Viewport,
    pub response: egui::Response,
    pub sockets: Vec<RenderedSocket<S>>,
    pub connection: Option<(S, S)>,
    pub position: Pos,
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
