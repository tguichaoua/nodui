use egui::{Color32, Pos2, Response};
use nodui_core::ui::{NodeSide, SocketShape};

#[derive(Debug, Clone)]
pub struct RenderedSocket<S> {
    pub id: S,
    pub response: Response,
    pub side: NodeSide,
    pub color: Color32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Socket<S> {
    pub id: S,
    pub side: NodeSide,
    pub text: String,
    pub text_color: Color32,
    pub filled: bool,
    pub shape: SocketShape,
    pub color: Color32,
}

impl<S> Socket<S> {
    #[inline]
    pub fn new(id: S, side: NodeSide) -> Self {
        Self {
            id,
            side,
            text: String::default(),
            text_color: Color32::BLACK,
            filled: false,
            shape: SocketShape::default(),
            color: Color32::WHITE,
        }
    }

    #[must_use]
    #[inline]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    #[must_use]
    #[inline]
    pub fn text_color(mut self, color: impl Into<Color32>) -> Self {
        self.text_color = color.into();
        self
    }

    #[must_use]
    #[inline]
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    #[must_use]
    #[inline]
    pub fn shape(mut self, shape: SocketShape) -> Self {
        self.shape = shape;
        self
    }

    #[must_use]
    #[inline]
    pub fn color(mut self, color: impl Into<Color32>) -> Self {
        self.color = color.into();
        self
    }
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

pub(crate) enum SocketInteraction<S> {
    None,
    Connect(S, S),
    InProgress(ConnectionInProgress<S>),
}

pub struct ConnectionInProgress<S> {
    pub source: RenderedSocket<S>,
    pub target: Option<RenderedSocket<S>>,
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
