use nodui::ui::{Color, NodeSide, NodeUI, SocketUI, TitleHeader};
use nodui::{ConnectionHint, Pos};

use crate::graph::{
    Connections, DummyGraph, DummyNode, DummySocket, NodeId, SocketId, SocketIndex,
};

pub struct GraphAdapter<'a> {
    pub graph: &'a mut DummyGraph,
}

impl<'a> GraphAdapter<'a> {
    pub fn new(graph: &'a mut DummyGraph) -> Self {
        Self { graph }
    }
}

impl<'a> nodui::GraphAdapter for GraphAdapter<'a> {
    type NodeId = NodeId;

    type SocketId = SocketId;

    fn nodes(
        &self,
    ) -> impl Iterator<Item: nodui::NodeAdapter<NodeId = Self::NodeId, SocketId = Self::SocketId>>
    {
        let nodes = self.graph.nodes();
        let connections = self.graph.connections();

        nodes.iter().map(|node| NodeAdapter { node, connections })
    }

    fn set_node_pos(&mut self, node_id: Self::NodeId, pos: Pos) {
        if let Some(node) = self.graph.get_node_mut(node_id) {
            node.set_pos(pos);
        }
    }

    fn connection_hint(&self, a: Self::SocketId, b: Self::SocketId) -> ConnectionHint {
        if crate::graph::connections::can_connect(a, b) {
            ConnectionHint::Accept
        } else {
            ConnectionHint::Reject
        }
    }

    fn connect(&mut self, a: Self::SocketId, b: Self::SocketId) {
        self.graph.connections_mut().connect(a, b);
    }

    fn connections(&self) -> impl Iterator<Item = (Self::SocketId, Self::SocketId)> {
        self.graph
            .connections()
            .iter()
            .map(|(a, b)| (SocketId::from(a), SocketId::from(b)))
    }
}

struct NodeAdapter<'a> {
    node: &'a DummyNode,
    connections: &'a Connections,
}

struct SocketAdapter<'a> {
    node: &'a DummyNode,
    socket: &'a DummySocket,
    connections: &'a Connections,
}

impl<'a> nodui::NodeAdapter for NodeAdapter<'a> {
    type NodeId = NodeId;

    type SocketId = SocketId;

    fn sockets(&self) -> impl Iterator<Item: nodui::SocketAdapter<SocketId = Self::SocketId>> {
        self.node.sockets().iter().map(|socket| SocketAdapter {
            connections: self.connections,
            node: self.node,
            socket,
        })
    }

    fn id(&self) -> Self::NodeId {
        self.node.id()
    }

    fn pos(&self) -> Pos {
        self.node.pos()
    }

    fn ui(&self) -> NodeUI {
        NodeUI::default().with_header(TitleHeader::new(
            format!("Dummy Node {:?}", self.node.id()),
            Color::RED,
        ))
    }
}

impl<'a> nodui::SocketAdapter for SocketAdapter<'a> {
    type SocketId = SocketId;

    fn id(&self) -> Self::SocketId {
        SocketId::from((self.node.id(), self.socket.index))
    }

    fn ui(&self) -> SocketUI {
        let id = self.id();

        let side = match self.socket.index {
            SocketIndex::Input(_) => NodeSide::Left,
            SocketIndex::Output(_) => NodeSide::Right,
        };

        SocketUI::new(side, self.connections.is_connected(id)).with_name(&self.socket.name)
    }
}
