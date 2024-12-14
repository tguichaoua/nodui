//! Connection between socket of the graph.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use super::id::SocketId;

/// Connection between sockets of the graph.
#[derive(Default, Serialize, Deserialize)]
pub struct Connections {
    /// The connections between two sockets.
    connections: HashSet<Pair>,
}

impl Connections {
    /// Try to connect those sockets.
    ///
    /// Returns `true` on success, `false` otherwise.
    pub fn connect(&mut self, a: SocketId, b: SocketId) -> bool {
        self.connections.insert(Pair::new(a, b))
    }

    /// An iterator over the connections.
    pub fn iter(&self) -> impl Iterator<Item = (SocketId, SocketId)> + '_ {
        self.connections.iter().map(|&Pair(a, b)| (a, b))
    }

    /// Whether or not this socket has at least one connections.
    pub fn is_connected(&self, socket: SocketId) -> bool {
        self.connections
            .iter()
            .any(|&Pair(a, b)| socket == a || socket == b)
    }

    // /// Get the [`OutputSocketId`] connected to this [`InputSocketId`], if any.
    // pub fn get(&self, socket: SocketId) -> Option<SocketId> {
    //     todo!();
    // }

    /// Remove the connection from this socket.
    pub fn disconnect(&mut self, socket: SocketId) {
        self.connections = self
            .connections
            .drain()
            .filter(|pair| !pair.contains(socket))
            .collect();
    }
}

/* -------------------------------------------------------------------------- */

#[derive(Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
struct Pair(SocketId, SocketId);

impl Pair {
    fn new(a: SocketId, b: SocketId) -> Self {
        if a <= b {
            Pair(a, b)
        } else {
            Pair(b, a)
        }
    }

    fn contains(&self, id: SocketId) -> bool {
        self.0 == id || self.1 == id
    }
}
