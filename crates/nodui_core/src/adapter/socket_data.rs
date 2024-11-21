//! Socket's data.

use crate::ui::{Color, NodeSide, SocketShape, SocketUI, TextUi};

use super::SocketField;

/// The data about a socket.
pub struct SocketData<'field, SocketId> {
    /// The unique identifier of the socket.
    pub id: SocketId,
    /// The side of the node this socket should be placed.
    pub side: NodeSide,
    /// Defines how a socket should be rendered.
    pub ui: SocketUI,
    /// The field of the socket, if any.
    pub field: Option<SocketField<'field>>,
}

impl<'field, Id> SocketData<'field, Id> {
    /// Creates a [`SocketData`] with the specified `id` and `side`.
    #[inline]
    pub fn new(id: Id, side: NodeSide) -> Self {
        Self {
            id,
            side,
            ui: SocketUI::default(),
            field: None,
        }
    }

    // TODO: inline `SocketUI` fields into `SocketData`?

    /// Sets the [`SocketUI`] used to render the socket.
    #[inline]
    #[must_use]
    pub fn with_ui(mut self, ui: SocketUI) -> Self {
        self.ui = ui;
        self
    }

    /// Whether the socket is connected.
    #[inline]
    #[must_use]
    pub fn with_connected(mut self, is_connected: bool) -> Self {
        self.ui.is_connected = is_connected;
        self
    }

    /// Sets the text next to the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_name(mut self, name: impl Into<TextUi>) -> Self {
        self.ui = self.ui.with_name(name);
        self
    }

    /// Sets the color of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        self.ui = self.ui.with_color(color);
        self
    }

    /// Sets the shape of the socket's handle.
    #[inline]
    #[must_use]
    pub fn with_shape(mut self, shape: SocketShape) -> Self {
        self.ui = self.ui.with_shape(shape);
        self
    }

    /// Sets the socket field.
    #[inline]
    #[must_use]
    pub fn with_field(mut self, field: impl Into<SocketField<'field>>) -> Self {
        self.field = Some(field.into());
        self
    }
}
