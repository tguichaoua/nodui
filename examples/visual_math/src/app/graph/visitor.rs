//! Implementation of the `GraphAdapter` trait.

use nodui::{
    ui::{Color, NodeSide, NodeUI},
    visitor::{self, NodeSeq, SizeHint, SocketData, SocketSeq},
    Pos,
};

use crate::graph::{BinaryOp, Connections, Input, NodeId, Op, OpNode, SocketId, UnaryOp};

use super::GraphApp;

/// An adapter for a node of the math graph.
struct NodeAdapter<'a, Node> {
    /// The current position of this node.
    pos: &'a mut Pos,
    /// The graph node.
    node: &'a Node,
    /// The connections of the graph.
    connections: &'a Connections,
    /// The currently selected node.
    selected_node: Option<NodeId>,
}

impl visitor::GraphAdapter for GraphApp {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn accept<'graph, V>(&'graph mut self, mut visitor: V)
    where
        V: visitor::GraphVisitor<'graph, Self::NodeId, Self::SocketId>,
    {
        let connections = self.graph.connections();
        let selected_node = self.selected_node;

        {
            let op_nodes = self.graph.op_nodes();
            let mut node_seq = visitor.nodes(SizeHint::of(op_nodes));

            for node in op_nodes {
                let pos = self.positions.entry(node.id().into()).or_default();

                node_seq.visit_node(NodeAdapter {
                    pos,
                    node,
                    connections,
                    selected_node,
                });
            }
        }

        {
            let inputs = self.graph.inputs();
            let mut node_seq = visitor.nodes(SizeHint::of(inputs));

            for node in inputs {
                let pos = self.positions.entry(node.id().into()).or_default();

                node_seq.visit_node(NodeAdapter {
                    pos,
                    node,
                    connections,
                    selected_node,
                });
            }
        }
    }
}

impl visitor::NodeAdapter for NodeAdapter<'_, OpNode> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn id(&self) -> Self::NodeId {
        self.node.id().into()
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

    fn accept<'node, V>(&'node mut self, mut visitor: V)
    where
        V: visitor::NodeVisitor<'node, Self::SocketId>,
    {
        let input_sockets = self.node.input_socket_ids();

        let mut socket_seq =
            visitor.sockets(SizeHint::of_iter(&input_sockets) + SizeHint::exact(1));

        for socket in input_sockets {
            socket_seq.visit_socket(
                SocketData::new(socket.into(), NodeSide::Left)
                    .with_connected(self.connections.is_connected(socket.into()))
                    .with_name(socket.name()),
            );
        }

        {
            let output_id = self.node.output_socket().into();

            let output_name = match self.node.op() {
                Op::Unary(UnaryOp::Neg) => "-A",
                Op::Binary(BinaryOp::Add) => "A+B",
                Op::Binary(BinaryOp::Sub) => "A-B",
                Op::Binary(BinaryOp::Mul) => "A*B",
                Op::Binary(BinaryOp::Div) => "A/B",
            };

            socket_seq.visit_socket(
                SocketData::new(output_id, NodeSide::Right)
                    .with_connected(self.connections.is_connected(output_id))
                    .with_name(output_name),
            );
        }
    }
}

impl visitor::NodeAdapter for NodeAdapter<'_, Input> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn id(&self) -> Self::NodeId {
        self.node.id().into()
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

    fn accept<'node, V>(&'node mut self, mut visitor: V)
    where
        V: visitor::NodeVisitor<'node, Self::SocketId>,
    {
        let mut socket_seq = visitor.sockets(SizeHint::exact(1));

        let socket_id = self.node.output_socket_id().into();
        socket_seq.visit_socket(
            SocketData::new(socket_id, NodeSide::Right)
                .with_connected(self.connections.is_connected(socket_id))
                .with_name(self.node.name()),
        );
    }
}
