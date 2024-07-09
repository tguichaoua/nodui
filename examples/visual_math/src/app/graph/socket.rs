//! Implementation of the nodui adapter traits for the sockets.

use std::iter;

use either::Either;
use nodui::ui::{NodeSide, SocketUI};

use crate::graph::{BinaryOp, Connections, Input, Op, OpNode, SocketId, UnaryOp};

/* -------------------------------------------------------------------------- */

/// An iterator over the sockets of the math graph.
pub(super) struct SocketIter<'a> {
    /// The connections of the graph.
    pub(super) connections: &'a Connections,
    /// The node this socket belong to.
    pub(super) node: Either<&'a OpNode, &'a Input>,
    /// The current index of the iterator.
    pub(super) socket_index: usize,
}

impl<'a> Iterator for SocketIter<'a> {
    type Item = SocketAdapter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let socket = match self.node {
            Either::Left(node) => {
                // NOTE: it looks ugly but since the iterator is lazy it's okay. (I hope)
                let mut iter = node
                    .input_socket_ids()
                    .map(|input_socket_id| SocketAdapter {
                        id: input_socket_id.into(),
                        connections: self.connections,
                        name: input_socket_id.name(),
                    })
                    .chain(iter::once_with(|| {
                        let name = match node.op() {
                            Op::Unary(UnaryOp::Neg) => "-A",
                            Op::Binary(BinaryOp::Add) => "A+B",
                            Op::Binary(BinaryOp::Sub) => "A-B",
                            Op::Binary(BinaryOp::Mul) => "A*B",
                            Op::Binary(BinaryOp::Div) => "A/B",
                        };

                        SocketAdapter {
                            id: node.output_socket().into(),
                            connections: self.connections,
                            name,
                        }
                    }));

                iter.nth(self.socket_index)?
            }
            Either::Right(input) if self.socket_index == 0 => SocketAdapter {
                id: input.output_socket_id().into(),
                connections: self.connections,
                name: input.name(),
            },
            Either::Right(_) => return None,
        };

        self.socket_index += 1;
        Some(socket)
    }
}

/* -------------------------------------------------------------------------- */

/// The adapter of a socket of the math graph.
pub struct SocketAdapter<'a> {
    /// The identifier of the socket.
    id: SocketId,
    /// The connections of the graph.
    connections: &'a Connections,
    /// The name of this socket.
    name: &'a str,
}

impl nodui::SocketAdapter for SocketAdapter<'_> {
    type SocketId = SocketId;

    fn id(&self) -> Self::SocketId {
        self.id
    }

    fn ui(&self) -> SocketUI {
        let side = match self.id {
            SocketId::Output(_) => NodeSide::Right,
            SocketId::Input(_) => NodeSide::Left,
        };

        SocketUI::new(side, self.connections.is_connected(self.id)).with_name(self.name)
    }
}

/* -------------------------------------------------------------------------- */
