//! Preparation and rendering of node's body part.

use std::{
    iter::Sum,
    ops::{Add, AddAssign},
    sync::Arc,
};

use egui::{
    epaint::RectShape,
    text::{Fonts, LayoutJob},
    vec2, Align, Checkbox, Color32, CursorIcon, DragValue, FontId, Galley, Margin, Pos2, Rect,
    Rounding, Sense, Ui, Vec2,
};
use nodui_core::{
    ui::{NodeBody, NodeLayout, NodeSide, SocketShape, SocketUI},
    NodeAdapter, SizeHint, SocketData, SocketField, SocketSeq,
};

use crate::{editor::SocketResponses, socket};

use super::{
    IntoEgui, DEFAULT_TEXT_COLOR, ROW_HEIGHT, SOCKET_FIELD_SIZE, SOCKET_NAME_FIELD_GAP,
    SOCKET_NAME_GAP, SOCKET_WIDTH,
};

/* -------------------------------------------------------------------------- */

/// The prepared data for a node body.
pub(super) struct PreparedBody<'field, SocketId> {
    /// The sockets to render to the body.
    sockets: Vec<PreparedSocket<'field, SocketId>>,
    /// The size required to render the body.
    size: Vec2,

    /// The layout to use to render the sockets.
    layout: NodeLayout,
    /// The color of the background of the node.
    background_color: Color32,

    /// The padding around the body.
    padding: Margin,
}

impl<S> PreparedBody<'_, S> {
    /// The total size occupied by the body.
    pub(super) fn size(&self) -> Vec2 {
        self.size
    }
}

/// The prepared data for a socket.
struct PreparedSocket<'field, SocketId> {
    /// The unique identifier of the socket.
    id: SocketId,
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

    /// The socket field.
    field: Option<SocketField<'field>>,
}

impl<S> PreparedSocket<'_, S> {
    /// Compute the size the socket will occupied.
    fn compute_size(&self) -> Vec2 {
        let socket_size = Vec2::splat(SOCKET_WIDTH);
        let socket_text_gap = vec2(SOCKET_NAME_GAP, 0.0);
        let text_size = self.text.size();
        let text_field_gap = vec2(SOCKET_NAME_FIELD_GAP, 0.0);
        let field_size = if self.field.is_some() {
            SOCKET_FIELD_SIZE
        } else {
            Vec2::ZERO
        };

        let mut size: Vec2 = [
            socket_size,
            socket_text_gap,
            text_size,
            text_field_gap,
            field_size,
        ]
        .into_iter()
        .map(HLayoutSize::from)
        .sum();

        // FIXME: it work will the computed height is lower than `ROW_HEIGHT`.
        size.y = ROW_HEIGHT;

        size
    }
}

/// Prepare the node body for its rendering.
pub(super) fn prepare<'node, Node>(
    fonts: &Fonts,
    body: &NodeBody,
    node: &'node mut Node,
) -> PreparedBody<'node, Node::SocketId>
where
    Node: NodeAdapter,
{
    let NodeBody {
        layout,
        background_color,
        padding,
        column_gap,
    } = body;

    let background_color = background_color.into_egui();
    let padding = padding.into_egui();

    let sockets = collect_sockets(node, fonts);

    let size: Vec2 = match layout {
        NodeLayout::Single => sockets
            .iter()
            .map(PreparedSocket::compute_size)
            .map(VLayoutSize::from)
            .sum(),
        NodeLayout::Double => {
            let mut left = VLayoutSize::from(Vec2::ZERO);
            let mut right = VLayoutSize::from(Vec2::ZERO);

            for s in &sockets {
                let mut size = match s.side {
                    NodeSide::Left => &mut left,
                    NodeSide::Right => &mut right,
                };

                size += VLayoutSize::from(s.compute_size());
            }

            let column_gap = vec2(0.0, *column_gap);

            [left.0, column_gap, right.0]
                .into_iter()
                .map(HLayoutSize::from)
                .sum()
        }
    };

    let size = size + padding.sum();

    PreparedBody {
        sockets,
        size,

        layout: *layout,
        background_color,

        padding,
    }
}

/// Prepares the socket to be rendered.
fn prepare_socket<'field, SocketId>(
    socket: SocketData<'field, SocketId>,
    fonts: &Fonts,
) -> PreparedSocket<'field, SocketId> {
    let SocketData {
        id,
        side,
        ui,
        field,
    } = socket;

    let SocketUI {
        name: text,
        is_connected,
        color,
        shape,
    } = ui;

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
        field,
    }
}

/* -------------------------------------------------------------------------- */

/// Collects the [`SocketData`] from the node.
fn collect_sockets<'node, Node>(
    node: &'node mut Node,
    fonts: &Fonts,
) -> Vec<PreparedSocket<'node, Node::SocketId>>
where
    Node: NodeAdapter,
{
    let mut sockets = Vec::new();

    node.accept(NodeVisitor {
        fonts,
        sockets: &mut sockets,
    });

    sockets
}

/// A node visitor to collect and prepare socket to be rendered.
struct NodeVisitor<'a, 'node, S> {
    /// The [`Fonts`] used to render texts.
    fonts: &'a Fonts,
    /// Where to store the prepared sockets.
    sockets: &'a mut Vec<PreparedSocket<'node, S>>,
}

impl<'node, S> nodui_core::NodeVisitor<'node, S> for NodeVisitor<'_, 'node, S> {
    fn sockets(&mut self, size_hint: SizeHint) -> impl SocketSeq<'node, S> {
        self.sockets.reserve(size_hint.min());

        NodeVisitor {
            fonts: self.fonts,
            sockets: self.sockets,
        }
    }
}

impl<'node, S> SocketSeq<'node, S> for NodeVisitor<'_, 'node, S> {
    #[inline]
    fn visit_socket(&mut self, socket: SocketData<'node, S>) {
        // self.sockets.push(prepare_socket(id, ui, self.fonts, field));
        self.sockets.push(prepare_socket(socket, self.fonts));
    }
}

/* -------------------------------------------------------------------------- */

impl<S> PreparedBody<'_, S>
where
    S: nodui_core::Id,
{
    /// Render the body.
    pub(super) fn show(
        self,
        ui: &mut Ui,
        pos: Pos2,
        node_size: Vec2,
        rounding: Rounding,
        socket_responses: &mut SocketResponses<S>,
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
                    show_single_column_body(ui, socket_responses, sockets, rect);
                }
                NodeLayout::Double => {
                    show_double_column_body(ui, socket_responses, sockets, rect);
                }
            }
        }
    }
}

/// Render the node body with a single column layout.
fn show_single_column_body<SocketId>(
    ui: &mut Ui,
    socket_responses: &mut SocketResponses<SocketId>,
    sockets: Vec<PreparedSocket<'_, SocketId>>,
    rect: Rect,
) where
    SocketId: nodui_core::Id,
{
    let mut pos = rect.min;

    for socket in sockets {
        // TODO: may be refactored
        // TODO: DRY this part and the other from `show_double_column_body`
        let (socket_x, text_x, field_x) = match socket.side {
            NodeSide::Left => (
                0.0,
                SOCKET_WIDTH + SOCKET_NAME_GAP,
                SOCKET_WIDTH + SOCKET_NAME_GAP + socket.text.size().x + SOCKET_NAME_FIELD_GAP,
            ),
            NodeSide::Right => (
                rect.width() - SOCKET_WIDTH,
                rect.width() - (SOCKET_WIDTH + SOCKET_NAME_GAP),
                rect.width()
                    - (SOCKET_WIDTH
                        + SOCKET_NAME_GAP
                        + socket.text.size().x
                        + SOCKET_FIELD_SIZE.x
                        + SOCKET_NAME_FIELD_GAP),
            ),
        };

        let socket_center = pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);
        let field_pos = pos + Vec2::new(field_x, (ROW_HEIGHT - SOCKET_FIELD_SIZE.y) / 2.0);

        show_socket(
            ui,
            socket_responses,
            socket_center,
            text_pos,
            field_pos,
            socket,
        );

        pos.y += ROW_HEIGHT;
    }
}

/// Render the node body with a double columns layout.
fn show_double_column_body<SocketId>(
    ui: &mut Ui,
    socket_responses: &mut SocketResponses<SocketId>,
    sockets: Vec<PreparedSocket<'_, SocketId>>,
    rect: Rect,
) where
    SocketId: nodui_core::Id,
{
    let mut left = rect.min;
    let mut right = rect.min;

    for socket in sockets {
        let (pos, socket_x, text_x, field_x) = match socket.side {
            NodeSide::Left => (
                &mut left,
                0.0,
                SOCKET_WIDTH + SOCKET_NAME_GAP,
                SOCKET_WIDTH + SOCKET_NAME_GAP + socket.text.size().x + SOCKET_NAME_FIELD_GAP,
            ),
            NodeSide::Right => (
                &mut right,
                rect.width() - SOCKET_WIDTH,
                rect.width() - (SOCKET_WIDTH + SOCKET_NAME_GAP),
                rect.width()
                    - (SOCKET_WIDTH
                        + SOCKET_NAME_GAP
                        + socket.text.size().x
                        + SOCKET_FIELD_SIZE.x
                        + SOCKET_NAME_FIELD_GAP),
            ),
        };

        let socket_center = *pos + Vec2::new(socket_x + SOCKET_WIDTH / 2.0, ROW_HEIGHT / 2.0);
        let text_pos = *pos + Vec2::new(text_x, (ROW_HEIGHT - socket.text.rect.height()) / 2.0);
        let field_pos = *pos + Vec2::new(field_x, (ROW_HEIGHT - SOCKET_FIELD_SIZE.y) / 2.0);

        show_socket(
            ui,
            socket_responses,
            socket_center,
            text_pos,
            field_pos,
            socket,
        );

        pos.y += ROW_HEIGHT;
    }
}

/// Render a socket.
fn show_socket<SocketId>(
    ui: &mut Ui,
    socket_responses: &mut SocketResponses<SocketId>,
    socket_center: Pos2,
    text_pos: Pos2,
    field_pos: Pos2,
    socket: PreparedSocket<'_, SocketId>,
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

        field,
    } = socket;

    {
        let rect = Rect::from_center_size(socket_center, Vec2::splat(SOCKET_WIDTH));
        let response = ui.interact(rect, ui.id().with(&id), Sense::click_and_drag());
        let response = response.on_hover_cursor(CursorIcon::PointingHand);
        socket_responses.insert(id, response, color, side);
    }

    ui.painter().add(socket::make_shape(
        shape,
        socket_center,
        SOCKET_WIDTH,
        color,
        is_connected,
    ));

    ui.painter()
        .add(egui::Shape::galley(text_pos, text, Color32::WHITE));

    if let Some(field) = field {
        let rect = Rect::from_min_size(field_pos, SOCKET_FIELD_SIZE);

        let _response = match field {
            SocketField::Bool(value) => ui.put(rect, Checkbox::without_text(value)),

            SocketField::F32(value) => ui.put(rect, DragValue::new(value)),
            SocketField::F64(value) => ui.put(rect, DragValue::new(value)),

            SocketField::I32(value) => ui.put(rect, DragValue::new(value)),
            SocketField::I8(value) => ui.put(rect, DragValue::new(value)),
            SocketField::I16(value) => ui.put(rect, DragValue::new(value)),
            SocketField::I64(value) => ui.put(rect, DragValue::new(value)),

            SocketField::U8(value) => ui.put(rect, DragValue::new(value)),
            SocketField::U16(value) => ui.put(rect, DragValue::new(value)),
            SocketField::U32(value) => ui.put(rect, DragValue::new(value)),
            SocketField::U64(value) => ui.put(rect, DragValue::new(value)),
        };
    }
}

/* -------------------------------------------------------------------------- */

/// Represent a size that stacks in a vertical layout way.
#[derive(Debug, Clone, Copy)]
struct VLayoutSize(Vec2);

impl From<Vec2> for VLayoutSize {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

impl From<VLayoutSize> for Vec2 {
    #[inline]
    fn from(value: VLayoutSize) -> Self {
        value.0
    }
}

impl Add for VLayoutSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0.x.max(rhs.0.x);
        let y = self.0.y + rhs.0.y;
        Self(vec2(x, y))
    }
}

impl AddAssign for VLayoutSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl AddAssign<VLayoutSize> for &mut VLayoutSize {
    #[inline]
    fn add_assign(&mut self, rhs: VLayoutSize) {
        **self = **self + rhs;
    }
}

impl Sum<VLayoutSize> for Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = VLayoutSize>>(iter: I) -> Self {
        let mut x = 0.0_f32;
        let mut y = 0.0_f32;
        iter.for_each(|VLayoutSize(size)| {
            x = x.max(size.x);
            y += size.y;
        });
        vec2(x, y)
    }
}

/* -------------------------------------------------------------------------- */

/// Represent a size that stacks in a horizontal layout way.
#[derive(Debug, Clone, Copy)]
struct HLayoutSize(Vec2);

impl From<Vec2> for HLayoutSize {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

impl From<HLayoutSize> for Vec2 {
    #[inline]
    fn from(value: HLayoutSize) -> Self {
        value.0
    }
}

impl Add for HLayoutSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.0.x + rhs.0.x;
        let y = self.0.y.max(rhs.0.y);
        Self(vec2(x, y))
    }
}

impl AddAssign for HLayoutSize {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sum<HLayoutSize> for Vec2 {
    #[inline]
    fn sum<I: Iterator<Item = HLayoutSize>>(iter: I) -> Self {
        let mut x = 0.0_f32;
        let mut y = 0.0_f32;
        iter.for_each(|HLayoutSize(size)| {
            x += size.x;
            y = y.max(size.y);
        });
        vec2(x, y)
    }
}

/* -------------------------------------------------------------------------- */
