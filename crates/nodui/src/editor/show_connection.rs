//! Rendering of connections.

use egui::{epaint::PathStroke, Shape};

use crate::ConnectionInProgress;

use super::{stages, GraphEditor, RenderedSocket};

/* -------------------------------------------------------------------------- */

impl<S> GraphEditor<stages::Connections<S>> {
    /// Render the connections between the sockets.
    #[inline]
    pub fn show_connections(
        self,
        build_fn: impl FnOnce(&mut ConnectionsUi<S>),
    ) -> GraphEditor<stages::End<S>> {
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

        let layer_id = egui::LayerId::new(egui::Order::Background, id);
        let mut painter = ui.painter().clone();
        painter.set_layer_id(layer_id);

        let mut connections_ui = ConnectionsUi {
            painter,
            sockets,
            connection: in_progress,
        };
        build_fn(&mut connections_ui);
        let ConnectionsUi {
            painter,
            sockets,
            connection: in_progress,
        } = connections_ui;

        if let Some(connection) = in_progress {
            // The user didn't render the in progress connection, so we do it for them.

            painter.add(Shape::LineSegment {
                points: [connection.source.pos(), connection.pointer_pos],
                stroke: egui::Stroke::new(5.0, egui::Color32::WHITE).into(),
            });
        }

        GraphEditor {
            id,
            stage: stages::End {
                ui,
                state,
                viewport,
                response,
                sockets,
                connection,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */

/// This is what you use to render the connections.
pub struct ConnectionsUi<S> {
    /// The painter we want to render to.
    painter: egui::Painter,
    /// The rendered sockets.
    sockets: Vec<RenderedSocket<S>>,
    /// A in-progress connection that have to be rendered.
    connection: Option<ConnectionInProgress<S>>,
}

impl<S> ConnectionsUi<S>
where
    S: PartialEq,
{
    /// Render the connection the user is currently doing.
    #[inline]
    pub fn in_progress_connection(
        &mut self,
        show: impl FnOnce(&egui::Painter, ConnectionInProgress<S>),
    ) {
        if let Some(connection) = self.connection.take() {
            show(&self.painter, connection);
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
    /// # fn foo(graph: nodui_egui::GraphEditor<nodui_egui::editor::stages::Connections>) {
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
    /// # fn foo(ui: &mut nodui_egui::ConnectionsUi<()>, a: &(), b: &()) {
    /// ui.connect_with(a, b, |painter, a, b| {
    ///     let stroke = if a.side == nodui_core::ui::NodeSide::Right {
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
