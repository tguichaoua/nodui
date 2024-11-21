use nodui::{
    ui::{Color, NodeSide, NodeUI, TitleHeader},
    ConnectionHint, GraphVisitor, NodeSeq, NodeVisitor, SizeHint, SocketData, SocketSeq,
};

use crate::graph::{self, Connections, DummyGraph, DummyNode, NodeId, SocketId, SocketIndex};

pub struct GraphAdapter<'a> {
    pub graph: &'a mut DummyGraph,
}

impl<'a> GraphAdapter<'a> {
    pub fn new(graph: &'a mut DummyGraph) -> Self {
        Self { graph }
    }
}

struct NodeAdapter<'a> {
    node: &'a mut DummyNode,
    connections: &'a Connections,
}

impl nodui::GraphAdapter for GraphAdapter<'_> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn accept<'graph, V>(&'graph mut self, mut visitor: V)
    where
        V: GraphVisitor<'graph, Self::NodeId, Self::SocketId>,
    {
        let graph::ViewMut { nodes, connections } = self.graph.view_mut();

        let mut node_seq = visitor.nodes(SizeHint::of(nodes));

        for node in nodes {
            node_seq.visit_node(NodeAdapter { node, connections });
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

impl<'graph> nodui::NodeAdapter for NodeAdapter<'graph> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn id(&self) -> Self::NodeId {
        self.node.id()
    }

    fn pos(&self) -> nodui::Pos {
        self.node.pos()
    }

    fn set_pos(&mut self, pos: nodui::Pos) {
        self.node.set_pos(pos);
    }

    fn ui(&self) -> nodui::ui::NodeUI {
        NodeUI::default().with_header(TitleHeader::new(
            format!("Dummy Node {:?}", self.node.id()),
            Color::RED,
        ))
    }

    fn accept<'node, V>(&'node mut self, mut visitor: V)
    where
        V: NodeVisitor<'node, Self::SocketId>,
    {
        let node_id = self.node.id();
        let sockets = self.node.sockets_mut();

        let mut socket_seq = visitor.sockets(SizeHint::of(sockets));

        for socket in sockets {
            let id = SocketId::from((node_id, socket.index));

            let side = match socket.index {
                SocketIndex::Input(_) => NodeSide::Left,
                SocketIndex::Output(_) => NodeSide::Right,
            };

            let mut data = SocketData::new(id, side)
                .with_connected(self.connections.is_connected(id))
                .with_name(&socket.name);

            if let Some(field) = socket.field.as_mut() {
                data = data.with_field(field)
            }

            socket_seq.visit_socket(data);
        }
    }
}
