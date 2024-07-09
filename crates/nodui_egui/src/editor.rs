//! The visual editor.

use std::collections::HashMap;
use std::hash::Hash;

use egui::epaint::RectShape;
use egui::{
    pos2, vec2, Color32, CursorIcon, NumExt, Rect, Response, Rounding, Sense, Shape, Stroke, Ui,
    Vec2,
};
use indexmap::IndexSet;
use nodui_core::adapter::{
    ConnectionHint, GraphAdapter, Id as NoduiId, NodeAdapter, NodeIterator, Pos,
};
use nodui_core::ui::NodeSide;

use crate::connection::{ConnectionRenderer, LineConnectionRenderer};
use crate::context_menu::{
    ContextMenuContent, MenuContext, NodeContextMenuContent, NodeMenuContext,
    SocketContextMenuContent, SocketMenuContext,
};
use crate::node;
use crate::socket::RenderedSocket;
use crate::viewport::{CanvasPos, Grid, Viewport};

/* -------------------------------------------------------------------------- */

// TODO: add usage example in docs of `GraphEditor`.

/// A graph editor to render and manipulate a graph throw the [`GraphAdapter`] trait.
#[allow(clippy::missing_docs_in_private_items)] // Too much fields (T.T)
pub struct GraphEditor<'a, G: GraphAdapter> {
    graph: G,
    id: egui::Id,

    width: Option<f32>,
    height: Option<f32>,
    view_aspect: Option<f32>,
    min_size: Vec2,

    grid_stroke: Stroke,
    background_color: Color32,

    look_at: Option<Pos>,

    connection_renderer: ConnectionRenderer,

    context_menu: Option<ContextMenuContent<'a, G>>,
    node_context_menu: Option<NodeContextMenuContent<'a, G>>,
    socket_context_menu: Option<SocketContextMenuContent<'a, G>>,
}

impl<'a, G: GraphAdapter> GraphEditor<'a, G> {
    /// Creates a new [`GraphEditor`].
    #[inline]
    pub fn new(graph: G, id_source: impl Hash) -> Self {
        Self {
            graph,
            id: egui::Id::new(id_source),

            width: None,
            height: None,
            view_aspect: None,
            min_size: Vec2::ZERO,

            grid_stroke: Stroke::new(0.5, Color32::DARK_GRAY),
            background_color: Color32::BLACK,

            look_at: None,

            connection_renderer: ConnectionRenderer::from(LineConnectionRenderer::default()),

            context_menu: None,
            node_context_menu: None,
            socket_context_menu: None,
        }
    }
}

impl<'a, G: GraphAdapter> GraphEditor<'a, G> {
    /// Move the viewport to make `pos` on the center of the viewport.
    #[inline]
    #[must_use]
    pub fn look_at(mut self, pos: Pos) -> Self {
        self.look_at = Some(pos);
        self
    }

    /// The stroke used to render the background grid.
    ///
    /// Use [`Stroke::NONE`] to disable the grid.
    #[inline]
    #[must_use]
    pub fn grid_stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.grid_stroke = stroke.into();
        self
    }

    /// `width / height` ratio of the editor region.
    ///
    /// By default no fixed aspect ratio is set (and width/height will fill the ui it is in).
    #[inline]
    #[must_use]
    pub fn view_aspect(mut self, view_aspect: f32) -> Self {
        self.view_aspect = Some(view_aspect);
        self
    }

    /// Width of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the width can be calculated from the height.
    #[inline]
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.min_size.x = width;
        self.width = Some(width);
        self
    }

    /// Height of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the height can be calculated from the width.
    #[inline]
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.min_size.y = height;
        self.height = Some(height);
        self
    }

    /// The [`ConnectionRenderer`] used to render the connections between sockets.
    ///
    /// It can be one of the renderers defined in [`connection`] or
    /// a [`CustomConnectionRenderer`].
    ///
    /// [`connection`]: crate::connection
    /// [`CustomConnectionRenderer`]: crate::connection::CustomConnectionRenderer
    #[inline]
    #[must_use]
    pub fn connection_renderer(
        mut self,
        connection_renderer: impl Into<ConnectionRenderer>,
    ) -> Self {
        self.connection_renderer = connection_renderer.into();
        self
    }

    /// Defines a callback used to render the context menu of the editor.
    ///
    /// See also [`Response::context_menu`].
    #[inline]
    #[must_use]
    pub fn context_menu(
        mut self,
        add_contents: impl FnMut(&mut Ui, MenuContext<'_, G>) + 'a,
    ) -> Self {
        self.context_menu = Some(Box::new(add_contents));
        self
    }

    /// Defines a callback used to render the context menu of a node.
    ///
    /// See also [`Response::context_menu`].
    #[inline]
    #[must_use]
    pub fn node_context_menu(
        mut self,
        add_contents: impl FnMut(&mut Ui, NodeMenuContext<'_, G>) + 'a,
    ) -> Self {
        self.node_context_menu = Some(Box::new(add_contents));
        self
    }

    /// Defines a callback used to render the context menu of a sockets.
    ///
    /// See also [`Response::context_menu`].
    #[inline]
    #[must_use]
    pub fn socket_context_menu(
        mut self,
        add_contents: impl FnMut(&mut Ui, SocketMenuContext<'_, G>) + 'a,
    ) -> Self {
        self.socket_context_menu = Some(Box::new(add_contents));
        self
    }
}

/* -------------------------------------------------------------------------- */

/// The state of the editor saved from on frame to another.
#[derive(Clone)]
struct GraphMemory<NodeId, SocketId> {
    /// The current viewport position.
    viewport_position: CanvasPos,
    /// The grid of the editor.
    grid: Grid,

    /// The node currently being dragged and the delta position form it's current position.
    dragged_node: Option<(NodeId, Vec2)>,
    /// The socket currently being dragged.
    dragged_socket: Option<SocketId>,

    /// The last know position of the pointer in graph coordinates.
    last_known_pointer_pos: Pos,

    /// The order in which render the node from back to top.
    node_order: IndexSet<NodeId>,
}

impl<N, S> Default for GraphMemory<N, S> {
    fn default() -> Self {
        Self {
            viewport_position: CanvasPos::ZERO,
            grid: Grid { size: 10.0 },
            dragged_node: None,
            dragged_socket: None,
            last_known_pointer_pos: Pos::default(),
            node_order: IndexSet::new(),
        }
    }
}

impl<N, S> GraphMemory<N, S>
where
    Self: Clone + Send + Sync + 'static,
{
    /// Loads the editor state.
    fn load(ctx: &egui::Context, id: egui::Id) -> Self {
        ctx.memory(|memory| memory.data.get_temp(id).unwrap_or_default())
    }

    /// Store the editor state.
    fn store(self, ctx: &egui::Context, id: egui::Id) {
        ctx.memory_mut(|memory| memory.data.insert_temp(id, self));
    }
}

impl<N: NoduiId, S> GraphMemory<N, S> {
    /// Move the specified node to the top of the nodes.
    fn set_node_on_top(&mut self, node_id: N) {
        self.node_order.shift_remove(&node_id);
        self.node_order.insert(node_id);
    }
}

/* -------------------------------------------------------------------------- */

/// A painter to render the shapes of a node.
pub(crate) struct NodePainter(Vec<Shape>);

impl NodePainter {
    /// Creates a [`NodePainter`].
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Adds a shape to this painter.
    pub(crate) fn add(&mut self, shape: impl Into<Shape>) {
        self.0.push(shape.into());
    }
}

impl From<NodePainter> for Shape {
    #[allow(clippy::missing_inline_in_public_items)] // `NodePainter` is private so it's needless.
    fn from(NodePainter(shapes): NodePainter) -> Self {
        Shape::Vec(shapes)
    }
}

/* -------------------------------------------------------------------------- */

/// The response of adding a [`GraphEditor`] to a [`Ui`].
pub struct GraphOutput {
    /// The response of the background of the editor background.
    pub response: Response,

    /// The graph position of the point on the middle of the viewport.
    pub position: Pos,

    /// The current viewport.
    viewport: Viewport,
}

impl GraphOutput {
    /// Latest reported pointer's graph position.
    ///
    /// Based on [`Context::pointer_latest_pos`](egui::Context::pointer_latest_pos).
    #[inline]
    #[must_use]
    pub fn pointer_latest_pos(&self) -> Option<Pos> {
        self.response
            .ctx
            .pointer_latest_pos()
            .map(|pointer| self.viewport.viewport_to_graph(pointer))
    }
}

/* -------------------------------------------------------------------------- */

impl<'a, G: GraphAdapter> GraphEditor<'a, G> {
    /// Show the graph editor.
    #[allow(clippy::too_many_lines)] // TODO: split this method for readability
    #[inline]
    pub fn show(self, ui: &mut Ui) -> GraphOutput {
        let Self {
            mut graph,
            id,

            width,
            height,
            view_aspect,
            min_size,

            grid_stroke,
            background_color,

            look_at,

            connection_renderer,

            mut context_menu,
            mut node_context_menu,
            mut socket_context_menu,
        } = self;

        /* ---- */

        let pos = ui.available_rect_before_wrap().min;

        let size = {
            let width = width
                .unwrap_or_else(|| {
                    if let (Some(height), Some(aspect)) = (height, view_aspect) {
                        height * aspect
                    } else {
                        ui.available_size_before_wrap().x
                    }
                })
                .at_least(min_size.x);

            let height = height
                .unwrap_or_else(|| {
                    if let Some(aspect) = view_aspect {
                        width / aspect
                    } else {
                        ui.available_size_before_wrap().y
                    }
                })
                .at_least(min_size.y);

            vec2(width, height)
        };

        let rect = Rect::from_min_size(pos, size);

        ui.advance_cursor_after_rect(rect);

        let mut ui = ui.child_ui_with_id_source(rect, *ui.layout(), id, None);
        ui.set_clip_rect(rect);

        /* ---- */

        let mut state = GraphMemory::<G::NodeId, G::SocketId>::load(ui.ctx(), id);

        /* ---- */

        let response = ui.interact(rect, id, Sense::click_and_drag());

        if response.dragged() {
            response.ctx.set_cursor_icon(CursorIcon::Grabbing);
            state.viewport_position -= response.drag_delta();
        }

        let viewport = {
            if let Some(look_at) = look_at {
                let pos = state.grid.graph_to_canvas(look_at);
                // state.viewport_position = -pos.to_vec2();
                state.viewport_position = pos;
            }

            Viewport {
                position: rect.center().to_vec2() - state.viewport_position.to_vec2(),
                grid: state.grid.clone(),
            }
        };

        if let Some(context_menu) = context_menu.as_mut() {
            response.context_menu(|ui| {
                if let Some(pointer) = response.interact_pointer_pos() {
                    state.last_known_pointer_pos = viewport.viewport_to_graph(pointer);
                }
                context_menu(
                    ui,
                    MenuContext {
                        graph: &mut graph,
                        pos: state.last_known_pointer_pos,
                    },
                );
            });
        }

        /* ---- */

        // Paint the background
        ui.painter()
            .add(RectShape::filled(rect, Rounding::ZERO, background_color));

        /* ---- */

        // Paint the grid

        if !grid_stroke.is_empty() {
            let dx = state.viewport_position.to_vec2().x % state.grid.size;
            let dy = state.viewport_position.to_vec2().y % state.grid.size;

            let center = rect.center() - vec2(dx, dy);

            #[allow(clippy::cast_possible_truncation)]
            let n = (size.x / state.grid.size) as i32 / 2;
            #[allow(clippy::cast_possible_truncation)]
            let m = (size.y / state.grid.size) as i32 / 2;

            for x in (-n)..(n + 2) {
                #[allow(clippy::cast_precision_loss)]
                let x = x as f32;
                let x = x.mul_add(state.grid.size, center.x);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(x, rect.min.y), pos2(x, rect.max.y)],
                    stroke: grid_stroke.into(),
                });
            }

            for y in (-m)..(m + 2) {
                #[allow(clippy::cast_precision_loss)]
                let y = y as f32;
                let y = y.mul_add(state.grid.size, center.y);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                    stroke: grid_stroke.into(),
                });
            }
        }

        /* ---- */

        let connections_shape_idx = ui.painter().add(Shape::Noop);

        // Reserve space to draw nodes
        let node_shape_indices: HashMap<_, _> = state
            .node_order
            .iter()
            .cloned()
            .map(|node_id| {
                let shape_id = ui.painter().add(Shape::Noop);
                (node_id, shape_id)
            })
            .collect();

        // Paints the nodes and collect the nodes and sockets responses.
        let mut socket_responses: SocketResponses<G::SocketId> = SocketResponses::new();

        let mut nodes = graph.nodes();
        let mut node_responses = Vec::with_capacity(nodes.size_hint().0);

        while let Some(mut node) = nodes.next() {
            let node_id = node.id();

            // If this is a new node, insert it on top, does nothing otherwise.
            state.node_order.insert(node_id.clone());

            let pos = {
                let delta_pos = match state.dragged_node.clone() {
                    Some((id, delta_pos)) if id == node_id => delta_pos,
                    _ => Vec2::ZERO,
                };

                viewport.grid.graph_to_canvas(node.pos()) + delta_pos
            };

            let mut node_painter = NodePainter::new();

            let node_response = node::show_node(
                &mut ui,
                node_id.clone(),
                node.ui(),
                &mut socket_responses,
                node.sockets(),
                viewport.canvas_to_viewport(pos),
                &mut node_painter,
            );

            if let Some(shape_id) = node_shape_indices.get(&node_id).copied() {
                ui.painter().set(shape_id, node_painter);
            } else {
                ui.painter().add(node_painter);
            }

            if node_response.drag_stopped() {
                state.dragged_node = None;
                let new_pos = pos + node_response.drag_delta();
                node.set_pos(viewport.grid.canvas_to_graph_nearest(new_pos));
            } else if node_response.drag_started() {
                state.dragged_node = Some((node_id.clone(), node_response.drag_delta()));
            } else if node_response.dragged() {
                if let Some(dragged_node) = state.dragged_node.as_mut() {
                    dragged_node.1 += node_response.drag_delta();
                }
            }

            if node_response.clicked() || node_response.dragged() {
                state.set_node_on_top(node_id.clone());
            }

            node_responses.push((node_id, node_response));
        }

        drop(nodes);

        if let Some(context_menu) = node_context_menu.as_mut() {
            for (id, response) in node_responses {
                response.context_menu(|ui| {
                    context_menu(
                        ui,
                        NodeMenuContext {
                            graph: &mut graph,
                            node_id: id,
                        },
                    );
                });
            }
        }

        /* ---------------------------------------------- */
        /* Handle socket responses                        */
        /* ---------------------------------------------- */

        if let Some(socket_id) = state.dragged_socket.as_ref() {
            // There is a socket being dragged.

            if let Some(socket) = socket_responses.get(socket_id) {
                // Check the response of the dragged socket.

                if socket.response.drag_stopped() {
                    // The drag has stopped.

                    if let Some((hovered_id, _)) = socket_responses.contains_pointer() {
                        // Another socket contains the pointer, the user want to connect the sockets.

                        graph.connect(socket_id.clone(), hovered_id.clone());
                    } else {
                        // The pointer is not on any socket.
                    }

                    // Reset the state.
                    state.dragged_socket = None;
                } else {
                    // The dragging is still happening.

                    // Draw the on-going connection.

                    let hint = if let Some((other_id, _)) = socket_responses.contains_pointer() {
                        let hint = graph.connection_hint(socket_id.clone(), other_id.clone());

                        if let ConnectionHint::Reject = hint {
                            ui.ctx().set_cursor_icon(CursorIcon::NoDrop);
                        }

                        Some(hint)
                    } else {
                        None
                    };

                    if let Some(pointer_pos) = socket.response.interact_pointer_pos() {
                        ui.painter().add(connection_renderer.socket_to_pointer(
                            socket,
                            pointer_pos,
                            hint,
                        ));
                    }
                }
            } else {
                // The currently dragged socket has been removed.
                state.dragged_socket = None;
            }
        } else if let Some((id, _)) = socket_responses.drag_started() {
            // A socket is being dragged.
            state.dragged_socket = Some(id.clone());
        }

        if let Some(context_menu) = socket_context_menu.as_mut() {
            for (socket_id, RenderedSocket { response, .. }) in &socket_responses.0 {
                response.context_menu(|ui| {
                    context_menu(
                        ui,
                        SocketMenuContext {
                            graph: &mut graph,
                            socket_id: socket_id.clone(),
                        },
                    );
                });
            }
        }

        /* ---- */

        {
            let connections = graph
                .connections()
                .filter_map(|(a, b)| {
                    let a = socket_responses.get(&a)?;
                    let b = socket_responses.get(&b)?;
                    Some((a, b))
                })
                .map(|(a, b)| connection_renderer.socket_to_socket(a, b))
                .collect::<Vec<_>>();

            ui.painter().set(connections_shape_idx, connections);
        }

        /* ---- */

        ui.painter().add(RectShape::stroke(
            rect,
            Rounding::ZERO,
            (1.0, grid_stroke.color),
        ));

        /* ---- */

        let output = GraphOutput {
            response,
            position: viewport.grid.canvas_to_graph(state.viewport_position),
            viewport,
        };

        /* ---- */

        state.store(ui.ctx(), id);

        /* ---- */

        output
    }
}

/* -------------------------------------------------------------------------- */

/// A collector to save the rendered socket data.
pub(crate) struct SocketResponses<SocketId>(HashMap<SocketId, RenderedSocket>);

impl<SocketId: NoduiId> SocketResponses<SocketId> {
    /// Creates a [`SocketResponses`].
    fn new() -> Self {
        Self(HashMap::new())
    }

    /// Gets on socket from its identifier.
    fn get(&self, socket_id: &SocketId) -> Option<&RenderedSocket> {
        self.0.get(socket_id)
    }

    /// Inserts a rendered socket.
    pub(crate) fn insert(
        &mut self,
        socket_id: SocketId,
        response: Response,
        color: Color32,
        side: NodeSide,
    ) {
        self.0.insert(
            socket_id,
            RenderedSocket {
                response,
                color,
                side,
            },
        );
    }

    /// Gets, if any, the socket that contains the pointer.
    fn contains_pointer(&self) -> Option<(&SocketId, &RenderedSocket)> {
        self.0
            .iter()
            .find(|(_, socket)| socket.response.contains_pointer())
    }

    /// Gets, if any, the socket which start being dragged.
    fn drag_started(&self) -> Option<(&SocketId, &RenderedSocket)> {
        self.0
            .iter()
            .find(|(_, socket)| socket.response.drag_started())
    }
}

/* -------------------------------------------------------------------------- */
