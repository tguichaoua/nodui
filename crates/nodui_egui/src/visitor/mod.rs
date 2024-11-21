#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

mod body;
mod header;

use std::collections::HashMap;

use egui::{
    epaint::RectShape, layers::ShapeIdx, vec2, Color32, LayerId, Rect, Response, Rounding, Sense,
    Ui, Vec2,
};
use nodui_core::{ui::NodeUI, visitor};

use crate::{
    conversion::IntoEgui,
    editor::{GraphMemory, NodePainter, SocketResponses},
    misc::collect::Collect,
    viewport::Viewport,
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
const SOCKET_FIELD_SIZE: Vec2 = vec2(50.0, ROW_HEIGHT);

/// Space between socket's name and field.
const SOCKET_NAME_FIELD_GAP: f32 = 5.0;

/* -------------------------------------------------------------------------- */

#[allow(clippy::too_many_arguments)] // TODO: refactor this
pub(crate) fn visit_graph<G>(
    graph: &mut G,
    ui: &mut Ui,
    state: &mut GraphMemory<G::NodeId, G::SocketId>,
    viewport: &Viewport,
    node_shape_indices: &HashMap<G::NodeId, ShapeIdx>,
    last_interacted_node_id: &mut Option<G::NodeId>,
    socket_responses: &mut SocketResponses<G::SocketId>,
    collect_node_response: &mut impl Collect<(G::NodeId, Response)>,
) where
    G: visitor::GraphAdapter,
{
    graph.accept(GraphVisitor {
        ui,
        state,
        viewport,
        node_shape_indices,
        last_interacted_node_id,
        socket_responses,
        collect_node_response,
    });
}

/* -------------------------------------------------------------------------- */

struct GraphVisitor<'a, N, S, C> {
    ui: &'a mut Ui,
    state: &'a mut GraphMemory<N, S>,
    viewport: &'a Viewport,
    node_shape_indices: &'a HashMap<N, ShapeIdx>,
    last_interacted_node_id: &'a mut Option<N>,
    socket_responses: &'a mut SocketResponses<S>,
    collect_node_response: &'a mut C,
}

impl<'graph, N, S, C> visitor::GraphVisitor<'graph, N, S> for GraphVisitor<'_, N, S, C>
where
    N: nodui_core::Id,
    S: nodui_core::Id,
    C: Collect<(N, Response)>,
{
    fn nodes(&mut self, size_hint: visitor::SizeHint) -> impl visitor::NodeSeq<'graph, N, S> {
        self.collect_node_response.reserve(size_hint.min());

        GraphVisitor {
            ui: self.ui,
            state: self.state,
            viewport: self.viewport,
            node_shape_indices: self.node_shape_indices,
            last_interacted_node_id: self.last_interacted_node_id,
            socket_responses: self.socket_responses,
            collect_node_response: self.collect_node_response,
        }
    }
}

impl<'graph, N, S, C> visitor::NodeSeq<'graph, N, S> for GraphVisitor<'_, N, S, C>
where
    N: nodui_core::Id,
    S: nodui_core::Id,
    C: Collect<(N, Response)>,
{
    fn visit_node(&mut self, mut node: impl visitor::NodeAdapter<NodeId = N, SocketId = S>) {
        let Self {
            ui,
            state,
            viewport,
            node_shape_indices,
            last_interacted_node_id,
            socket_responses,
            collect_node_response,
        } = self;

        let id = node.id();
        // If this is a new node, insert it on top, does nothing otherwise.
        state.node_order.insert(id.clone());

        let canvas_pos = {
            let delta_pos = match state.dragged_node.clone() {
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

        let response = ui.interact(
            Rect::from_min_size(pos, node_size),
            ui.id().with(id.clone()),
            Sense::click_and_drag(),
        );

        let header_pos = pos;
        let body_pos = pos + vec2(0.0, header.size().y);

        let (header_rounding, body_rounding) = split_rounding(NODE_ROUNDING, header.has_content());

        let mut painter = NodePainter::new();

        header.show(&mut painter, header_pos, node_size, header_rounding);

        body.show(
            ui,
            &mut painter,
            body_pos,
            node_size,
            body_rounding,
            socket_responses,
        );

        // Add a stroke around the node to make it easier to see.
        painter.add(RectShape::stroke(
            Rect::from_min_size(pos, node_size),
            NODE_ROUNDING,
            outline.into_egui(),
        ));

        if let Some(shape_id) = node_shape_indices.get(&id).copied() {
            ui.painter().set(shape_id, painter);
        } else {
            ui.painter().add(painter);
        }

        if response.drag_stopped() {
            state.dragged_node = None;
            let new_pos = canvas_pos + response.drag_delta();
            node.set_pos(viewport.grid.canvas_to_graph_nearest(new_pos));
        } else if response.drag_started() {
            state.dragged_node = Some((id.clone(), response.drag_delta()));
        } else if response.dragged() {
            if let Some(dragged_node) = state.dragged_node.as_mut() {
                dragged_node.1 += response.drag_delta();
            }
        }

        if response.clicked || response.fake_primary_click || response.dragged() {
            **last_interacted_node_id = Some(id.clone());
            state.set_node_on_top(id.clone());
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
