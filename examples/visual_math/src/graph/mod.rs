//! A simple math expression graph.

mod connections;
mod expr;
mod id;
mod input;
mod node;

pub use self::connections::Connections;
pub use self::expr::Expr;
pub use self::id::{
    InputId, InputSocketId, IntoOutputSocketId, NodeId, OpNodeId, OutputSocketId, SocketId,
};
pub use self::input::Input;
pub use self::node::{BinaryOp, Op, OpNode, UnaryOp};

use self::id::SocketIndex;

/// A math expression graph.
#[derive(Default)]
pub struct Graph {
    /// The operation nodes.
    nodes: Vec<OpNode>,
    /// The inputs.
    inputs: Vec<Input>,
    /// The connections.
    connections: Connections,
}

pub struct ViewMut<'a> {
    /// The operation nodes.
    pub nodes: &'a mut [OpNode],
    /// The inputs.
    pub inputs: &'a mut [Input],
    /// The connections.
    pub connections: &'a mut Connections,
}

impl Graph {
    /// Creates an empty [`Graph`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a mutable view of this graph.
    pub fn view_mut(&mut self) -> ViewMut<'_> {
        ViewMut {
            nodes: &mut self.nodes,
            inputs: &mut self.inputs,
            connections: &mut self.connections,
        }
    }

    /// A reference to the connections of this graph.
    pub fn connections(&self) -> &Connections {
        &self.connections
    }

    /// A mutable reference to the connections of this graph.
    pub fn connections_mut(&mut self) -> &mut Connections {
        &mut self.connections
    }
}

impl Graph {
    /// Remove a node.
    pub fn remove(&mut self, id: NodeId) {
        match id {
            NodeId::Op(id) => {
                if let Some(index) = self.nodes.iter().position(|n| n.id() == id) {
                    self.nodes.swap_remove(index);
                } else {
                    return;
                }
            }
            NodeId::Input(id) => {
                if let Some(index) = self.inputs.iter().position(|i| i.id() == id) {
                    self.inputs.remove(index);
                } else {
                    return;
                }
            }
        }

        self.connections.remove_by_node(id);
    }
}

impl Graph {
    /// Get an operation node from its identifier.
    pub fn get_op_node(&self, id: OpNodeId) -> Option<&OpNode> {
        self.nodes.iter().find(|n| n.id() == id)
    }

    /// Creates an operation node.
    pub fn add_op_node(&mut self, op: Op) -> OpNodeId {
        let id = OpNodeId::new();
        let node = OpNode::new(id, op);
        self.nodes.push(node);
        id
    }

    /// Creates an unary operation node and connect its input to the specified socket.
    pub fn add_unary_op_node_and_connect_input(
        &mut self,
        op: UnaryOp,
        socket_to_connect: impl IntoOutputSocketId,
    ) -> OpNodeId {
        let node_id = self.add_op_node(op.into());
        self.connections.connect_inner(
            socket_to_connect.into_output_socket_id(),
            node_id.input_socket_id(SocketIndex::A),
        );
        node_id
    }

    /// Creates an binary operation node and connect its inputs to the specified sockets.
    pub fn add_binary_op_node_and_connect_input(
        &mut self,
        op: BinaryOp,
        socket_to_connect_a: impl IntoOutputSocketId,
        socket_to_connect_b: impl IntoOutputSocketId,
    ) -> OpNodeId {
        let node_id = self.add_op_node(op.into());
        self.connections.connect_inner(
            socket_to_connect_a.into_output_socket_id(),
            node_id.input_socket_id(SocketIndex::A),
        );
        self.connections.connect_inner(
            socket_to_connect_b.into_output_socket_id(),
            node_id.input_socket_id(SocketIndex::B),
        );
        node_id
    }
}

impl Graph {
    /// A mutable access to the inputs.
    pub fn inputs_mut(&mut self) -> &mut [Input] {
        &mut self.inputs
    }

    /// Gets an input from its identifier.
    pub fn get_input(&self, id: InputId) -> Option<&Input> {
        self.inputs.iter().find(|n| n.id() == id)
    }

    /// Creates a new input.
    pub fn add_input(&mut self, name: impl Into<String>, value: f32) -> InputId {
        let id = InputId::new();
        let input = Input::new(id, name.into(), value);
        self.inputs.push(input);

        id
    }
}
