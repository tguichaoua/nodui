use nodui::{
    ui::{Color, NodeSide, NodeUI, SocketUI, TitleHeader},
    visitor::{self, NodeSeq, SizeHint, SocketSeq},
};

use crate::graph::{self, Connections, DummyNode, NodeId, SocketId, SocketIndex};

use super::adapter::GraphAdapter;

struct NodeAdapter<'a> {
    node: &'a mut DummyNode,
    connections: &'a Connections,
}

impl visitor::GraphAdapter for GraphAdapter<'_> {
    type NodeId = NodeId;
    type SocketId = SocketId;

    fn accept<'graph, V>(&'graph mut self, mut visitor: V)
    where
        V: visitor::GraphVisitor<'graph, Self::NodeId, Self::SocketId>,
    {
        let graph::ViewMut { nodes, connections } = self.graph.view_mut();

        let mut node_seq = visitor.nodes(SizeHint::of(nodes));

        for node in nodes {
            node_seq.visit_node(NodeAdapter { node, connections });
        }
    }
}

impl<'graph> visitor::NodeAdapter for NodeAdapter<'graph> {
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
        V: visitor::NodeVisitor<'node, Self::SocketId>,
    {
        let node_id = self.node.id();
        let sockets = self.node.sockets_mut();

        let mut socket_seq = visitor.sockets(SizeHint::of(sockets));

        for socket in sockets {
            let id = SocketId::from((node_id, socket.index));

            let ui = {
                let side = match socket.index {
                    SocketIndex::Input(_) => NodeSide::Left,
                    SocketIndex::Output(_) => NodeSide::Right,
                };

                SocketUI::new(side, self.connections.is_connected(id)).with_name(&socket.name)
            };

            socket_seq.visit_socket(id, ui, socket.field.as_mut());
        }
    }
}
