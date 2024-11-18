//! Implementation of [`nodui::GraphAdapter`].

use either::Either;
use nodui::ConnectionHint;

use crate::graph::{NodeId, SocketId};

use super::node::NodeAdapter;
use super::GraphApp;

/* -------------------------------------------------------------------------- */

impl nodui::GraphAdapter for GraphApp {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn nodes(
        &self,
    ) -> impl Iterator<Item: nodui::NodeAdapter<NodeId = Self::NodeId, SocketId = Self::SocketId>>
    {
        let op_nodes = self
            .graph
            .op_nodes()
            .iter()
            .map(|node| (NodeId::from(node.id()), Either::Left(node)));

        let input_nodes = self
            .graph
            .inputs()
            .iter()
            .map(|node| (NodeId::from(node.id()), Either::Right(node)));

        let connections = self.graph.connections();

        op_nodes.chain(input_nodes).map(|(id, node)| {
            let pos = self.positions.get(&id).copied().unwrap_or_default();
            NodeAdapter {
                pos,
                node,
                connections,
                selected_node: self.selected_node,
            }
        })
    }

    fn set_node_pos(&mut self, node_id: Self::NodeId, pos: nodui::Pos) {
        self.positions.insert(node_id, pos);
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
