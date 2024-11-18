//! Implementation of the nodui adapter traits for the nodes.

use either::Either;
use nodui::ui::{Color, NodeUI};
use nodui::Pos;

use crate::graph::{Connections, Input, NodeId, OpNode, SocketId};

use super::socket::SocketIter;

/* -------------------------------------------------------------------------- */

/// An adapter for a node of the math graph.
pub(super) struct NodeAdapter<'a> {
    /// The current position of this node.
    pub(super) pos: Pos,
    /// The graph node.
    pub(super) node: Either<&'a OpNode, &'a Input>,
    /// The connections of the graph.
    pub(super) connections: &'a Connections,
    /// The currently selected node.
    pub(super) selected_node: Option<NodeId>,
}

impl nodui::NodeAdapter for NodeAdapter<'_> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn sockets(&self) -> impl Iterator<Item: nodui::SocketAdapter<SocketId = Self::SocketId>> {
        SocketIter {
            connections: self.connections,
            node: self.node,
            socket_index: 0,
        }
    }

    fn id(&self) -> Self::NodeId {
        either::for_both!(self.node, node => node.id().into())
    }

    fn pos(&self) -> Pos {
        self.pos
    }

    fn ui(&self) -> NodeUI {
        let ui = NodeUI::default();

        // Add a red outline if the node is selected.
        if self.selected_node == Some(self.id()) {
            ui.with_outline((2.0, Color::RED))
        } else {
            ui.with_outline((1.0, Color::WHITE))
        }
    }
}

/* -------------------------------------------------------------------------- */
