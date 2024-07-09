//! Conversion from a [`nodui_core`] type to a [`egui`] type.

/// Conversion from a [`nodui_core`] type to a [`egui`] type.
pub(crate) trait IntoEgui {
    /// The corresponding [`egui`] type.
    type EguiTy;

    /// Convert this value to its [`egui`] equivalent.
    fn into_egui(self) -> Self::EguiTy;
}

impl<T: IntoEgui> IntoEgui for Option<T> {
    type EguiTy = Option<T::EguiTy>;

    fn into_egui(self) -> Self::EguiTy {
        self.map(IntoEgui::into_egui)
    }
}

impl IntoEgui for nodui_core::ui::Padding {
    type EguiTy = egui::Margin;

    fn into_egui(self) -> Self::EguiTy {
        let nodui_core::ui::Padding {
            top,
            left,
            right,
            bottom,
        } = self;
        egui::Margin {
            left,
            right,
            top,
            bottom,
        }
    }
}

impl IntoEgui for nodui_core::ui::Color {
    type EguiTy = egui::Color32;

    fn into_egui(self) -> Self::EguiTy {
        let (r, g, b, a) = self.rgba();
        egui::Color32::from_rgba_unmultiplied(r, g, b, a)
    }
}

impl IntoEgui for nodui_core::ui::Stroke {
    type EguiTy = egui::Stroke;

    fn into_egui(self) -> Self::EguiTy {
        let Self { width, color } = self;
        let color = color.into_egui();
        egui::Stroke { width, color }
    }
}
