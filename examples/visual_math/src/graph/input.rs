//! The input value of a graph.

use super::{id::InputId, OutputSocketId};

/// An input entry of the math graph.
pub struct Input {
    /// The unique identifier of this input.
    id: InputId,
    /// The name of this input.
    name: String,
    /// The current value of this input.
    value: f32,
}

impl Input {
    /// Creates a [`Input`].
    pub(super) fn new(id: InputId, name: String, value: f32) -> Self {
        Self { id, name, value }
    }

    /// The unique identifier of this input.
    pub fn id(&self) -> InputId {
        self.id
    }

    /// The name of this input.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The current value of this input.
    pub fn value(&self) -> f32 {
        self.value
    }

    /// An mutable reference to this input's name.
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    /// An mutable reference to this input's value.
    pub fn value_mut(&mut self) -> &mut f32 {
        &mut self.value
    }

    /// The identifier of the output socket of this node.
    pub fn output_socket_id(&self) -> OutputSocketId {
        OutputSocketId {
            node_id: self.id.into(),
        }
    }
}
