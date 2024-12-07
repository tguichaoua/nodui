//! Rendering of node's body.

use egui::{epaint::RectShape, vec2, Color32, Margin, Pos2, Rect, Rounding, Vec2};

use crate::{
    misc::{collector::Collector, layout},
    NodeLayout, NodeSide, RenderedSocket,
};

use super::{socket::PreparedSocket, ROW_HEIGHT, SOCKET_NAME_GAP, SOCKET_WIDTH};

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
pub(crate) fn prepare<S>(layout: NodeLayout, sockets: Vec<PreparedSocket<S>>) -> PreparedBody<S> {
    let background_color = Color32::from_rgba_unmultiplied(0, 0, 0, 170);
    let padding = Margin::same(5.0);
    let column_gap = 5.0;

    let size: Vec2 = match layout {
        NodeLayout::Single => {
            layout::stack_vertically(sockets.iter().map(PreparedSocket::compute_size))
        }
        NodeLayout::Double => {
            let mut left = Vec2::ZERO;
            let mut right = Vec2::ZERO;

            for s in &sockets {
                let size = match s.side {
                    NodeSide::Left => &mut left,
                    NodeSide::Right => &mut right,
                };

                *size = layout::stack_vertically([*size, s.compute_size()]);
            }

            let column_gap = vec2(0.0, column_gap);

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

/// Render the node body with a single column layout.
fn show_single_column_body<S>(
    ui: &mut egui::Ui,
    rendered_sockets: &mut Collector<RenderedSocket<S>>,
    sockets: Vec<PreparedSocket<S>>,
    rect: Rect,
) where
    S: core::hash::Hash,
{
    let mut pos = rect.min;

    for socket in sockets {
        // TODO: may be refactored
        // TODO: DRY this part and the other from `show_double_column_body`
        let (socket_x, text_x) = match socket.side {
            NodeSide::Left => (0.0, SOCKET_WIDTH + SOCKET_NAME_GAP),
            NodeSide::Right => (
                rect.width() - SOCKET_WIDTH,
                rect.width() - (SOCKET_WIDTH + SOCKET_NAME_GAP),
            ),
        };

        let socket_center = pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);

        show_socket(ui, rendered_sockets, socket_center, text_pos, socket);

        pos.y += ROW_HEIGHT;
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
    let mut left = rect.min;
    let mut right = rect.min;

    for socket in sockets {
        let (pos, socket_x, text_x) = match socket.side {
            NodeSide::Left => (&mut left, 0.0, SOCKET_WIDTH + SOCKET_NAME_GAP),
            NodeSide::Right => (
                &mut right,
                rect.width() - SOCKET_WIDTH,
                rect.width() - (SOCKET_WIDTH + SOCKET_NAME_GAP),
            ),
        };

        let socket_center = *pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = *pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);

        show_socket(ui, rendered_sockets, socket_center, text_pos, socket);

        pos.y += ROW_HEIGHT;
    }
}

/// Render a socket.
fn show_socket<S>(
    ui: &mut egui::Ui,
    rendered_sockets: &mut Collector<RenderedSocket<S>>,
    socket_center: Pos2,
    text_pos: Pos2,
    socket: PreparedSocket<S>,
) where
    S: core::hash::Hash,
{
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

    ui.painter()
        .add(egui::Shape::galley(text_pos, text, Color32::WHITE));
}
