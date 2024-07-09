//! Data for the visual editor viewport.

use std::ops::{Add, AddAssign, Sub, SubAssign};

use egui::Vec2;

use nodui_core::adapter::Pos;

/// The information about the viewport of the visual editor.
pub(crate) struct Viewport {
    /// The offset of the viewport.
    pub(crate) position: Vec2,
    /// The grid of the visual editor.
    pub(crate) grid: Grid,
}

/// The grid of the visual editor.
///
/// Used for coordinates convertion between the graph coordinate system ([`Pos`]) and
/// the ui coordinate system ([`CanvasPos`]).
#[derive(Clone)]
pub(crate) struct Grid {
    /// The size of the grid in pixel per unit.
    pub(crate) size: f32,
}

impl Viewport {
    /// Converts a UI position into a [`CanvasPos`].
    pub(crate) fn viewport_to_canvas(&self, pos: egui::Pos2) -> CanvasPos {
        CanvasPos(pos - self.position)
    }

    /// Converts a [`CanvasPos`] into a UI position.
    pub(crate) fn canvas_to_viewport(&self, pos: CanvasPos) -> egui::Pos2 {
        let CanvasPos(pos) = pos;
        pos + self.position
    }

    /// Converts a UI position into a graph position.
    pub(crate) fn viewport_to_graph(&self, pos: egui::Pos2) -> Pos {
        self.grid.canvas_to_graph(self.viewport_to_canvas(pos))
    }
}

impl Grid {
    /// Converts a graph position into a canvas position.
    pub(crate) fn graph_to_canvas(&self, pos: Pos) -> CanvasPos {
        let Pos { x, y } = pos;

        #[allow(clippy::cast_precision_loss)]
        let x = x as f32 * self.size;
        #[allow(clippy::cast_precision_loss)]
        let y = -y as f32 * self.size;

        CanvasPos(egui::pos2(x, y))
    }

    /// Converts a canvas position to a graph position without rounding the coordinates.
    fn canvas_to_graph_unrounded(&self, pos: CanvasPos) -> (f32, f32) {
        let CanvasPos(egui::Pos2 { x, y }) = pos;

        // egui `y` is top-to-bottom, while `Pos` is bottom-to-top.
        let y = -y;

        // Convert values
        let x = x / self.size;
        let y = y / self.size;

        (x, y)
    }

    /// Converts a canvas position to a graph position by
    /// rounding value to the nearest top-left graph position.
    pub(crate) fn canvas_to_graph(&self, pos: CanvasPos) -> Pos {
        let (x, y) = self.canvas_to_graph_unrounded(pos);

        #[allow(clippy::cast_possible_truncation)]
        // Round `x` toward -Inf (aka the left)
        let x = x.floor() as i32;

        #[allow(clippy::cast_possible_truncation)]
        // Round `y` toward +Inf (aka the top)
        let y = y.ceil() as i32;

        Pos { x, y }
    }

    /// Converts a canvas position to a graph position by
    /// rounding value to the nearest graph position.
    pub(crate) fn canvas_to_graph_nearest(&self, pos: CanvasPos) -> Pos {
        let (x, y) = self.canvas_to_graph_unrounded(pos);

        #[allow(clippy::cast_possible_truncation)]
        let x = x.round() as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = y.round() as i32;

        Pos { x, y }
    }
}

/* -------------------------------------------------------------------------- */

/// An opaque intermediate value for conversion between graph coordinates
/// and UI coordinates.
#[derive(Debug, Clone, Copy)]
pub(crate) struct CanvasPos(egui::Pos2);

impl CanvasPos {
    /// The zero for [`CanvasPos`].
    pub(crate) const ZERO: CanvasPos = CanvasPos(egui::Pos2::ZERO);

    /// Converts a [`CanvasPos`] to a [`Vec2`].
    pub(crate) fn to_vec2(self) -> Vec2 {
        self.0.to_vec2()
    }
}

impl Add<Vec2> for CanvasPos {
    type Output = CanvasPos;

    fn add(self, rhs: Vec2) -> Self::Output {
        let CanvasPos(pos) = self;
        CanvasPos(pos + rhs)
    }
}

impl AddAssign<Vec2> for CanvasPos {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl Sub<Vec2> for CanvasPos {
    type Output = CanvasPos;

    fn sub(self, rhs: Vec2) -> Self::Output {
        let CanvasPos(pos) = self;
        CanvasPos(pos - rhs)
    }
}

impl SubAssign<Vec2> for CanvasPos {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

/* -------------------------------------------------------------------------- */
