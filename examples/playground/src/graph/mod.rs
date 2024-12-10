mod connections;
mod id;
mod node;
mod socket;

use serde::{Deserialize, Serialize};

pub use connections::Connections;
pub use id::{NodeId, SocketId};
pub(crate) use node::{HeaderMode, Node, NodeHeaderStyle, NodeStyle};
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
        style: NodeStyle,
        sockets: impl IntoIterator<Item = SocketStyle>,
    ) -> &mut Node {
        self.nodes.push(Node::new(pos, style, sockets));
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

    fn find_socket_mut(&mut self, socket_id: SocketId) -> Option<(&mut Node, usize)> {
        for node in &mut self.nodes {
            if let Some(index) = node.sockets.iter().position(|s| s.id() == socket_id) {
                return Some((node, index));
            }
        }

        None
    }

    pub fn remove_socket(&mut self, socket_id: SocketId) {
        if let Some((node, index)) = self.find_socket_mut(socket_id) {
            node.sockets.remove(index);
        }
        self.connections.disconnect(socket_id);
    }

    pub fn move_socket_up(&mut self, socket_id: SocketId) {
        if let Some((node, index)) = self.find_socket_mut(socket_id) {
            if let Some(prev_index) = index.checked_sub(1) {
                node.sockets_mut().swap(prev_index, index);
            }
        }
    }

    pub fn move_socket_down(&mut self, socket_id: SocketId) {
        if let Some((node, index)) = self.find_socket_mut(socket_id) {
            let next_index = index + 1;
            if next_index < node.sockets().len() {
                node.sockets_mut().swap(index, next_index);
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

/* -------------------------------------------------------------------------- */

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct Maybe<T> {
    pub enabled: bool,
    pub value: T,
}

impl<T> Maybe<T> {
    pub fn disabled_with(value: T) -> Self {
        Self {
            enabled: false,
            value,
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.enabled.then_some(&self.value)
    }
}

/* -------------------------------------------------------------------------- */
