pub mod connections;

pub use connections::Connections;
use nodui::Pos;
use serde::{Deserialize, Serialize};

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId(uuid::Uuid);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InputSocketId(pub NodeId, pub u16);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OutputSocketId(pub NodeId, pub u16);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SocketIndex {
    Input(u16),
    Output(u16),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SocketId {
    Input(InputSocketId),
    Output(OutputSocketId),
}

impl From<InputSocketId> for SocketId {
    fn from(value: InputSocketId) -> Self {
        SocketId::Input(value)
    }
}

impl From<OutputSocketId> for SocketId {
    fn from(value: OutputSocketId) -> Self {
        SocketId::Output(value)
    }
}

impl From<(NodeId, SocketIndex)> for SocketId {
    fn from((node_id, socket_index): (NodeId, SocketIndex)) -> Self {
        match socket_index {
            SocketIndex::Input(index) => SocketId::Input(InputSocketId(node_id, index)),
            SocketIndex::Output(index) => SocketId::Output(OutputSocketId(node_id, index)),
        }
    }
}

impl NodeId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

// impl SocketId {
//     pub fn node_id(self) -> NodeId {
//         match self {
//             SocketId::Input(InputSocketId(node_id, _)) => node_id,
//             SocketId::Output(OutputSocketId(node_id, _)) => node_id,
//         }
//     }

//     pub fn index(self) -> SocketIndex {
//         match self {
//             SocketId::Input(InputSocketId(_, index)) => SocketIndex::Input(index),
//             SocketId::Output(OutputSocketId(_, index)) => SocketIndex::Output(index),
//         }
//     }
// }

/* -------------------------------------------------------------------------- */

#[derive(Default, Serialize, Deserialize)]
pub struct DummyGraph {
    nodes: Vec<DummyNode>,
    connections: Connections,
}

#[derive(Serialize, Deserialize)]
pub struct DummyNode {
    id: NodeId,
    pos: Pos,
    sockets: Vec<DummySocket>,
}

#[derive(Serialize, Deserialize)]
pub struct DummySocket {
    pub index: SocketIndex,
    pub name: String,
}

/* -------------------------------------------------------------------------- */

impl DummyGraph {
    pub fn connections(&self) -> &Connections {
        &self.connections
    }

    pub fn connections_mut(&mut self) -> &mut Connections {
        &mut self.connections
    }

    pub fn nodes_and_connections_mut(&mut self) -> (&mut [DummyNode], &mut Connections) {
        (&mut self.nodes, &mut self.connections)
    }

    pub fn add_node(
        &mut self,
        pos: Pos,
        input_sockets: impl IntoIterator<Item: Into<String>>,
        output_sockets: impl IntoIterator<Item: Into<String>>,
    ) -> NodeId {
        let id = NodeId::new();

        let input_sockets = input_sockets
            .into_iter()
            .enumerate()
            .map(|(i, name)| DummySocket {
                index: SocketIndex::Input(i.try_into().unwrap()),
                name: name.into(),
            });

        let output_sockets = output_sockets
            .into_iter()
            .enumerate()
            .map(|(i, name)| DummySocket {
                index: SocketIndex::Output(i.try_into().unwrap()),
                name: name.into(),
            });

        let sockets = input_sockets.chain(output_sockets).collect();

        self.nodes.push(DummyNode { id, pos, sockets });
        id
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        if let Some(idx) = self.nodes.iter().position(|n| n.id == node_id) {
            self.nodes.swap_remove(idx);
            self.connections.remove_by_node(node_id);
        }
    }

    // pub fn get_socket(&self, id: SocketId) -> Option<&DummySocket> {
    //     let node_id = id.node_id();

    //     self.nodes.iter().find(|n| n.id == node_id).and_then(|n| {
    //         let index = id.index();
    //         n.sockets.iter().find(|s| s.index == index)
    //     })
    // }
}

impl DummyNode {
    pub fn sockets(&self) -> &[DummySocket] {
        &self.sockets
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Pos) {
        self.pos = pos
    }
}

/* -------------------------------------------------------------------------- */

pub fn make_dummy() -> DummyGraph {
    let mut graph = DummyGraph::default();

    graph.add_node(Pos::new(-5, -2), ["In"], ["Out", "Charles", "David"]);

    graph.add_node(Pos::new(5, 3), ["In", "Charles", "David"], ["Out"]);

    graph
}
