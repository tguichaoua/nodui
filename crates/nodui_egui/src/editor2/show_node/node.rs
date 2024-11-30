use egui::{epaint::RectShape, vec2, Pos2, Rect, Response, Rounding, Vec2};

use crate::{
    editor2::{
        header::{Header, TitleHeader},
        socket::RenderedSocket,
    },
    misc::{collector::Collector, layout},
    Socket,
};

use super::render::{self, body::PreparedBody, header::PreparedHeader};
use super::GraphUi;

/* -------------------------------------------------------------------------- */

/// The rounding of a node.
const NODE_ROUNDING: Rounding = Rounding::same(5.0);

/* -------------------------------------------------------------------------- */

pub struct NodeUi<S> {
    header: Header,
    layout: nodui_core::ui::NodeLayout,
    sockets: Vec<Socket<S>>,
    outline: egui::Stroke,
}

pub struct NodeResponse<'a, R, S> {
    pub inner: R,
    pub response: Response,
    pub sockets: &'a [RenderedSocket<S>],
}

impl<S> GraphUi<S> {
    #[inline]
    pub fn node<'a, R>(
        &mut self,
        id_salt: impl core::hash::Hash,
        pos: &mut nodui_core::Pos,
        build_fn: impl FnOnce(&mut NodeUi<S>) -> R + 'a,
    ) -> NodeResponse<'_, R, S>
    where
        S: core::hash::Hash,
    {
        let mut node_ui = NodeUi::new();
        let inner = build_fn(&mut node_ui);
        let node = self.ui.fonts(|fonts| node_ui.prepare(fonts));

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
    fn new() -> NodeUi<S> {
        NodeUi {
            header: Header::None,
            layout: nodui_core::ui::NodeLayout::Double,
            sockets: Vec::new(),
            outline: egui::Stroke::new(0.5, egui::Color32::WHITE),
        }
    }

    fn prepare(self, fonts: &egui::text::Fonts) -> PreparedNode<S> {
        let Self {
            header,
            layout,
            sockets,
            outline,
        } = self;

        let header = render::header::prepare(header, fonts);
        let sockets = sockets
            .into_iter()
            .map(|s| render::socket::prepare(s, fonts))
            .collect();
        let body = render::body::prepare(layout, sockets);

        PreparedNode {
            header,
            body,
            outline,
        }
    }
}

impl<S> NodeUi<S> {
    #[inline]
    pub fn header_title(
        &mut self,
        text: impl Into<String>,
        text_color: impl Into<egui::Color32>,
        background: impl Into<egui::Color32>,
    ) {
        let text = text.into();
        let text_color = text_color.into();
        let background = background.into();

        self.header = Header::Title(TitleHeader {
            text,
            text_color,
            background,
        });
    }

    #[inline]
    pub fn layout(&mut self, layout: nodui_core::ui::NodeLayout) {
        self.layout = layout;
    }

    #[inline]
    pub fn double_column_layout(&mut self) {
        self.layout = nodui_core::ui::NodeLayout::Double;
    }

    #[inline]
    pub fn single_column_layout(&mut self) {
        self.layout = nodui_core::ui::NodeLayout::Single;
    }

    #[inline]
    pub fn socket(&mut self, socket: Socket<S>) {
        self.sockets.push(socket);
    }
}

/* -------------------------------------------------------------------------- */

pub(super) struct PreparedNode<S> {
    header: PreparedHeader,
    body: PreparedBody<S>,
    outline: egui::Stroke,
}

impl<S> PreparedNode<S> {
    pub(super) fn size(&self) -> Vec2 {
        layout::stack_vertically([self.header.size(), self.body.size()])
    }

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
