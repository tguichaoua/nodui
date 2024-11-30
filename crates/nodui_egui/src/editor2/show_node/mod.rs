mod node;
mod render;

use egui::{Id, Vec2};

use crate::misc::collector::Collector;

use super::{stages, GraphEditor, RenderedSocket, Viewport};

pub use node::{NodeResponse, NodeUi};

/* -------------------------------------------------------------------------- */

pub struct GraphUi<S> {
    pub(super) graph_id: Id,
    pub(super) dragged_node: Option<(Id, Vec2)>,
    pub(super) viewport: Viewport,
    pub(super) ui: egui::Ui,
    pub(super) rendered_sockets: Collector<RenderedSocket<S>>,
}

/* -------------------------------------------------------------------------- */

impl<S> GraphEditor<stages::Nodes<S>>
where
    S: Clone + PartialEq,
{
    #[inline]
    pub fn show_nodes(
        self,
        build_fn: impl FnOnce(&mut GraphUi<S>),
    ) -> GraphEditor<stages::Connections<S>> {
        let Self {
            id,
            stage:
                stages::Nodes {
                    ui,
                    mut state,
                    viewport,
                    response,
                },
        } = self;

        // TODO
        let can_connect_socket = true;

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

        state.dragged_node = dragged_node;

        let sockets = rendered_sockets.into_vec();

        let socket_interaction = if can_connect_socket {
            crate::editor2::socket::handle_socket_responses(&mut state.dragged_socket, &sockets)
        } else {
            // Stop the currently dragged socket if creating connection is disabled.
            state.dragged_socket = None;
            crate::editor2::socket::SocketInteraction::None
        };

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

// impl GraphEditor {
//     #[inline]
//     #[allow(clippy::too_many_lines)] // TODO: refactorize this
//     pub fn show_nodes<'a, S, R>(
//         self,
//         ui: &mut egui::Ui,
//         build_fn: impl FnOnce(&mut GraphUi<S>) -> R + 'a,
//     ) -> ShowNodesResponse<S, R>
//     where
//         S: Clone + PartialEq + Send + Sync + 'static,
//     {
//         let Self {
//             id,

//             width,
//             height,
//             view_aspect,
//             min_size,
//         } = self;

//         // TODO
//         let background_color = Color32::BLACK;
//         let grid_stroke = Stroke::new(0.5, Color32::DARK_GRAY);
//         let can_connect_socket = true;
//         let look_at = None;

//         /* ---- */
//         let pos = ui.available_rect_before_wrap().min;

//         let size = {
//             let width = width
//                 .unwrap_or_else(|| {
//                     if let (Some(height), Some(aspect)) = (height, view_aspect) {
//                         height * aspect
//                     } else {
//                         ui.available_size_before_wrap().x
//                     }
//                 })
//                 .at_least(min_size.x);

//             let height = height
//                 .unwrap_or_else(|| {
//                     if let Some(aspect) = view_aspect {
//                         width / aspect
//                     } else {
//                         ui.available_size_before_wrap().y
//                     }
//                 })
//                 .at_least(min_size.y);

//             vec2(width, height)
//         };

//         let rect = Rect::from_min_size(pos, size);

//         ui.advance_cursor_after_rect(rect);

//         let mut ui = ui.new_child(UiBuilder::new().id_salt(id).max_rect(rect));
//         ui.set_clip_rect(rect);

//         /* ---- */
//         let mut state = GraphState::<S>::load(ui.ctx(), id);

//         /* ---- */
//         let response = ui.interact(rect, id, egui::Sense::click_and_drag());

//         if response.dragged() {
//             response.ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
//             state.viewport_position -= response.drag_delta();
//         }

//         let viewport = {
//             if let Some(look_at) = look_at {
//                 let pos = state.grid.graph_to_canvas(look_at);
//                 state.viewport_position = pos;
//             }

//             Viewport {
//                 position: rect.center().to_vec2() - state.viewport_position.to_vec2(),
//                 grid: state.grid.clone(),
//             }
//         };

//         /* ---- */
//         // Paint the background
//         ui.painter()
//             .add(RectShape::filled(rect, Rounding::ZERO, background_color));

//         /* ---- */
//         // Paint the grid
//         if !grid_stroke.is_empty() {
//             let dx = state.viewport_position.to_vec2().x % state.grid.size;
//             let dy = state.viewport_position.to_vec2().y % state.grid.size;

//             let center = rect.center() - vec2(dx, dy);

//             #[allow(clippy::cast_possible_truncation)]
//             let n = (size.x / state.grid.size) as i32 / 2;
//             #[allow(clippy::cast_possible_truncation)]
//             let m = (size.y / state.grid.size) as i32 / 2;

//             for x in (-n)..(n + 2) {
//                 #[allow(clippy::cast_precision_loss)]
//                 let x = x as f32;
//                 let x = x.mul_add(state.grid.size, center.x);

//                 ui.painter().add(Shape::LineSegment {
//                     points: [pos2(x, rect.min.y), pos2(x, rect.max.y)],
//                     stroke: grid_stroke.into(),
//                 });
//             }

//             for y in (-m)..(m + 2) {
//                 #[allow(clippy::cast_precision_loss)]
//                 let y = y as f32;
//                 let y = y.mul_add(state.grid.size, center.y);

//                 ui.painter().add(Shape::LineSegment {
//                     points: [pos2(rect.min.x, y), pos2(rect.max.x, y)],
//                     stroke: grid_stroke.into(),
//                 });
//             }
//         }

//         /* ---- */
//         let mut graph_ui = GraphUi {
//             ui,
//             graph_id: id,
//             dragged_node: state.dragged_node,
//             viewport,
//             rendered_sockets: Collector::new(),
//         };

//         let inner = build_fn(&mut graph_ui);

//         let GraphUi {
//             graph_id: _,
//             dragged_node,
//             viewport,
//             ui,
//             rendered_sockets,
//         } = graph_ui;

//         state.dragged_node = dragged_node;

//         let sockets = rendered_sockets.into_vec();

//         /* ---------------------------------------------- */
//         /* Handle socket responses                        */
//         /* ---------------------------------------------- */
//         let socket_interaction = if can_connect_socket {
//             handle_socket_responses(&mut state.dragged_socket, &sockets)
//         } else {
//             // Stop the currently dragged socket if creating connection is disabled.
//             state.dragged_socket = None;
//             SocketInteraction::None
//         };

//         /* ---- */
//         // TODO: handle connections

//         // {
//         //     let connections = graph
//         //         .connections()
//         //         .filter_map(|(a, b)| {
//         //             let a = socket_responses.get(&a)?;
//         //             let b = socket_responses.get(&b)?;
//         //             Some((a, b))
//         //         })
//         //         .map(|(a, b)| connection_renderer.socket_to_socket(a, b))
//         //         .collect::<Vec<_>>();

//         //     ui.painter().set(connections_shape_idx, connections);
//         // }

//         /* ---- */
//         // Stroke around the editor.
//         ui.painter().add(RectShape::stroke(
//             rect,
//             Rounding::ZERO,
//             (1.0, grid_stroke.color),
//         ));

//         /* ---- */
//         state.store(ui.ctx(), id);

//         /* ---- */
//         ShowNodesResponse {
//             id,
//             ui,
//             sockets,
//             inner,
//             response,
//             viewport,
//         }
//     }
// }

/* -------------------------------------------------------------------------- */
