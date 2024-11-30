use egui::{epaint::PathStroke, Shape};

use super::{socket::ConnectionInProgress, stages, GraphEditor, RenderedSocket};

/* -------------------------------------------------------------------------- */

impl<S> GraphEditor<stages::Connections<S>> {
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
            crate::editor2::socket::SocketInteraction::None => (None, None),
            crate::editor2::socket::SocketInteraction::Connect(a, b) => (Some((a, b)), None),
            crate::editor2::socket::SocketInteraction::InProgress(in_progress) => {
                (None, Some(in_progress))
            }
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

pub struct ConnectionsUi<S> {
    painter: egui::Painter,
    sockets: Vec<RenderedSocket<S>>,
    connection: Option<ConnectionInProgress<S>>,
}

impl<S> ConnectionsUi<S>
where
    S: PartialEq,
{
    #[inline]
    pub fn in_progress_connection(
        &mut self,
        show: impl FnOnce(&egui::Painter, ConnectionInProgress<S>),
    ) {
        if let Some(connection) = self.connection.take() {
            show(&self.painter, connection);
        }
    }

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
    ///
    ///
    /// # Example
    ///
    /// Here's an example of connecting two sockets with a straight line using the
    /// color of the socket being at the right of its node.
    ///
    /// ```
    /// # fn foo(ui: &mut nodui_egui::ConnectionsUi<()>, a: &(), b: &()) {
    /// ui.connect_with(&a, &b, |painter, a, b| {
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
