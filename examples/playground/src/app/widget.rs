use nodui::ui::{Color, NodeLayout, NodeSide, TextUi};

/* -------------------------------------------------------------------------- */

pub struct NodeSideButton<'a> {
    value: &'a mut NodeSide,
}

impl NodeSideButton<'_> {
    pub fn new(value: &mut NodeSide) -> NodeSideButton<'_> {
        NodeSideButton { value }
    }
}

impl egui::Widget for NodeSideButton<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (text, next) = match self.value {
            NodeSide::Left => (egui::RichText::new("Left"), NodeSide::Right),
            NodeSide::Right => (egui::RichText::new("Right"), NodeSide::Left),
        };

        let btn = ui.add(egui::Button::new(text));

        if btn.clicked() {
            *self.value = next;
        }

        btn
    }
}

/* -------------------------------------------------------------------------- */

pub fn node_layout_options(ui: &mut egui::Ui, value: &mut NodeLayout) {
    ui.horizontal(|ui| {
        if ui
            .selectable_label(*value == NodeLayout::Single, "Single")
            .clicked()
        {
            *value = NodeLayout::Single;
        }

        if ui
            .selectable_label(*value == NodeLayout::Double, "Double")
            .clicked()
        {
            *value = NodeLayout::Double;
        }
    });
}

/* -------------------------------------------------------------------------- */

pub fn socket_shape_combo_box(
    id_salt: impl std::hash::Hash,
    ui: &mut egui::Ui,
    value: &mut nodui::ui::SocketShape,
) {
    egui::ComboBox::from_id_salt(id_salt)
        .selected_text(format!("{value:?}",))
        .show_ui(ui, |ui| {
            ui.selectable_value(value, nodui::ui::SocketShape::Circle, "Circle");
            ui.selectable_value(value, nodui::ui::SocketShape::Square, "Square");
            ui.selectable_value(value, nodui::ui::SocketShape::Triangle, "Triangle");
        });
}

/* -------------------------------------------------------------------------- */

pub fn text_ui(ui: &mut egui::Ui, text_ui: &mut TextUi) {
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut text_ui.text).desired_width(70.0));
        ui.color_edit_button_srgba_unmultiplied(
            text_ui.color.get_or_insert(Color::WHITE).as_array_mut(),
        );
    });
}
/* -------------------------------------------------------------------------- */

pub fn stroke(ui: &mut egui::Ui, stroke: &mut egui::Stroke) {
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba(&mut stroke.color);
        ui.add(egui::DragValue::new(&mut stroke.width));
    });
}

pub fn nodui_stroke(ui: &mut egui::Ui, stroke: &mut nodui::ui::Stroke) {
    ui.horizontal(|ui| {
        ui.color_edit_button_srgba_unmultiplied(stroke.color.as_array_mut());
        ui.add(egui::DragValue::new(&mut stroke.width));
    });
}

/* -------------------------------------------------------------------------- */
