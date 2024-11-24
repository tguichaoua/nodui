use nodui::{SizeHint, SocketData, SocketSeq};

use crate::graph;

pub(super) struct NodeAdapter<'a> {
    pub(super) node: &'a mut graph::Node,
    pub(super) connections: &'a graph::Connections,
}

impl nodui::NodeAdapter for NodeAdapter<'_> {
    type NodeId = graph::NodeId;
    type SocketId = graph::SocketId;

    fn id(&self) -> Self::NodeId {
        self.node.id()
    }

    fn pos(&self) -> nodui::Pos {
        self.node.pos
    }

    fn set_pos(&mut self, pos: nodui::Pos) {
        self.node.pos = pos;
    }

    fn ui(&self) -> nodui::ui::NodeUI {
        self.node.style.clone().into()
    }

    fn accept<'node, V>(&'node mut self, mut visitor: V)
    where
        V: nodui::NodeVisitor<'node, Self::SocketId>,
    {
        let sockets = self.node.sockets_mut();
        let mut socket_seq = visitor.sockets(SizeHint::of(sockets));

        for socket in sockets {
            let id = socket.id();

            let is_connected = self.connections.is_connected(id);

            socket_seq.visit_socket(SocketData {
                id,
                side: socket.style.side,
                ui: nodui::ui::SocketUI {
                    name: socket.style.name.clone(),
                    is_connected,
                    color: socket.style.color,
                    shape: socket.style.shape,
                },
                field: None,
            });
        }
    }
}
