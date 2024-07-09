//! Implementation of [`nodui::GraphAdapter`].

use nodui::ConnectionHint;

use crate::graph::{NodeId, SocketId};

use super::node::NodeIter;
use super::GraphApp;

/* -------------------------------------------------------------------------- */

impl nodui::GraphAdapter for GraphApp {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn nodes(
        &mut self,
    ) -> impl nodui::NodeIterator<NodeId = Self::NodeId, SocketId = Self::SocketId> {
        NodeIter {
            nodes: self.graph.op_nodes().iter(),
            inputs: self.graph.inputs().iter(),
            connections: self.graph.connections(),

            positions: &mut self.positions,
            selected_node: self.selected_node,
        }
    }

    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> ConnectionHint {
        if crate::graph::Connections::can_connect(a, b) {
            ConnectionHint::Accept
        } else {
            ConnectionHint::Reject
        }
    }

    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) {
        if self.graph.connections_mut().connect(a, b) {
            self.may_need_to_rebuild_expr = true;
        }
    }

    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> {
        self.graph
            .connections()
            .iter()
            .map(|(a, b)| (SocketId::from(a), SocketId::from(b)))
    }
}

/* -------------------------------------------------------------------------- */
