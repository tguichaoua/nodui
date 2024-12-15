//! Rendering of node's body.

use egui::{epaint::RectShape, vec2, Color32, Margin, Pos2, Rect, Rounding, Vec2};

use crate::{
    misc::{collector::Collector, layout},
    NodeLayout, NodeSide, RenderedSocket,
};

use super::{socket::PreparedSocket, SOCKET_NAME_GAP, SOCKET_WIDTH};

/* -------------------------------------------------------------------------- */

/// The prepared data for a node body.
pub(crate) struct PreparedBody<S> {
    /// The sockets to render to the body.
    sockets: Vec<PreparedSocket<S>>,
    /// The size required to render the body.
    size: Vec2,
    /// The layout to use to render the sockets.
    layout: NodeLayout,
    /// The color of the background of the node.
    background_color: Color32,

    /// The padding around the body.
    padding: Margin,
}

impl<S> PreparedBody<S> {
    /// The space occupied by the body.
    pub(crate) fn size(&self) -> Vec2 {
        self.size
    }
}

/* -------------------------------------------------------------------------- */

/// Prepare the node body for its rendering.
pub(crate) fn prepare<S>(
    spacing: &egui::Spacing,
    background_color: Color32,
    layout: NodeLayout,
    sockets: Vec<PreparedSocket<S>>,
) -> PreparedBody<S> {
    let padding = Margin::same(5.0);
    let socket_vertical_gap = spacing.item_spacing.y;

    let size: Vec2 = match layout {
        NodeLayout::Single => layout::stack_vertically_with_gap(
            sockets.iter().map(PreparedSocket::compute_size),
            socket_vertical_gap,
        ),
        NodeLayout::Double => {
            let left = layout::stack_vertically_with_gap(
                sockets
                    .iter()
                    .filter(|s| s.side == NodeSide::Left)
                    .map(PreparedSocket::compute_size),
                socket_vertical_gap,
            );

            let right = layout::stack_vertically_with_gap(
                sockets
                    .iter()
                    .filter(|s| s.side == NodeSide::Right)
                    .map(PreparedSocket::compute_size),
                socket_vertical_gap,
            );

            let column_gap = if left == Vec2::ZERO || right == Vec2::ZERO {
                Vec2::ZERO
            } else {
                vec2(spacing.item_spacing.x, 0.0)
            };

            layout::stack_horizontally([left, column_gap, right])
        }
    };

    let size = size + padding.sum();

    PreparedBody {
        sockets,
        size,

        layout,
        background_color,

        padding,
    }
}

/* -------------------------------------------------------------------------- */

impl<S> PreparedBody<S>
where
    S: core::hash::Hash,
{
    /// Render the body.
    pub(crate) fn show(
        self,
        ui: &mut egui::Ui,
        pos: Pos2,
        node_size: Vec2,
        rounding: Rounding,
        rendered_sockets: &mut Collector<RenderedSocket<S>>,
    ) {
        let Self {
            sockets,
            size,
            layout,
            background_color,
            padding,
        } = self;

        let rect = Rect::from_min_size(pos, vec2(node_size.x, size.y));
        ui.painter()
            .add(RectShape::filled(rect, rounding, background_color));

        {
            let rect = Rect::from_min_size(pos, vec2(node_size.x, size.y));
            let rect = rect - padding;

            match layout {
                NodeLayout::Single => {
                    show_single_column_body(ui, rendered_sockets, sockets, rect);
                }
                NodeLayout::Double => {
                    show_double_column_body(ui, rendered_sockets, sockets, rect);
                }
            }
        }
    }
}

/// Defines the position of the elements of a socket.
#[derive(Clone, Copy)]
struct SocketGeometry {
    /// The `x` coordinates of the socket's shape relative from the socket position.
    socket_x: f32,
    /// The `x` coordinates of the socket's name relative from the socket position.
    text_x: f32,
}

/// Defines the position of the elements of the sockets.
#[derive(Clone, Copy)]
struct SocketGeometries {
    /// The geometry of a left socket.
    left: SocketGeometry,
    /// The geometry of a right socket.
    right: SocketGeometry,
}

/// Computes the socket geometry based on the width available.
fn compute_socket_geometries(width: f32) -> SocketGeometries {
    SocketGeometries {
        left: SocketGeometry {
            socket_x: 0.0,
            text_x: SOCKET_WIDTH + SOCKET_NAME_GAP,
        },
        right: SocketGeometry {
            socket_x: width - SOCKET_WIDTH,
            text_x: width - (SOCKET_WIDTH + SOCKET_NAME_GAP),
        },
    }
}

/// Render the node body with a single column layout.
fn show_single_column_body<S>(
    ui: &mut egui::Ui,
    rendered_sockets: &mut Collector<RenderedSocket<S>>,
    sockets: Vec<PreparedSocket<S>>,
    rect: Rect,
) where
    S: core::hash::Hash,
{
    let geometry = compute_socket_geometries(rect.width());
    let mut pos = rect.min;

    for socket in sockets {
        let geometry = match socket.side {
            NodeSide::Left => geometry.left,
            NodeSide::Right => geometry.right,
        };

        show_socket(ui, rendered_sockets, &mut pos, geometry, socket);
    }
}

/// Render the node body with a double columns layout.
fn show_double_column_body<S>(
    ui: &mut egui::Ui,
    rendered_sockets: &mut Collector<RenderedSocket<S>>,
    sockets: Vec<PreparedSocket<S>>,
    rect: Rect,
) where
    S: core::hash::Hash,
{
    let geometry = compute_socket_geometries(rect.width());
    let mut left = rect.min;
    let mut right = rect.min;

    for socket in sockets {
        let (pos, geometry) = match socket.side {
            NodeSide::Left => (&mut left, geometry.left),
            NodeSide::Right => (&mut right, geometry.right),
        };

        show_socket(ui, rendered_sockets, pos, geometry, socket);
    }
}

/// Render a socket.
fn show_socket<S>(
    ui: &mut egui::Ui,
    rendered_sockets: &mut Collector<RenderedSocket<S>>,
    pos: &mut Pos2,
    geometry: SocketGeometry,
    socket: PreparedSocket<S>,
) where
    S: core::hash::Hash,
{
    let size = socket.compute_size();
    let socket_center = *pos + vec2(geometry.socket_x + SOCKET_WIDTH / 2.0, size.y / 2.0);
    let text_pos = *pos + vec2(geometry.text_x, (size.y - socket.text.rect.height()) / 2.0);

    pos.y += size.y + ui.spacing().item_spacing.y;

    let PreparedSocket {
        id,
        side,
        text,
        filled: is_connected,
        color,
        shape,
    } = socket;

    {
        let rect = Rect::from_center_size(socket_center, Vec2::splat(SOCKET_WIDTH));
        let response = ui.interact(rect, ui.id().with(&id), egui::Sense::click_and_drag());
        let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        rendered_sockets.push(RenderedSocket {
            id,
            response,
            side,
            color,
        });
    }

    ui.painter().add(crate::socket::make_shape(
        shape,
        socket_center,
        SOCKET_WIDTH,
        color,
        is_connected,
    ));

    ui.painter().add(egui::Shape::galley(
        text_pos,
        text,
        ui.visuals().strong_text_color(),
    ));
}
