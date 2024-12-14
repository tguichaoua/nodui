//! Rendering of connections.

use egui::{epaint::PathStroke, Color32, LayerId, Shape};

use crate::ConnectionInProgress;

use super::{stages, GraphEditor, GraphResponse, RenderedSocket};

/* -------------------------------------------------------------------------- */

impl<S> GraphEditor<stages::Connections<S>> {
    /// Render the connections between the sockets.
    #[inline]
    pub fn show_connections(self, build_fn: impl FnOnce(&mut ConnectionsUi<S>)) -> GraphResponse<S>
    where
        S: Send + Sync + Clone + 'static,
    {
        let Self {
            id,
            stage:
                stages::Connections {
                    ui,
                    state,
                    response,
                    viewport,
                    sockets,
                    socket_interaction,
                },
        } = self;

        let (connection, in_progress) = match socket_interaction {
            crate::socket::SocketInteraction::None => (None, None),
            crate::socket::SocketInteraction::Connect(a, b) => (Some((a, b)), None),
            crate::socket::SocketInteraction::InProgress(in_progress) => (None, Some(in_progress)),
        };

        let layer_id = LayerId::new(egui::Order::Background, id);
        let mut painter = ui.painter().clone();
        painter.set_layer_id(layer_id);

        let mut connections_ui = ConnectionsUi {
            preferred_color: ui.visuals().strong_text_color(),
            painter,
            sockets,
            connection: in_progress,
        };
        build_fn(&mut connections_ui);

        // If user didn't render the in progress connection, we do it for them.
        connections_ui
            .in_progress_connection_line(egui::Stroke::new(5.0, connections_ui.preferred_color()));

        let ConnectionsUi {
            preferred_color: _,
            painter: _,
            sockets,
            connection: _,
        } = connections_ui;

        let position = viewport.grid.canvas_to_graph(state.viewport_position);

        state.store(ui.ctx(), id);

        GraphResponse {
            viewport,
            response,
            sockets,
            connection,
            position,
        }
    }
}

/* -------------------------------------------------------------------------- */

/// This is what you use to render the connections.
pub struct ConnectionsUi<S> {
    /// A good default color for connections that matches the current theme.
    preferred_color: Color32,
    /// The painter we want to render to.
    painter: egui::Painter,
    /// The rendered sockets.
    sockets: Vec<RenderedSocket<S>>,
    /// A in-progress connection that have to be rendered.
    connection: Option<ConnectionInProgress<S>>,
}

impl<S> ConnectionsUi<S> {
    /// A good default color for connections that matches the current theme.
    #[inline]
    pub fn preferred_color(&self) -> Color32 {
        self.preferred_color
    }
}

impl<S> ConnectionsUi<S> {
    /// Render the connection the user is currently doing.
    ///
    /// If called multiple times, only the first call will have effect.
    ///
    /// # Low-level API
    ///
    /// This methods is a low-level API, it gives you direct access to the [`egui::Painter`].
    /// Prefer the usage of other `in_progress_connection_*` methods instead.
    #[inline]
    pub fn in_progress_connection(
        &mut self,
        show: impl FnOnce(&egui::Painter, ConnectionInProgress<S>),
    ) {
        if let Some(connection) = self.connection.take() {
            let mut top_most_painter = self.painter.clone();
            top_most_painter.set_layer_id(LayerId::new(
                egui::Order::Tooltip,
                self.painter.layer_id().id,
            ));
            show(&top_most_painter, connection);
        }
    }

    /// Render the in-progress connection with a straight line.
    ///
    /// See [`Self::in_progress_connection`].
    #[inline]
    pub fn in_progress_connection_line(&mut self, stroke: impl Into<egui::Stroke>) {
        self.in_progress_connection(|painter, connection| {
            let ConnectionInProgress {
                source,
                target: _,
                pointer_pos,
            } = connection;

            let points = [source.pos(), pointer_pos];
            let stroke = stroke.into().into();

            painter.add(Shape::LineSegment { points, stroke });
        });
    }

    /// Render the in-progress connection with a straight line with a stroke based on
    /// the source socket and the socket currently being hovered.
    ///
    /// See [`Self::in_progress_connection`].
    ///
    /// # Example
    ///
    /// In this example, we use the feedback callback to change the color and width of the line
    /// the signal the user the pointer is hovering a socket.
    ///
    /// ```
    /// # fn foo<S: PartialEq + Send + Sync + Clone + 'static>(graph: nodui::GraphEditor<nodui::stages::Connections::<S>>) {
    /// graph.show_connections(|ui| {
    ///     ui.in_progress_connection_line_with_feedback(|_, target| {
    ///         if target.is_some() {
    ///             egui::Stroke::new(5.0, egui::Color32::GREEN)
    ///         } else {
    ///             egui::Stroke::new(3.0, egui::Color32::WHITE)
    ///         }
    ///     });
    /// });
    /// # }
    /// ```
    #[inline]
    pub fn in_progress_connection_line_with_feedback(
        &mut self,
        stroke: impl FnOnce(RenderedSocket<S>, Option<RenderedSocket<S>>) -> egui::Stroke,
    ) {
        self.in_progress_connection(|painter, connection| {
            let ConnectionInProgress {
                source,
                target,
                pointer_pos,
            } = connection;

            let points = [source.pos(), pointer_pos];
            let stroke = stroke(source, target).into();

            painter.add(Shape::LineSegment { points, stroke });
        });
    }
}

impl<S> ConnectionsUi<S>
where
    S: PartialEq,
{
    /// Render the connection between two sockets.
    ///
    /// The method receive the id of the two sockets to connected and call the callback function
    /// which receive the [`Painter`](egui::Painter) to render to and the [`RenderedSocket`] of the
    /// two sockets.
    ///
    /// # Example
    ///
    /// Here's an example of connecting two sockets with a straight line using the
    /// color of the socket being at the right of its node.
    ///
    /// ```
    /// # fn foo(ui: &mut nodui::ConnectionsUi<()>, a: &(), b: &()) {
    /// ui.connect_with(a, b, |painter, a, b| {
    ///     let stroke = if a.side == nodui::NodeSide::Right {
    ///         egui::Stroke::new(3.0, a.color)
    ///     } else {
    ///         egui::Stroke::new(3.0, b.color)
    ///     };
    ///
    ///     painter.add(egui::Shape::LineSegment {
    ///         points: [a.pos(), b.pos()],
    ///         stroke: stroke.into(),
    ///     });
    /// });
    /// # }
    /// ```
    ///
    /// # Low-level API
    ///
    /// This methods is a low-level API, it gives you direct access to the [`egui::Painter`].
    /// Prefer the usage of other `connect_*` methods instead.
    #[inline]
    pub fn connect_with(
        &mut self,
        a: &S,
        b: &S,
        show: impl FnOnce(&egui::Painter, &RenderedSocket<S>, &RenderedSocket<S>),
    ) {
        if let Some(a) = self.sockets.iter().find(|s| &s.id == a) {
            if let Some(b) = self.sockets.iter().find(|s| &s.id == b) {
                show(&self.painter, a, b);
            }
        }
    }

    /// Render the connection between two sockets with a straight line.
    ///
    /// See [`Self::connect_with`].
    #[inline]
    pub fn connect_line(&mut self, a: &S, b: &S, stroke: impl Into<PathStroke>) {
        self.connect_with(a, b, |painter, a, b| {
            let stroke = stroke.into();
            painter.add(Shape::LineSegment {
                points: [a.pos(), b.pos()],
                stroke,
            });
        });
    }
}

/* -------------------------------------------------------------------------- */
