//! Rendering for nodes.

use std::sync::Arc;

use egui::epaint::{RectShape, TextShape};
use egui::text::{Fonts, LayoutJob};
use egui::{
    vec2, Align, Color32, CursorIcon, FontId, Galley, Margin, Pos2, Rect, Response, Rounding,
    Sense, Ui, Vec2,
};
use nodui_core::adapter::SocketAdapter;
use nodui_core::ui::{
    NodeBody, NodeHeader, NodeLayout, NodeSide, NodeUI, SocketShape, SocketUI, TitleHeader,
};

use crate::conversion::IntoEgui;
use crate::editor::{NodePainter, SocketResponses};
use crate::socket;

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

/* -------------------------------------------------------------------------- */

/// Show a node in the [`Ui`].
pub(super) fn show_node<NodeId, SocketId>(
    ui: &mut Ui,
    id: NodeId,
    node_ui: NodeUI,
    socket_responses: &mut SocketResponses<SocketId>,
    sockets: impl Iterator<Item: SocketAdapter<SocketId = SocketId>>,
    pos: Pos2,
    painter: &mut NodePainter,
) -> Response
where
    NodeId: nodui_core::Id,
    SocketId: nodui_core::Id,
{
    let (header, body) = ui.fonts(|fonts| {
        let header = prepare_header(fonts, node_ui.header);
        let sockets = prepare_body(fonts, node_ui.body, sockets);

        (header, sockets)
    });

    let node_size = {
        let width = f32::max(header.size.x, body.size.x);
        let height = header.size.y + body.size.y;
        vec2(width, height)
    };

    let response = ui.interact(
        Rect::from_min_size(pos, node_size),
        ui.id().with(id),
        Sense::click_and_drag(),
    );

    {
        let mut pos = pos;

        let (header_rounding, body_rounding) = split_rounding(
            NODE_ROUNDING,
            !matches!(header.content, HeaderContent::None),
        );

        match header.content {
            HeaderContent::None => {}
            HeaderContent::Title(TitleHeaderContent {
                title,
                padding,
                background,
            }) => {
                let rect = Rect::from_min_size(pos, vec2(node_size.x, header.size.y));
                painter.add(RectShape::filled(rect, header_rounding, background));

                painter.add(TextShape::new(
                    pos + padding.left_top(),
                    title,
                    Color32::WHITE,
                ));

                pos.y += header.size.y;
            }
        }

        {
            let rect = Rect::from_min_size(pos, vec2(node_size.x, body.size.y));
            painter.add(RectShape::filled(
                rect,
                body_rounding,
                body.background_color,
            ));

            {
                let rect = Rect::from_min_size(pos, vec2(node_size.x, body.size.y));
                let rect = rect - body.padding;

                match body.layout {
                    NodeLayout::Single => {
                        show_single_column_body(ui, painter, socket_responses, body, rect);
                    }
                    NodeLayout::Double => {
                        show_double_column_body(ui, painter, socket_responses, body, rect);
                    }
                }
            }

            pos.y += rect.height();
        }
    }

    // Add a stroke around the node to make it easier to see.
    painter.add(RectShape::stroke(
        Rect::from_min_size(pos, node_size),
        NODE_ROUNDING,
        node_ui.outline.into_egui(),
    ));

    response
}

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

/// The prepared data to render the header of the node.
struct PreparedHeader {
    /// The prepared header content.
    content: HeaderContent,
    /// The size required to render the header.
    size: Vec2,
}

/// The prepared content of the node header.
enum HeaderContent {
    /// No header.
    None,
    /// A [`TitleHeader`].
    Title(TitleHeaderContent),
}

/// The prepared content for a [`TitleHeader`].
struct TitleHeaderContent {
    /// The title text of the header.
    title: Arc<Galley>,
    /// The padding of the header.
    padding: Margin,
    /// The background color of the header.
    background: Color32,
}

/// Prepare the header for its rendering.
fn prepare_header(fonts: &Fonts, header: NodeHeader) -> PreparedHeader {
    match header {
        NodeHeader::None => PreparedHeader {
            content: HeaderContent::None,
            size: Vec2::ZERO,
        },
        NodeHeader::Title(TitleHeader { title, background }) => {
            // TODO: allow user to customize this value ?
            let padding = Margin::same(5.0);

            let background = background.into_egui();

            let title = fonts.layout_job(LayoutJob {
                halign: Align::LEFT,
                ..LayoutJob::simple_singleline(
                    title.text,
                    FontId::monospace(12.0),
                    title.color.into_egui().unwrap_or(Color32::WHITE),
                )
            });

            let size = padding.sum() + title.rect.size();

            PreparedHeader {
                content: HeaderContent::Title(TitleHeaderContent {
                    title,
                    padding,
                    background,
                }),
                size,
            }
        }
    }
}

/* -------------------------------------------------------------------------- */

/// The prepared data for a node body.
struct PreparedBody<Id> {
    /// The sockets to render to the body.
    sockets: Vec<PreparedSocket<Id>>,
    /// The size required to render the body.
    size: Vec2,

    /// The layout to use to render the sockets.
    layout: NodeLayout,
    /// The color of the background of the node.
    background_color: Color32,
    /// The space between the socket's handle and the socket's name.
    socket_text_gap: f32,

    /// The padding around the body.
    padding: Margin,
}

/// The prepared data for a socket.
struct PreparedSocket<Id> {
    /// The unique identifier of the socket.
    id: Id,
    /// The side on which the socket is rendered.
    side: NodeSide,
    /// The text name of the socket.
    text: Arc<Galley>,
    /// Whether or not this socket has at least one connection.
    is_connected: bool,
    /// The color of the socket's handle.
    color: Color32,
    /// The shape of the socket's handle.
    shape: SocketShape,
}

/// Prepare the node body for its rendering.
#[allow(clippy::needless_pass_by_value)] // false positive ? // TODO: investigate
fn prepare_body<I>(
    fonts: &Fonts,
    body: NodeBody,
    sockets: I,
) -> PreparedBody<<I::Item as SocketAdapter>::SocketId>
where
    I: Iterator<Item: SocketAdapter>,
{
    let NodeBody {
        layout,
        background_color,
        padding,
        socket_text_gap,
        column_gap,
    } = body;

    let background_color = background_color.into_egui();
    let padding = padding.into_egui();

    let sockets: Vec<_> = sockets
        .map(|socket| {
            let id = socket.id();
            let SocketUI {
                name: text,
                side,
                is_connected,
                color,
                shape,
            } = socket.ui();

            let color = color.into_egui();

            let text = fonts.layout_job(LayoutJob {
                halign: match side {
                    NodeSide::Left => Align::LEFT,
                    NodeSide::Right => Align::RIGHT,
                },
                ..LayoutJob::simple_singleline(
                    text.text,
                    FontId::monospace(12.0),
                    text.color.into_egui().unwrap_or(DEFAULT_TEXT_COLOR),
                )
            });

            PreparedSocket {
                id,
                side,
                text,
                is_connected,
                color,
                shape,
            }
        })
        .collect();

    let text_size = match layout {
        NodeLayout::Single => {
            let width = sockets
                .iter()
                .map(|s| s.text.rect.width())
                .max_by(f32::total_cmp)
                .unwrap_or(0.0);

            #[allow(clippy::cast_precision_loss)]
            let height = ROW_HEIGHT * sockets.len() as f32;

            vec2(width, height)
        }
        NodeLayout::Double => {
            let mut left = Vec2::ZERO;
            let mut right = Vec2::ZERO;

            for s in &sockets {
                let size = match s.side {
                    NodeSide::Left => &mut left,
                    NodeSide::Right => &mut right,
                };

                size.x = size.x.max(s.text.rect.width());
                size.y += ROW_HEIGHT;
            }

            let width = left.x + column_gap + right.x;
            let height = f32::max(left.y, right.y);

            vec2(width, height)
        }
    };

    let size = {
        let width = padding.left
            + SOCKET_WIDTH
            + socket_text_gap
            + text_size.x
            + socket_text_gap
            + SOCKET_WIDTH
            + padding.right;

        let height = padding.top + text_size.y + padding.bottom;

        vec2(width, height)
    };

    PreparedBody {
        sockets,
        size,

        layout,
        background_color,
        socket_text_gap,

        padding,
    }
}

/* -------------------------------------------------------------------------- */

/// Render the node body with a single column layout.
fn show_single_column_body<SocketId>(
    ui: &mut Ui,
    painter: &mut NodePainter,
    socket_responses: &mut SocketResponses<SocketId>,
    body: PreparedBody<SocketId>,
    rect: Rect,
) where
    SocketId: nodui_core::Id,
{
    let PreparedBody {
        sockets,
        socket_text_gap,
        ..
    } = body;

    let mut pos = rect.min;

    for socket in sockets {
        let (socket_x, text_x) = match socket.side {
            NodeSide::Left => (0.0, SOCKET_WIDTH + socket_text_gap),
            NodeSide::Right => (
                rect.width() - SOCKET_WIDTH,
                rect.width() - SOCKET_WIDTH - socket_text_gap,
            ),
        };

        let socket_center = pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);

        show_socket(
            ui,
            painter,
            socket_responses,
            socket_center,
            text_pos,
            socket,
        );

        pos.y += ROW_HEIGHT;
    }
}

/// Render the node body with a double columns layout.
fn show_double_column_body<SocketId>(
    ui: &mut Ui,
    painter: &mut NodePainter,
    socket_responses: &mut SocketResponses<SocketId>,
    body: PreparedBody<SocketId>,
    rect: Rect,
) where
    SocketId: nodui_core::Id,
{
    let PreparedBody {
        sockets,
        socket_text_gap,
        ..
    } = body;

    let mut left = rect.min;
    let mut right = rect.min;

    for socket in sockets {
        let (pos, socket_x, text_x) = match socket.side {
            NodeSide::Left => (&mut left, 0.0, SOCKET_WIDTH + socket_text_gap),
            NodeSide::Right => (
                &mut right,
                rect.width() - SOCKET_WIDTH,
                rect.width() - SOCKET_WIDTH - socket_text_gap,
            ),
        };

        let socket_center = *pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = *pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);

        show_socket(
            ui,
            painter,
            socket_responses,
            socket_center,
            text_pos,
            socket,
        );

        pos.y += ROW_HEIGHT;
    }
}

/// Render a socket.
fn show_socket<SocketId>(
    ui: &mut Ui,
    painter: &mut NodePainter,
    socket_responses: &mut SocketResponses<SocketId>,
    socket_center: Pos2,
    text_pos: Pos2,
    socket: PreparedSocket<SocketId>,
) where
    SocketId: nodui_core::Id,
{
    let PreparedSocket {
        id,
        side,
        text,
        is_connected,
        color,
        shape,
    } = socket;

    {
        let rect = Rect::from_center_size(socket_center, Vec2::splat(SOCKET_WIDTH));
        let response = ui.interact(rect, ui.id().with(&id), Sense::click_and_drag());
        let response = response.on_hover_cursor(CursorIcon::PointingHand);
        socket_responses.insert(id, response, color, side);
    }

    painter.add(socket::make_shape(
        shape,
        socket_center,
        SOCKET_WIDTH,
        color,
        is_connected,
    ));

    painter.add(egui::Shape::galley(text_pos, text, Color32::WHITE));
}

/* -------------------------------------------------------------------------- */
