//! Rendering options for the sockets.

use super::{Color, TextUi};

/* -------------------------------------------------------------------------- */

/// Defines how a socket should be rendered.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SocketUI {
    /// The text next to the socket's handle.
    pub name: TextUi,
    /// The side of the node this socket should be placed.
    pub side: NodeSide,
    /// Whether or not this socket is connected to at least one other socket.
    pub is_connected: bool,
    /// The color of socket's handle.
    pub color: Color,
    /// The shape of the handle of the socket.
    pub shape: SocketShape,
}

impl SocketUI {
    /// Creates a [`SocketUI`].
    #[inline]
    #[must_use]
    pub fn new(side: NodeSide, is_connected: bool) -> Self {
        Self {
            name: TextUi::default(),
            side,
            is_connected,
            color: Color::WHITE,
            shape: SocketShape::default(),
        }
    }

    /// Sets the text next to the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_name(mut self, name: impl Into<TextUi>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the color of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the shape of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_shape(mut self, shape: SocketShape) -> Self {
        self.shape = shape;
        self
    }
}

/* -------------------------------------------------------------------------- */

/// The shape of a socket's handle.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketShape {
    /// Circle shape.
    #[default]
    Circle,
    /// Square shape.
    Square,
    /// Triangle shape.
    Triangle,
}

/* -------------------------------------------------------------------------- */

/// The node side where a socket is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum NodeSide {
    /// The socket is rendered on the left side of the node.
    Left,
    /// The socket is rendered on the right side of the node.
    Right,
}

/* -------------------------------------------------------------------------- */
