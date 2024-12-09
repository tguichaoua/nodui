//! The rendering of the nodes.

mod header;
mod node;
mod render;

use egui::{Id, Vec2};

use crate::misc::collector::Collector;

use super::{stages, GraphEditor, RenderedSocket, Viewport};

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

impl<S> GraphEditor<stages::Nodes<S>>
where
    S: Clone + PartialEq,
{
    /// Render the nodes.
    #[inline]
    pub fn show_nodes(
        self,
        build_fn: impl FnOnce(&mut GraphUi<S>),
    ) -> GraphEditor<stages::Connections<S>> {
        let Self {
            id,
            stage:
                stages::Nodes {
                    ui,
                    mut state,
                    viewport,
                    response,
                },
        } = self;

        // TODO
        let can_connect_socket = true;

        let mut graph_ui = GraphUi {
            ui,
            graph_id: id,
            dragged_node: state.dragged_node,
            viewport,
            rendered_sockets: Collector::new(),
        };

        build_fn(&mut graph_ui);

        let GraphUi {
            graph_id: _,
            dragged_node,
            viewport,
            ui,
            rendered_sockets,
        } = graph_ui;

        state.dragged_node = dragged_node;

        let sockets = rendered_sockets.into_vec();

        let socket_interaction = if can_connect_socket {
            crate::socket::handle_socket_responses(&mut state.dragged_socket, &sockets)
        } else {
            // Stop the currently dragged socket if creating connection is disabled.
            state.dragged_socket = None;
            crate::socket::SocketInteraction::None
        };

        GraphEditor {
            id,
            stage: stages::Connections {
                ui,
                state,
                viewport,
                response,
                sockets,
                socket_interaction,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
