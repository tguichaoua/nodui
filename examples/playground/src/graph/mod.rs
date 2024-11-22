mod connections;
mod id;
mod node;
mod socket;

use serde::{Deserialize, Serialize};

pub use connections::Connections;
pub use id::{NodeId, SocketId};
pub use node::Node;
pub use socket::Socket;

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

    pub fn add_node(&mut self, pos: nodui::Pos) -> &mut Node {
        self.nodes.push(Node::new(pos));
        #[allow(clippy::unwrap_used)]
        self.nodes.last_mut().unwrap()
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.id() == id)
    }
}
