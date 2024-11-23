mod connections;
mod id;
mod node;
mod socket;

use serde::{Deserialize, Serialize};

pub use connections::Connections;
pub use id::{NodeId, SocketId};
pub use node::{Node, NodeStyle};
pub use socket::{Socket, SocketStyle};

#[derive(Default, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    connections: Connections,
}

pub struct ViewMut<'a> {
    pub nodes: &'a mut [Node],
    pub connections: &'a mut Connections,
}

impl Graph {
    pub fn view_mut(&mut self) -> ViewMut<'_> {
        ViewMut {
            nodes: &mut self.nodes,
            connections: &mut self.connections,
        }
    }

    pub fn connections(&self) -> &Connections {
        &self.connections
    }

    pub fn connections_mut(&mut self) -> &mut Connections {
        &mut self.connections
    }

    pub fn add_node(
        &mut self,
        pos: nodui::Pos,
        settings: NodeStyle,
        sockets: impl IntoIterator<Item = SocketStyle>,
    ) -> &mut Node {
        self.nodes.push(Node::new(pos, settings, sockets));
        #[allow(clippy::unwrap_used)]
        self.nodes.last_mut().unwrap()
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.iter().find(|node| node.id() == id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id() == id)
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        if let Some(index) = self.nodes.iter().position(|n| n.id() == node_id) {
            let node = self.nodes.remove(index);
            for socket in node.sockets {
                self.connections.disconnect(socket.id());
            }
        }
    }

    pub fn remove_socket(&mut self, socket_id: SocketId) {
        for node in &mut self.nodes {
            if let Some(index) = node.sockets.iter().position(|s| s.id() == socket_id) {
                node.sockets.remove(index);
                self.connections.disconnect(socket_id);
                return;
            }
        }
    }

    pub fn replace_sockets(
        &mut self,
        node_id: NodeId,
        sockets: impl IntoIterator<Item = SocketStyle>,
    ) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.id() == node_id) {
            for socket in &node.sockets {
                self.connections.disconnect(socket.id());
            }
            node.sockets = sockets.into_iter().map(Socket::new).collect();
        }
    }
}
