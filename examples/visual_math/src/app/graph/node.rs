//! Implementation of the nodui adapter traits for the nodes.

use core::slice;
use std::collections::HashMap;

use either::Either;
use nodui::ui::{Color, NodeUI};
use nodui::Pos;

use crate::graph::{Connections, Input, NodeId, OpNode, SocketId};

use super::socket::SocketIter;

/* -------------------------------------------------------------------------- */

/// An iterator over the node of the math graph.
pub(super) struct NodeIter<'a> {
    /// The operation nodes of the graph.
    pub(super) nodes: slice::Iter<'a, OpNode>,
    /// The input of the graph.
    pub(super) inputs: slice::Iter<'a, Input>,
    /// The connections of the graph.
    pub(super) connections: &'a Connections,
    /// The positions of the nodes.
    pub(super) positions: &'a mut HashMap<NodeId, Pos>,
    /// The currently selected node.
    pub(super) selected_node: Option<NodeId>,
}

impl nodui::NodeIterator for NodeIter<'_> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    type Item<'this> = NodeAdapter<'this>
    where
        Self: 'this;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if let Some(node) = self.nodes.next() {
            let pos = self.positions.entry(node.id().into()).or_default();
            return Some(NodeAdapter {
                pos,
                node: Either::Left(node),
                connections: self.connections,
                selected_node: self.selected_node,
            });
        }

        self.inputs.next().map(|input| {
            let pos = self.positions.entry(input.id().into()).or_default();
            NodeAdapter {
                pos,
                node: Either::Right(input),
                connections: self.connections,
                selected_node: self.selected_node,
            }
        })
    }
}

/* -------------------------------------------------------------------------- */

/// An adapter for a node of the math graph.
pub struct NodeAdapter<'a> {
    /// The current position of this node.
    pos: &'a mut Pos,
    /// The graph node.
    node: Either<&'a OpNode, &'a Input>,
    /// The connections of the graph.
    connections: &'a Connections,
    /// The currently selected node.
    selected_node: Option<NodeId>,
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
        *self.pos
    }

    fn set_pos(&mut self, pos: Pos) {
        *self.pos = pos;
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
