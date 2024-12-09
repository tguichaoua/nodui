//! The rendering of the editor's viewport and the nodes.

mod header;
mod node;
mod render;

use egui::{epaint::RectShape, pos2, vec2, Id, Rect, Rounding, Shape, UiBuilder, Vec2};

use crate::misc::collector::Collector;

use super::{stages, state::EditorState, GraphEditor, RenderedSocket, Viewport};

pub use node::{NodeResponse, NodeUi};

/* -------------------------------------------------------------------------- */

/// This is what you use to render the nodes.
///
/// See [`GraphEditor::show_nodes`].
pub struct GraphUi<S> {
    /// The id of the graph editor.
    pub(super) graph_id: Id,
    /// The id and delta position of the node being dragged, id any.
    pub(super) dragged_node: Option<(Id, Vec2)>,
    /// The viewport of the editor.
    pub(super) viewport: Viewport,
    /// The [`egui::Ui`] used to render the editor.
    pub(super) ui: egui::Ui,
    /// The rendered sockets.
    pub(super) rendered_sockets: Collector<RenderedSocket<S>>,
}

/* -------------------------------------------------------------------------- */

impl GraphEditor<stages::Settings> {
    /// Shows the viewport of the editor.
    #[inline]
    pub fn show<S>(
        self,
        ui: &mut egui::Ui,
        build_fn: impl FnOnce(&mut GraphUi<S>),
    ) -> GraphEditor<stages::Connections<S>>
    where
        S: PartialEq + Send + Sync + Clone + 'static,
    {
        let Self {
            id,
            stage:
                stages::Settings {
                    grid_stroke,
                    background_color,
                    look_at,
                    can_connect_socket,
                    viewport,
                },
        } = self;

        /* ---- */

        let pos = ui.available_rect_before_wrap().min;
        let size = viewport.compute(ui);
        let rect = Rect::from_min_size(pos, size);

        ui.advance_cursor_after_rect(rect);

        let mut ui = ui.new_child(UiBuilder::new().id_salt(id).max_rect(rect));
        ui.set_clip_rect(rect);

        /* ---- */

        let mut state = EditorState::<S>::load(ui.ctx(), id);

        /* ---- */

        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if response.dragged() {
            response.ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            state.viewport_position -= response.drag_delta();
        }

        let viewport = {
            if let Some(look_at) = look_at {
                let pos = state.grid.graph_to_canvas(look_at);
                state.viewport_position = pos;
            }

            Viewport {
                position: rect.center().to_vec2() - state.viewport_position.to_vec2(),
                grid: state.grid.clone(),
            }
        };

        /* ---- */

        // Paint the background
        ui.painter()
            .add(RectShape::filled(rect, Rounding::ZERO, background_color));

        /* ---- */

        // Paint the grid
        if !grid_stroke.is_empty() {
            show_grid(
                ui.painter(),
                rect,
                state.viewport_position.to_vec2(),
                state.grid.size,
                grid_stroke,
            );
        }

        /* ---- */

        let mut graph_ui = GraphUi {
            ui,
            graph_id: id,
            dragged_node: state.dragged_node,
            viewport,
            rendered_sockets: Collector::new(),
        };

        build_fn(&mut graph_ui);

        let GraphUi {
            graph_id: _,
            dragged_node,
            viewport,
            ui,
            rendered_sockets,
        } = graph_ui;

        /* ---- */

        state.dragged_node = dragged_node;

        let sockets = rendered_sockets.into_vec();

        let socket_interaction = if can_connect_socket {
            crate::socket::handle_socket_responses(&mut state.dragged_socket, &sockets)
        } else {
            // Stop the currently dragged socket if creating connection is disabled.
            state.dragged_socket = None;
            crate::socket::SocketInteraction::None
        };

        /* ---- */

        GraphEditor {
            id,
            stage: stages::Connections {
                ui,
                state,
                viewport,
                response,
                sockets,
                socket_interaction,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */

/// Show the editor grid.
fn show_grid(
    painter: &egui::Painter,
    rect: Rect,
    position: Vec2,
    grid_size: f32,
    stroke: egui::Stroke,
) {
    let dx = position.x % grid_size;
    let dy = position.y % grid_size;

    let center = rect.center() - vec2(dx, dy);

    #[allow(clippy::cast_possible_truncation)]
    let n = (rect.width() / grid_size) as i32 / 2;
    #[allow(clippy::cast_possible_truncation)]
    let m = (rect.height() / grid_size) as i32 / 2;

    for x in (-n)..(n + 2) {
        #[allow(clippy::cast_precision_loss)]
        let x = x as f32;
        let x = x.mul_add(grid_size, center.x);

        painter.add(Shape::LineSegment {
            points: [pos2(x, rect.min.y), pos2(x, rect.max.y)],
            stroke: stroke.into(),
        });
    }

    for y in (-m)..(m + 2) {
        #[allow(clippy::cast_precision_loss)]
        let y = y as f32;
        let y = y.mul_add(grid_size, center.y);

        painter.add(Shape::LineSegment {
            points: [pos2(rect.min.x, y), pos2(rect.max.x, y)],
            stroke: stroke.into(),
        });
    }

    // Outline around the viewport
    painter.add(RectShape::stroke(rect, Rounding::ZERO, (1.0, stroke.color)));
}

/* -------------------------------------------------------------------------- */
