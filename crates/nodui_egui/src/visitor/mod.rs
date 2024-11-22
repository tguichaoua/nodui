//! Implementation of the visitor to visit graphs.

mod body;
mod header;

use egui::{epaint::RectShape, vec2, Color32, LayerId, Rect, Response, Rounding, Sense, Ui, Vec2};
use nodui_core::{ui::NodeUI, NodeAdapter, NodeSeq, SizeHint};

use crate::{
    conversion::IntoEgui, editor::SocketResponses, misc::collect::Collect, viewport::Viewport,
};

/* -------------------------------------------------------------------------- */

// TODO: make those values customizable ?

/// The width of the socket's handle.
const SOCKET_WIDTH: f32 = 10.0;

/// The height of a socket line.
const ROW_HEIGHT: f32 = 20.0;

/// The rounding of a node.
const NODE_ROUNDING: Rounding = Rounding::same(5.0);

/// The default color to apply to the texts.
const DEFAULT_TEXT_COLOR: Color32 = Color32::WHITE;

/// The space reserved for the socket's field.
const SOCKET_FIELD_SIZE: Vec2 = vec2(40.0, ROW_HEIGHT);

/// Space between socket's name its socket shape.
const SOCKET_NAME_GAP: f32 = 5.0;

/// Space between socket's name and field.
const SOCKET_NAME_FIELD_GAP: f32 = 5.0;

/* -------------------------------------------------------------------------- */

/// A visitor to visit a graph and render its nodes.
pub(crate) struct GraphVisitor<'a, N, S, C> {
    /// The [`Ui`] used to render the nodes.
    pub(crate) ui: &'a mut Ui,
    /// The node currently being dragged and the delta position form it's current position
    pub(crate) dragged_node: &'a mut Option<(N, Vec2)>,
    /// The viewport of the editor.
    pub(crate) viewport: &'a Viewport,
    /// A reference to the id of the last interacted node, if any.
    pub(crate) last_interacted_node_id: &'a mut Option<N>,
    /// The socket responses.
    pub(crate) socket_responses: &'a mut SocketResponses<S>,
    /// A collector to collect nodes' response.
    pub(crate) collect_node_response: &'a mut C,
}

impl<'graph, N, S, C> nodui_core::GraphVisitor<'graph, N, S> for GraphVisitor<'_, N, S, C>
where
    N: nodui_core::Id,
    S: nodui_core::Id,
    C: Collect<(N, Response)>,
{
    fn nodes(&mut self, size_hint: SizeHint) -> impl NodeSeq<'graph, N, S> {
        self.collect_node_response.reserve(size_hint.min);

        GraphVisitor {
            ui: self.ui,
            dragged_node: self.dragged_node,
            viewport: self.viewport,
            last_interacted_node_id: self.last_interacted_node_id,
            socket_responses: self.socket_responses,
            collect_node_response: self.collect_node_response,
        }
    }
}

impl<'graph, N, S, C> NodeSeq<'graph, N, S> for GraphVisitor<'_, N, S, C>
where
    N: nodui_core::Id,
    S: nodui_core::Id,
    C: Collect<(N, Response)>,
{
    fn visit_node(&mut self, mut node: impl NodeAdapter<NodeId = N, SocketId = S>) {
        let Self {
            ui,
            dragged_node,
            viewport,
            last_interacted_node_id,
            socket_responses,
            collect_node_response,
        } = self;

        let id = node.id();

        let canvas_pos = {
            let delta_pos = match dragged_node.clone() {
                Some((dragged_id, delta_pos)) if dragged_id == id => delta_pos,
                _ => Vec2::ZERO,
            };

            viewport.grid.graph_to_canvas(node.pos()) + delta_pos
        };

        let pos = viewport.canvas_to_viewport(canvas_pos);

        let NodeUI {
            header,
            body,
            outline,
        } = node.ui();

        let node = &mut node;

        let (header, body) = ui.fonts(|fonts| {
            let header = header::prepare(fonts, header);
            let body = body::prepare(fonts, &body, node);

            (header, body)
        });

        let node_size = {
            let width = f32::max(header.size().x, body.size().x);
            let height = header.size().y + body.size().y;
            vec2(width, height)
        };

        let layer_id = LayerId::new(egui::Order::Middle, ui.id().with(id.clone()));

        let response = ui
            .with_layer_id(layer_id, |ui| {
                let response = ui.interact(
                    Rect::from_min_size(pos, node_size),
                    ui.id().with(id.clone()),
                    Sense::click_and_drag(),
                );

                let header_pos = pos;
                let body_pos = pos + vec2(0.0, header.size().y);

                let (header_rounding, body_rounding) =
                    split_rounding(NODE_ROUNDING, header.has_content());

                header.show(ui, header_pos, node_size, header_rounding);

                body.show(ui, body_pos, node_size, body_rounding, socket_responses);

                // Add a stroke around the node to make it easier to see.
                ui.painter().add(RectShape::stroke(
                    Rect::from_min_size(pos, node_size),
                    NODE_ROUNDING,
                    outline.into_egui(),
                ));

                response
            })
            .inner;

        if response.drag_stopped() {
            **dragged_node = None;
            let new_pos = canvas_pos + response.drag_delta();
            node.set_pos(viewport.grid.canvas_to_graph_nearest(new_pos));
        } else if response.drag_started() {
            **dragged_node = Some((id.clone(), response.drag_delta()));
        } else if response.dragged() {
            if let Some(dragged_node) = dragged_node.as_mut() {
                dragged_node.1 += response.drag_delta();
            }
        }

        if response.clicked || response.fake_primary_click || response.dragged() {
            **last_interacted_node_id = Some(id.clone());
            ui.ctx().move_to_top(layer_id);
        }

        collect_node_response.collect((id, response));
    }
}

/* -------------------------------------------------------------------------- */

/// Split the node rounding to the different parts of the node.
fn split_rounding(node_rounding: Rounding, has_header: bool) -> (Rounding, Rounding) {
    let Rounding { nw, ne, sw, se } = node_rounding;

    let top = Rounding {
        nw,
        ne,
        ..Default::default()
    };

    let bottom = Rounding {
        sw,
        se,
        ..Default::default()
    };

    if has_header {
        (top, bottom)
    } else {
        (Rounding::ZERO, node_rounding)
    }
}

/* -------------------------------------------------------------------------- */
