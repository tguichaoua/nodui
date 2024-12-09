use core::f32;

use crate::graph;

pub fn node_side(value: &mut nodui::NodeSide) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        let (text, next) = match *value {
            nodui::NodeSide::Left => (egui::RichText::new("Left"), nodui::NodeSide::Right),
            nodui::NodeSide::Right => (egui::RichText::new("Right"), nodui::NodeSide::Left),
        };

        let btn = ui.add(egui::Button::new(text));

        if btn.clicked() {
            *value = next;
        }

        btn
    }
}

pub fn node_layout(value: &mut nodui::NodeLayout) -> impl egui::Widget + '_ {
    |ui: &mut egui::Ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(value, nodui::NodeLayout::Single, "Single");
            ui.selectable_value(value, nodui::NodeLayout::Double, "Double");
        })
        .response
    }
}

pub fn socket_shape(
    id_salt: impl std::hash::Hash,
    value: &mut nodui::SocketShape,
) -> impl egui::Widget + '_ {
    let combo_box = egui::ComboBox::from_id_salt(id_salt).selected_text(format!("{value:?}"));
    |ui: &mut egui::Ui| {
        combo_box
            .show_ui(ui, move |ui| {
                ui.selectable_value(value, nodui::SocketShape::Circle, "Circle");
                ui.selectable_value(value, nodui::SocketShape::Square, "Square");
                ui.selectable_value(value, nodui::SocketShape::Triangle, "Triangle");
            })
            .response
    }
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
            title_color,
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
                            ui.color_edit_button_srgba(title_color);
                            ui.add(egui::TextEdit::singleline(title).desired_width(f32::INFINITY));
                        });
                        ui.end_row();

                        ui.label("Background");
                        ui.color_edit_button_srgba(background);
                        ui.end_row();
                    });
            });
        })
        .response
    }
}

/* -------------------------------------------------------------------------- */