use core::f32;

use crate::graph;

pub fn node_side(value: &mut nodui::ui::NodeSide) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        let (text, next) = match *value {
            nodui::ui::NodeSide::Left => (egui::RichText::new("Left"), nodui::ui::NodeSide::Right),
            nodui::ui::NodeSide::Right => (egui::RichText::new("Right"), nodui::ui::NodeSide::Left),
        };

        let btn = ui.add(egui::Button::new(text));

        if btn.clicked() {
            *value = next;
        }

        btn
    }
}

pub fn node_layout(value: &mut nodui::ui::NodeLayout) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(value, nodui::ui::NodeLayout::Single, "Single");
            ui.selectable_value(value, nodui::ui::NodeLayout::Double, "Double");
        })
        .response
    }
}

pub fn socket_shape(
    id_salt: impl std::hash::Hash,
    value: &mut nodui::ui::SocketShape,
) -> impl egui::Widget + '_ {
    let combo_box = egui::ComboBox::from_id_salt(id_salt).selected_text(format!("{value:?}"));
    |ui: &mut egui::Ui| {
        combo_box
            .show_ui(ui, move |ui| {
                ui.selectable_value(value, nodui::ui::SocketShape::Circle, "Circle");
                ui.selectable_value(value, nodui::ui::SocketShape::Square, "Square");
                ui.selectable_value(value, nodui::ui::SocketShape::Triangle, "Triangle");
            })
            .response
    }
}

pub fn text_ui(ui: &mut egui::Ui, text_ui: &mut nodui::ui::TextUi) {
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(&mut text_ui.text).desired_width(70.0));
        ui.color_edit_button_srgba_unmultiplied(
            text_ui
                .color
                .get_or_insert(nodui::ui::Color::WHITE)
                .as_array_mut(),
        );
    });
}

pub fn node_header_style(
    id_salt: impl std::hash::Hash,
    header_style: &mut graph::NodeHeaderStyle,
) -> impl egui::Widget + '_ {
    let id = egui::Id::new(id_salt);

    move |ui: &mut egui::Ui| {
        let graph::NodeHeaderStyle {
            mode,
            title,
            background,
        } = header_style;

        ui.vertical_centered_justified(|ui| {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                egui::Grid::new(id.with("grid"))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Mode");
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_salt(id.with("header mode"))
                                .selected_text(format!("{mode:?}"))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(mode, graph::HeaderMode::None, "None");
                                    ui.selectable_value(mode, graph::HeaderMode::Title, "Title");
                                });

                            ui.add_space(ui.available_width());
                        });
                        ui.end_row();

                        ui.label("Title");
                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba_unmultiplied(
                                title
                                    .color
                                    .get_or_insert(nodui::ui::Color::BLACK)
                                    .as_array_mut(),
                            );
                            ui.add(
                                egui::TextEdit::singleline(&mut title.text)
                                    .desired_width(f32::INFINITY),
                            );
                        });
                        ui.end_row();

                        ui.label("Background");
                        ui.color_edit_button_srgba_unmultiplied(background.as_array_mut());
                        ui.end_row();
                    });
            });
        })
        .response
    }
}

/* -------------------------------------------------------------------------- */

pub fn nodui_color(value: &mut nodui::ui::Color) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| ui.color_edit_button_srgba_unmultiplied(value.as_array_mut())
}

pub fn nodui_stroke(value: &mut nodui::ui::Stroke) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        let nodui::ui::Stroke { width, color } = *value;
        let (r, g, b, a) = color.rgba();

        let mut egui_value = egui::Stroke {
            width,
            color: egui::Color32::from_rgba_unmultiplied(r, g, b, a),
        };

        let response = ui.add(&mut egui_value);

        let egui::Stroke { width, color } = egui_value;
        let [r, g, b, a] = color.to_srgba_unmultiplied();

        *value = nodui::ui::Stroke {
            width,
            color: nodui::ui::Color::from_rgba(r, g, b, a),
        };

        response
    }
}

pub fn nodui_padding(value: &mut nodui::ui::Padding) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        let nodui::ui::Padding {
            top,
            left,
            right,
            bottom,
        } = *value;

        let mut egui_value = egui::Margin {
            top,
            left,
            right,
            bottom,
        };

        let response = ui.add(&mut egui_value);

        let egui::Margin {
            top,
            left,
            right,
            bottom,
        } = egui_value;

        *value = nodui::ui::Padding {
            top,
            left,
            right,
            bottom,
        };

        response
    }
}
