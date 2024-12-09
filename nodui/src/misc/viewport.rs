//! Viewport sizing.

use egui::{vec2, NumExt, Vec2};

/// The preferences for the viewport size.
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct ViewportSize {
    /// The desired width of the viewport.
    pub(crate) width: Option<f32>,
    /// The desired height of the viewport.
    pub(crate) height: Option<f32>,
    /// The desired aspect ratio of the viewport.
    pub(crate) view_aspect: Option<f32>,
    /// The minimum size of the viewport.
    pub(crate) min_size: Vec2,
}

impl ViewportSize {
    /// `width / height` ratio of the editor region.
    ///
    /// By default no fixed aspect ratio is set (and width/height will fill the ui it is in).
    #[inline]
    #[must_use]
    pub(crate) fn view_aspect(mut self, view_aspect: f32) -> Self {
        self.view_aspect = Some(view_aspect);
        self
    }

    /// Width of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the width can be calculated from the height.
    #[inline]
    #[must_use]
    pub(crate) fn width(mut self, width: f32) -> Self {
        self.min_size.x = width;
        self.width = Some(width);
        self
    }

    /// Height of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the height can be calculated from the width.
    #[inline]
    #[must_use]
    pub(crate) fn height(mut self, height: f32) -> Self {
        self.min_size.y = height;
        self.height = Some(height);
        self
    }

    /// Computes the size the viewport may occupied.
    pub(crate) fn compute(self, ui: &egui::Ui) -> Vec2 {
        let Self {
            width,
            height,
            view_aspect,
            min_size,
        } = self;

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
    }
}
