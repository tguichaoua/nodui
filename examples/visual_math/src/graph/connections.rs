//! Connection between socket of the graph.

use std::collections::HashMap;

use super::{
    id::{InputSocketId, OutputSocketId, SocketId},
    NodeId,
};

/// Connection between sockets of the graph.
#[derive(Default)]
pub struct Connections {
    /// The connections between the [`InputSocketId`] and the [`OutputSocketId`].
    connections: HashMap<InputSocketId, OutputSocketId>,
}

impl Connections {
    /// Try to connect those sockets.
    ///
    /// Returns `true` on success, `false` otherwise.
    pub fn connect(&mut self, a: SocketId, b: SocketId) -> bool {
        let Some((input_socket, output_socket)) = prepare_connection(a, b) else {
            return false;
        };

        self.connections.insert(input_socket, output_socket);
        true
    }

    /// Insert a connection between two sockets.
    pub(super) fn connect_inner(&mut self, a: OutputSocketId, b: InputSocketId) {
        self.connections.insert(b, a);
    }

    /// An iterator over the connections.
    pub fn iter(&self) -> impl Iterator<Item = (InputSocketId, OutputSocketId)> + '_ {
        self.connections.iter().map(|(&k, &v)| (k, v))
    }

    /// Whether or not this socket has at least one connections.
    pub fn is_connected(&self, socket: SocketId) -> bool {
        match socket {
            SocketId::Output(output) => self.connections.values().any(|&s| s == output),
            SocketId::Input(input) => self.connections.contains_key(&input),
        }
    }

    /// Removes all connections connected to this node.
    pub fn remove_by_node(&mut self, node_id: NodeId) {
        self.connections
            .retain(|k, v| k.node_id != node_id && v.node_id != node_id);
    }

    /// Get the [`OutputSocketId`] connected to this [`InputSocketId`], if any.
    pub fn get(&self, input: InputSocketId) -> Option<OutputSocketId> {
        self.connections.get(&input).copied()
    }

    /// Remove the connection from this socket.
    pub fn disconnect(&mut self, socket: InputSocketId) {
        self.connections.remove(&socket);
    }

    /// Removes all connections from this socket.
    pub fn disconnect_all(&mut self, socket: OutputSocketId) {
        self.connections.retain(|_, v| *v != socket);
    }

    /// Whether or not those sockets can be connected.
    pub fn can_connect(a: SocketId, b: SocketId) -> bool {
        prepare_connection(a, b).is_some()
    }
}

/// Extract which socket is the input and which one is the output.
///
/// Returns `None` is the socket cannot be connected.
fn prepare_connection(a: SocketId, b: SocketId) -> Option<(InputSocketId, OutputSocketId)> {
    match (a, b) {
        (SocketId::Input(input), SocketId::Output(output))
        | (SocketId::Output(output), SocketId::Input(input))
            if input.node_id != output.node_id =>
        {
            Some((input, output))
        }

        _ => None,
    }
}
