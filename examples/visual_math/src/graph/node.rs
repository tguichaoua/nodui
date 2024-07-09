//! Nodes for the operations.

use super::{
    id::{OpNodeId, SocketIndex},
    InputSocketId, OutputSocketId,
};

/* -------------------------------------------------------------------------- */

/// An operation node.
pub struct OpNode {
    /// The unique identifier of this node.
    id: OpNodeId,
    /// The math operation of this node.
    op: Op,
}

impl OpNode {
    /// Creates a [`OpNode`].
    pub(super) fn new(id: OpNodeId, kind: Op) -> Self {
        Self { id, op: kind }
    }

    /// The unique identifier of this node.
    pub fn id(&self) -> OpNodeId {
        self.id
    }

    /// The math operation of this node.
    pub fn op(&self) -> Op {
        self.op
    }

    /// An iterator over the input socket's identifiers of this node.
    pub fn input_socket_ids(&self) -> impl Iterator<Item = InputSocketId> + '_ {
        let n = match self.op {
            Op::Unary(_) => 1,
            Op::Binary(_) => 2,
        };

        (0..n).map(|i| {
            self.id.input_socket_id(match i {
                0 => SocketIndex::A,
                1 => SocketIndex::B,
                _ => unreachable!(),
            })
        })
    }

    /// The identifier of the output socket of this node.
    pub fn output_socket(&self) -> OutputSocketId {
        OutputSocketId {
            node_id: self.id.into(),
        }
    }
}

/* -------------------------------------------------------------------------- */

/// A math operation.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    /// A unary operation.
    Unary(UnaryOp),
    /// A binary operation.
    Binary(BinaryOp),
}

/// A unary math operation.
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    /// The negative operation.
    Neg,
}

/// A binary math operation.
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    /// The addition operation.
    Add,
    /// The subtractive operation.
    Sub,
    /// The multiplication operation.
    Mul,
    /// The division operation.
    Div,
}

impl From<UnaryOp> for Op {
    fn from(value: UnaryOp) -> Self {
        Op::Unary(value)
    }
}

impl From<BinaryOp> for Op {
    fn from(value: BinaryOp) -> Self {
        Op::Binary(value)
    }
}

/* -------------------------------------------------------------------------- */
