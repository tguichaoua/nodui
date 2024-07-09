//! Rendering for sockets.

use egui::epaint::{CircleShape, PathShape, RectShape};
use egui::{vec2, Color32, Pos2, Rect, Response, Rounding, Shape, Stroke, Vec2};

use nodui_core::ui::{NodeSide, SocketShape};

/* -------------------------------------------------------------------------- */

/// Data about a socket after being rendered.
pub struct RenderedSocket {
    /// The [`Response`] to interact with the socket.
    pub(crate) response: Response,
    /// The color of the socket.
    pub(crate) color: Color32,
    /// The side on which the socket is rendered.
    pub(crate) side: NodeSide,
}

impl RenderedSocket {
    /// The color of the socket.
    #[inline]
    #[must_use]
    pub fn color(&self) -> Color32 {
        self.color
    }

    /// The side on which the socket is rendered.
    #[inline]
    #[must_use]
    pub fn side(&self) -> NodeSide {
        self.side
    }

    /// The UI position in which the socket is rendered.
    #[inline]
    #[must_use]
    pub fn pos(&self) -> Pos2 {
        self.response.rect.center()
    }
}

/* -------------------------------------------------------------------------- */

/// Create a [`Shape`] for a socket.
pub(crate) fn make_shape(
    shape: SocketShape,
    center: Pos2,
    width: f32,
    color: Color32,
    is_connected: bool,
) -> Shape {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_3};

    let fill = if is_connected {
        color
    } else {
        Color32::default()
    };

    let stroke = Stroke::new(1.0, color);

    match shape {
        SocketShape::Circle => Shape::Circle(CircleShape {
            center,
            radius: width / 2.0,
            fill,
            stroke,
        }),

        SocketShape::Square => Shape::Rect(RectShape {
            rect: Rect::from_center_size(center, Vec2::splat(width * FRAC_1_SQRT_2)),
            fill,
            stroke,
            rounding: Rounding::default(),
            fill_texture_id: egui::TextureId::default(),
            uv: Rect::ZERO,
            blur_width: 0.0,
        }),

        SocketShape::Triangle => Shape::Path(PathShape {
            points: vec![
                center + (width / 2.0) * vec2(f32::cos(0.0), f32::sin(0.0)),
                center + (width / 2.0) * vec2(f32::cos(2.0 * FRAC_PI_3), f32::sin(2.0 * FRAC_PI_3)),
                center + (width / 2.0) * vec2(f32::cos(4.0 * FRAC_PI_3), f32::sin(4.0 * FRAC_PI_3)),
            ],

            closed: true,
            fill,
            stroke: stroke.into(),
        }),
    }
}

/* -------------------------------------------------------------------------- */
