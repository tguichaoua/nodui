//! Rendering for sockets.

use egui::epaint::{CircleShape, PathShape, RectShape};
use egui::{vec2, Color32, Pos2, Rect, Response, Rounding, Shape, Stroke, Vec2};

/* -------------------------------------------------------------------------- */

/// A socket to be rendered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Socket<S> {
    /// The id of the socket.
    pub id: S,
    /// On which side of the node the socket is rendered.
    pub side: NodeSide,
    /// The text of the socket.
    pub text: String,
    /// The color of the text.
    ///
    /// Note: [`Color32::PLACEHOLDER`] will be replace by [`egui::Visuals::text_color()`].
    pub text_color: Color32,
    /// Whether or not the shape should be filled.
    pub filled: bool,
    /// The shape of the socket.
    pub shape: SocketShape,
    /// The color of the shape of the socket.
    ///
    /// Note: [`Color32::PLACEHOLDER`] will be replace by `text_color`.
    pub color: Color32,
}

impl<S> Socket<S> {
    /// Creates a [`Socket`] with the default settings.
    #[inline]
    pub fn new(id: S, side: NodeSide) -> Self {
        Self {
            id,
            side,
            text: String::default(),
            text_color: Color32::PLACEHOLDER,
            filled: false,
            shape: SocketShape::default(),
            color: Color32::PLACEHOLDER,
        }
    }

    /// The text of the socket.
    #[must_use]
    #[inline]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// The color of the text.
    #[must_use]
    #[inline]
    pub fn text_color(mut self, color: impl Into<Color32>) -> Self {
        self.text_color = color.into();
        self
    }

    /// Whether or not the shape should be filled.
    #[must_use]
    #[inline]
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// The shape of the socket.
    #[must_use]
    #[inline]
    pub fn shape(mut self, shape: SocketShape) -> Self {
        self.shape = shape;
        self
    }

    /// The color of the shape of the socket.
    #[must_use]
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }
}

/* -------------------------------------------------------------------------- */

/// A socket after it has been rendered.
#[derive(Debug, Clone)]
pub struct RenderedSocket<S> {
    /// The id of the socket.
    pub id: S,
    /// The [`Response`] of the socket widget.
    pub response: Response,
    /// On which side of the node the socket is rendered.
    pub side: NodeSide,
    /// The color of the shape of the socket.
    pub color: Color32,
}

impl<S> RenderedSocket<S> {
    /// The UI position in which the socket is rendered.
    #[inline]
    #[must_use]
    pub fn pos(&self) -> Pos2 {
        self.response.rect.center()
    }
}

/* -------------------------------------------------------------------------- */

/// An interaction the user may have with the sockets.
pub(crate) enum SocketInteraction<S> {
    /// No interaction.
    None,
    /// The user try to connect two socket.
    Connect(S, S),
    /// The user is dragging a socket.
    InProgress(ConnectionInProgress<S>),
}

/// An in progress connection between two sockets.
pub struct ConnectionInProgress<S> {
    /// The socket from which this connection has begin.
    pub source: RenderedSocket<S>,
    /// The socket that is currently under the pointer, if any.
    pub target: Option<RenderedSocket<S>>,
    /// The current position of the pointer.
    pub pointer_pos: Pos2,
}

/// Handle the socket responses.
///
/// E.g. when the user drag-n-drop a socket to create a connection.
pub(crate) fn handle_socket_responses<S>(
    dragged_socket_id: &mut Option<S>,
    rendered_sockets: &[RenderedSocket<S>],
) -> SocketInteraction<S>
where
    S: Clone + PartialEq,
{
    let mut interaction = SocketInteraction::None;

    if let Some(socket_id) = dragged_socket_id.as_ref() {
        // There is a socket being dragged.

        let dragged_socket = rendered_sockets.iter().find(|s| &s.id == socket_id);

        if let Some(socket) = dragged_socket {
            // Check the response of the dragged socket.

            if socket.response.drag_stopped() {
                // The drag has stopped.

                let hovered = rendered_sockets.iter().find(|s| s.response.hovered());

                if let Some(hovered_socket) = hovered {
                    // Another socket contains the pointer, the user want to connect the sockets.

                    interaction =
                        SocketInteraction::Connect(socket_id.clone(), hovered_socket.id.clone());
                } else {
                    // The pointer is not on any socket.
                    // Do nothing.
                }

                // In all cases, reset the state.
                *dragged_socket_id = None;
            } else {
                // The dragging is still happening.

                // Draw the on-going connection.

                let hovered = rendered_sockets
                    .iter()
                    .find(|s| s.response.contains_pointer());

                if let Some(pointer_pos) = socket.response.interact_pointer_pos() {
                    interaction = SocketInteraction::InProgress(ConnectionInProgress {
                        source: socket.clone(),
                        target: hovered.cloned(),
                        pointer_pos,
                    });
                }
            }
        } else {
            // The currently dragged socket has been removed.
            *dragged_socket_id = None;
        }
    } else if let Some(socket) = rendered_sockets.iter().find(|s| s.response.drag_started()) {
        // A socket is being dragged.
        *dragged_socket_id = Some(socket.id.clone());
    }

    interaction
}

/* -------------------------------------------------------------------------- */

/// The shape of a socket's handle.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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

/// Create a [`Shape`] for a socket.
pub(crate) fn make_shape(
    shape: SocketShape,
    center: Pos2,
    width: f32,
    color: Color32,
    is_connected: bool,
) -> Shape {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_3};

    let fill = if is_connected {
        color
    } else {
        Color32::default()
    };

    let stroke = Stroke::new(1.0, color);

    match shape {
        SocketShape::Circle => Shape::Circle(CircleShape {
            center,
            radius: width / 2.0,
            fill,
            stroke,
        }),

        SocketShape::Square => Shape::Rect(RectShape {
            rect: Rect::from_center_size(center, Vec2::splat(width * FRAC_1_SQRT_2)),
            fill,
            stroke,
            rounding: Rounding::default(),
            fill_texture_id: egui::TextureId::default(),
            uv: Rect::ZERO,
            blur_width: 0.0,
        }),

        SocketShape::Triangle => Shape::Path(PathShape {
            points: vec![
                center + (width / 2.0) * vec2(f32::cos(0.0), f32::sin(0.0)),
                center + (width / 2.0) * vec2(f32::cos(2.0 * FRAC_PI_3), f32::sin(2.0 * FRAC_PI_3)),
                center + (width / 2.0) * vec2(f32::cos(4.0 * FRAC_PI_3), f32::sin(4.0 * FRAC_PI_3)),
            ],

            closed: true,
            fill,
            stroke: stroke.into(),
        }),
    }
}

/* -------------------------------------------------------------------------- */
