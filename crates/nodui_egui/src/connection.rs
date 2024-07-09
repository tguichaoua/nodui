//! Renderer for the connections between sockets.

use egui::{Color32, Pos2, Shape, Stroke};
use nodui_core::adapter::ConnectionHint;

use crate::socket::RenderedSocket;

/* -------------------------------------------------------------------------- */

/// A renderer to render the connections between sockets.
pub enum ConnectionRenderer {
    /// Renders the connection with straight lines.
    Line(LineConnectionRenderer),

    /// A custom renderer.
    Custom(Box<dyn CustomConnectionRenderer>),
}

impl From<LineConnectionRenderer> for ConnectionRenderer {
    #[inline]
    fn from(value: LineConnectionRenderer) -> Self {
        ConnectionRenderer::Line(value)
    }
}

impl<T> From<T> for ConnectionRenderer
where
    T: CustomConnectionRenderer + 'static,
{
    #[inline]
    fn from(value: T) -> Self {
        ConnectionRenderer::Custom(Box::new(value))
    }
}

impl ConnectionRenderer {
    /// Creates a [`Shape`] that represents the connection between the two sockets.
    #[inline]
    pub fn socket_to_socket(&self, a: &RenderedSocket, b: &RenderedSocket) -> Shape {
        match self {
            ConnectionRenderer::Line(renderer) => renderer.socket_to_socket(a, b),
            ConnectionRenderer::Custom(renderer) => renderer.socket_to_socket(a, b),
        }
    }

    /// Creates a [`Shape`] that represents a in-going connection between a socket and the pointer.
    #[inline]
    pub fn socket_to_pointer(
        &self,
        socket: &RenderedSocket,
        pointer: Pos2,
        hint: Option<ConnectionHint>,
    ) -> Shape {
        match self {
            ConnectionRenderer::Line(renderer) => renderer.socket_to_pointer(socket, pointer, hint),
            ConnectionRenderer::Custom(renderer) => {
                renderer.socket_to_pointer(socket, pointer, hint)
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

/// A custom [`ConnectionRenderer`] use to render the connection between sockets.
pub trait CustomConnectionRenderer {
    /// Creates a [`Shape`] that represents the connection between the two sockets.
    fn socket_to_socket(&self, a: &RenderedSocket, b: &RenderedSocket) -> Shape;

    /// Creates a [`Shape`] that represents a in-going connection between a socket and the pointer.
    fn socket_to_pointer(
        &self,
        socket: &RenderedSocket,
        pointer: Pos2,
        hint: Option<ConnectionHint>,
    ) -> Shape;
}

/* -------------------------------------------------------------------------- */

/// A [`ConnectionRenderer`] that use straight line.
pub struct LineConnectionRenderer {
    /// The color of the line if the hint is [`ConnectionHint::Accept`].
    pub on_accept_color: Color32,

    /// The color of the line if the hint is [`ConnectionHint::Reject`].
    pub on_reject_color: Color32,

    /// The color of the lines.
    pub default_color: Color32,

    /// The width of the lines.
    pub line_width: f32,

    /// The width of the line when there is a hint.
    pub on_hint_line_width: f32,
}

impl Default for LineConnectionRenderer {
    #[inline]
    fn default() -> Self {
        Self {
            on_accept_color: Color32::GREEN,
            on_reject_color: Color32::RED,
            default_color: Color32::WHITE,
            line_width: 3.0,
            on_hint_line_width: 5.0,
        }
    }
}

impl LineConnectionRenderer {
    /// The color of the line if the hint is [`ConnectionHint::Accept`].
    #[inline]
    #[must_use]
    pub fn on_accept_color(mut self, color: Color32) -> Self {
        self.on_accept_color = color;
        self
    }

    /// The color of the line if the hint is [`ConnectionHint::Reject`].
    #[inline]
    #[must_use]
    pub fn on_reject_color(mut self, color: Color32) -> Self {
        self.on_reject_color = color;
        self
    }

    /// The color of the lines.
    #[inline]
    #[must_use]
    pub fn default_color(mut self, color: Color32) -> Self {
        self.default_color = color;
        self
    }

    /// The width of the lines.
    #[inline]
    #[must_use]
    pub fn line_width(mut self, width: f32) -> Self {
        self.line_width = width;
        self
    }

    /// The width of the line when there is a hint.
    #[inline]
    #[must_use]
    pub fn on_hint_line_width(mut self, width: f32) -> Self {
        self.on_hint_line_width = width;
        self
    }
}

impl LineConnectionRenderer {
    /// Creates a [`Shape`] that represents the connection between the two sockets.
    #[inline]
    pub fn socket_to_socket(&self, a: &RenderedSocket, b: &RenderedSocket) -> Shape {
        Shape::LineSegment {
            points: [a.pos(), b.pos()],
            stroke: Stroke::new(self.line_width, a.color).into(),
        }
    }

    /// Creates a [`Shape`] that represents a in-going connection between a socket and the pointer.
    #[inline]
    pub fn socket_to_pointer(
        &self,
        socket: &RenderedSocket,
        pointer: Pos2,
        hint: Option<ConnectionHint>,
    ) -> Shape {
        let stroke = match hint {
            Some(ConnectionHint::Accept) => {
                Stroke::new(self.on_hint_line_width, self.on_accept_color)
            }
            Some(ConnectionHint::Reject) => {
                Stroke::new(self.on_hint_line_width, self.on_reject_color)
            }
            None => Stroke::new(self.line_width, self.default_color),
        };

        Shape::LineSegment {
            points: [socket.pos(), pointer],
            stroke: stroke.into(),
        }
    }
}

/* -------------------------------------------------------------------------- */
