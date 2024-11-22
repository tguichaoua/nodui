//! Identifier for the graph.

/* -------------------------------------------------------------------------- */

/// The unique identifier for a node of the graph.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum NodeId {
    /// An operation node identifier.
    Op(OpNodeId),
    /// An input node identifier.
    Input(InputId),
}

/// The unique identifier for an operation node.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct OpNodeId(uuid::Uuid);

impl OpNodeId {
    /// Generates a new random [`OpNodeId`].
    pub(super) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Creates a [`InputSocketId`] for this node with the specified [`SocketIndex`].
    pub(super) fn input_socket_id(self, index: SocketIndex) -> InputSocketId {
        InputSocketId {
            node_id: self,
            index,
        }
    }
}

/// The unique identifier for an input node.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct InputId(uuid::Uuid);

impl InputId {
    /// Generates a new random [`InputId`].
    pub(super) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl From<OpNodeId> for NodeId {
    fn from(value: OpNodeId) -> Self {
        NodeId::Op(value)
    }
}

impl From<InputId> for NodeId {
    fn from(value: InputId) -> Self {
        NodeId::Input(value)
    }
}

/* -------------------------------------------------------------------------- */

/// The unique identifier for a socket of the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketId {
    /// An output socket id.
    Output(OutputSocketId),
    /// An input socket id.
    Input(InputSocketId),
}

/// The unique identifier for an output socket.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct OutputSocketId {
    /// The node id this socket belong to.
    ///
    /// We don't need more information since any node has only one output.
    pub(super) node_id: NodeId,
}

/// The unique identifier for an input socket.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct InputSocketId {
    /// The node id this socket belong to.
    pub(super) node_id: OpNodeId,
    /// The index of the socket.
    pub(super) index: SocketIndex,
}

/// An index to differentiate socket of a node.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub(super) enum SocketIndex {
    #[allow(clippy::missing_docs_in_private_items)]
    A,
    #[allow(clippy::missing_docs_in_private_items)]
    B,
}

impl InputSocketId {
    /// The name of the socket.
    pub fn name(self) -> &'static str {
        match self.index {
            SocketIndex::A => "A",
            SocketIndex::B => "B",
        }
    }
}

impl From<OutputSocketId> for SocketId {
    fn from(value: OutputSocketId) -> Self {
        SocketId::Output(value)
    }
}

impl From<InputSocketId> for SocketId {
    fn from(value: InputSocketId) -> Self {
        SocketId::Input(value)
    }
}

impl PartialEq<OpNodeId> for NodeId {
    fn eq(&self, other: &OpNodeId) -> bool {
        match self {
            NodeId::Op(id) => id == other,
            NodeId::Input(_) => false,
        }
    }
}

impl PartialEq<NodeId> for OpNodeId {
    fn eq(&self, other: &NodeId) -> bool {
        other == self
    }
}

impl PartialEq<InputId> for NodeId {
    fn eq(&self, other: &InputId) -> bool {
        match self {
            NodeId::Op(_) => false,
            NodeId::Input(id) => id == other,
        }
    }
}

impl PartialEq<NodeId> for InputId {
    fn eq(&self, other: &NodeId) -> bool {
        other == self
    }
}

/* -------------------------------------------------------------------------- */

#[allow(clippy::missing_docs_in_private_items)]
mod private {
    pub trait Sealed {}

    impl Sealed for super::OutputSocketId {}
    impl Sealed for super::InputId {}
    impl Sealed for super::OpNodeId {}
    impl Sealed for super::NodeId {}
}

/// A value that can be converted into a [`OutputSocketId`].
pub trait IntoOutputSocketId: private::Sealed {
    /// Converts this value into a [`OutputSocketId`].
    fn into_output_socket_id(self) -> OutputSocketId;
}

impl IntoOutputSocketId for OutputSocketId {
    fn into_output_socket_id(self) -> OutputSocketId {
        self
    }
}

impl IntoOutputSocketId for InputId {
    fn into_output_socket_id(self) -> OutputSocketId {
        OutputSocketId {
            node_id: self.into(),
        }
    }
}

impl IntoOutputSocketId for OpNodeId {
    fn into_output_socket_id(self) -> OutputSocketId {
        OutputSocketId {
            node_id: self.into(),
        }
    }
}

impl IntoOutputSocketId for NodeId {
    fn into_output_socket_id(self) -> OutputSocketId {
        OutputSocketId { node_id: self }
    }
}

/* -------------------------------------------------------------------------- */
