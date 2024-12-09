//! Node rendering.

use egui::{epaint::RectShape, vec2, Pos2, Rect, Response, Rounding, Vec2};

use crate::{
    misc::{collector::Collector, layout},
    Header, NodeLayout, Pos, RenderedSocket, Socket,
};

use super::render::{self, body::PreparedBody, header::PreparedHeader};
use super::GraphUi;

/* -------------------------------------------------------------------------- */

/// The rounding of a node.
const NODE_ROUNDING: Rounding = Rounding::same(5.0);

/* -------------------------------------------------------------------------- */

/// This is what you use to render a node.
///
/// See [`GraphUi::node`].
pub struct NodeUi<S> {
    /// The header of the node.
    header: Header,
    /// The layout of the sockets.
    layout: NodeLayout,
    /// The sockets.
    sockets: Vec<Socket<S>>,
    /// The outline.
    outline: Option<egui::Stroke>,
}

/// What [`GraphUi::node`] returns.
pub struct NodeResponse<'a, R, S> {
    /// The result of the callback.
    pub inner: R,
    /// The [`Response`] of the node.
    pub response: Response,
    /// The rendered socket of the node.
    pub sockets: &'a [RenderedSocket<S>],
}

/* -------------------------------------------------------------------------- */

impl<S> GraphUi<S> {
    /// Render a node.
    ///
    /// `id_salt` must be a unique id for the node.
    #[inline]
    pub fn node<'a, R>(
        &mut self,
        id_salt: impl core::hash::Hash,
        pos: &mut Pos,
        build_fn: impl FnOnce(&mut NodeUi<S>) -> R + 'a,
    ) -> NodeResponse<'_, R, S>
    where
        S: core::hash::Hash,
    {
        let mut node_ui = NodeUi::new();
        let inner = build_fn(&mut node_ui);
        let node = self
            .ui
            .fonts(|fonts| node_ui.prepare(self.ui.visuals(), fonts));

        let id = self.graph_id.with(id_salt);

        let canvas_pos = {
            let delta_pos = match self.dragged_node {
                Some((dragged_id, delta_pos)) if dragged_id == id => delta_pos,
                _ => Vec2::ZERO,
            };

            self.viewport.grid.graph_to_canvas(*pos) + delta_pos
        };

        let ui_pos = self.viewport.canvas_to_viewport(canvas_pos);

        let node_size = node.size();

        let layer_id = egui::LayerId::new(egui::Order::Middle, id);

        let (response, sockets) = self
            .ui
            .with_layer_id(layer_id, |ui| {
                let response = ui.interact(
                    Rect::from_min_size(ui_pos, node_size),
                    id,
                    egui::Sense::click_and_drag(),
                );

                let (sockets, ()) = self.rendered_sockets.watch(|rendered_sockets| {
                    node.show(ui, ui_pos, rendered_sockets);
                });

                (response, sockets)
            })
            .inner;

        if response.drag_stopped() {
            self.dragged_node = None;
            let new_pos = canvas_pos + response.drag_delta();
            // node.set_pos(viewport.grid.canvas_to_graph_nearest(new_pos));
            *pos = self.viewport.grid.canvas_to_graph_nearest(new_pos);
        } else if response.drag_started() {
            self.dragged_node = Some((id, response.drag_delta()));
        } else if response.dragged() {
            if let Some(dragged_node) = self.dragged_node.as_mut() {
                dragged_node.1 += response.drag_delta();
            }
        }

        if response.clicked || response.fake_primary_click || response.dragged() {
            self.ui.ctx().move_to_top(layer_id);
        }

        NodeResponse {
            inner,
            response,
            sockets,
        }
    }
}

/* -------------------------------------------------------------------------- */

impl<S> NodeUi<S> {
    /// Creates a new [`NodeUi<S>`].
    fn new() -> NodeUi<S> {
        NodeUi {
            header: Header::None,
            layout: NodeLayout::Double,
            sockets: Vec::new(),
            outline: None,
        }
    }

    /// Do the computations required to render the node.
    fn prepare(self, visuals: &egui::Visuals, fonts: &egui::text::Fonts) -> PreparedNode<S> {
        let Self {
            header,
            layout,
            sockets,
            outline,
        } = self;

        let outline = outline.unwrap_or(visuals.window_stroke);

        let sockets = sockets
            .into_iter()
            .map(|s| render::socket::prepare(s, visuals, fonts))
            .collect();
        let body = render::body::prepare(visuals, layout, sockets);
        let header = render::header::prepare(header, body.background_color(), visuals, fonts);

        PreparedNode {
            header,
            body,
            outline,
        }
    }
}

impl<S> NodeUi<S> {
    /// Sets the header of the node.
    #[inline]
    pub fn header(&mut self, header: impl Into<Header>) {
        self.header = header.into();
    }

    /// Sets the layout for the sockets.
    #[inline]
    pub fn layout(&mut self, layout: NodeLayout) {
        self.layout = layout;
    }

    /// Use two columns for the sockets.
    #[inline]
    pub fn double_column_layout(&mut self) {
        self.layout = NodeLayout::Double;
    }

    /// Use a single column for the socket.
    #[inline]
    pub fn single_column_layout(&mut self) {
        self.layout = NodeLayout::Single;
    }

    /// Add a socket to the node.
    #[inline]
    pub fn socket(&mut self, socket: Socket<S>) {
        self.sockets.push(socket);
    }

    /// Sets the outline of the node.
    #[inline]
    pub fn outline(&mut self, outline: impl Into<egui::Stroke>) {
        self.outline = Some(outline.into());
    }
}

/* -------------------------------------------------------------------------- */

/// Computed data to render the node.
pub(super) struct PreparedNode<S> {
    /// Computed  data to render the header.
    header: PreparedHeader,
    /// Computed data to render the body.
    body: PreparedBody<S>,
    /// The outline of the node.
    outline: egui::Stroke,
}

impl<S> PreparedNode<S> {
    /// The space occupied by the node.
    pub(super) fn size(&self) -> Vec2 {
        layout::stack_vertically([self.header.size(), self.body.size()])
    }

    /// Render the node to the [`egui::Ui`].
    pub(super) fn show(
        self,
        ui: &mut egui::Ui,
        pos: Pos2,
        rendered_sockets: &mut Collector<RenderedSocket<S>>,
    ) where
        S: core::hash::Hash,
    {
        let size = self.size();

        let Self {
            header,
            body,
            outline,
        } = self;

        let header_pos = pos;
        let body_pos = pos + vec2(0.0, header.size().y);

        let (header_rounding, body_rounding) = split_rounding(NODE_ROUNDING, header.has_content());

        header.show(ui, header_pos, size, header_rounding);

        body.show(ui, body_pos, size, body_rounding, rendered_sockets);

        // Add a stroke around the node to make it easier to see.
        ui.painter().add(RectShape::stroke(
            Rect::from_min_size(pos, size),
            NODE_ROUNDING,
            outline,
        ));
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
