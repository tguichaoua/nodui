use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{InputSocketId, NodeId, OutputSocketId, SocketId};

#[derive(Default, Serialize, Deserialize)]
pub struct Connections {
    connections: HashMap<InputSocketId, OutputSocketId>,
}

impl Connections {
    pub fn connect(&mut self, a: SocketId, b: SocketId) -> bool {
        let Some((input_socket, output_socket)) = prepare_connection(a, b) else {
            return false;
        };

        self.connect_inner(input_socket, output_socket);
        true
    }

    pub(super) fn connect_inner(&mut self, input: InputSocketId, output: OutputSocketId) {
        self.connections.insert(input, output);
    }

    pub fn iter(&self) -> impl Iterator<Item = (InputSocketId, OutputSocketId)> + '_ {
        self.connections.iter().map(|(&k, &v)| (k, v))
    }

    pub fn is_connected(&self, socket: SocketId) -> bool {
        match socket {
            SocketId::Output(output) => self.connections.values().any(|&s| s == output),
            SocketId::Input(input) => self.connections.contains_key(&input),
        }
    }

    pub fn remove_by_node(&mut self, node_id: NodeId) {
        self.connections
            .retain(|k, v| k.0 != node_id && v.0 != node_id);
    }

    // pub fn get(&self, input: InputSocketId) -> Option<OutputSocketId> {
    //     self.connections.get(&input).copied()
    // }

    pub fn disconnect(&mut self, socket: InputSocketId) {
        self.connections.remove(&socket);
    }

    pub fn disconnect_all(&mut self, socket: OutputSocketId) {
        self.connections.retain(|_, v| *v != socket);
    }
}

pub fn can_connect(a: SocketId, b: SocketId) -> bool {
    prepare_connection(a, b).is_some()
}

fn prepare_connection(a: SocketId, b: SocketId) -> Option<(InputSocketId, OutputSocketId)> {
    match (a, b) {
        (SocketId::Input(input), SocketId::Output(output))
        | (SocketId::Output(output), SocketId::Input(input))
            if input.0 != output.0 =>
        {
            Some((input, output))
        }

        _ => None,
    }
}
