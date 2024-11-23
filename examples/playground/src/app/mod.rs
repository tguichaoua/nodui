mod adapter;
mod widget;

use adapter::GraphAdapter;
use egui::Modifiers;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct App {
    graph: GraphAdapter,

    editor_bg_color: egui::Color32,
    editor_grid_stroke: egui::Stroke,
}

impl Default for App {
    #[inline]
    fn default() -> Self {
        Self {
            graph: GraphAdapter::default(),
            editor_bg_color: egui::Color32::BLACK,
            editor_grid_stroke: egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
        }
    }
}

impl App {
    /// Creates the [`App`].
    #[must_use]
    #[inline]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    #[inline]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("nodui_playground_inspector").show(ctx, |ui| {
            self.show_inspector(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_graph(ui);
        });

        ctx.input_mut(|input| {
            if input.consume_key(Modifiers::NONE, egui::Key::Delete) {
                self.graph.delete_selected_node();
            }
        });
    }
}

impl App {
    fn show_inspector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Editor Settings");
        egui::Grid::new(ui.id().with("graph settings grid"))
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Background Color");
                ui.color_edit_button_srgba(&mut self.editor_bg_color);
                ui.end_row();

                ui.label("Grid");
                widget::stroke(ui, &mut self.editor_grid_stroke);
                ui.end_row();
            });

        ui.separator();

        let mut socket_to_remove = None;
        let mut selected_node = self.graph.selected_node();

        egui::Grid::new(ui.id().with("node inspector grid"))
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("ID");
                if let Some(node) = selected_node.as_ref() {
                    ui.add(egui::Label::new(node.id().to_string()).truncate());
                }
                ui.end_row();

                ui.label("Title");
                if let Some(node) = selected_node.as_mut() {
                    ui.text_edit_singleline(&mut node.style.header.title.text);
                }
                ui.end_row();

                ui.label("Header Color");
                if let Some(node) = selected_node.as_mut() {
                    ui.color_edit_button_srgba_unmultiplied(
                        node.style.header.background.as_array_mut(),
                    );
                }
                ui.end_row();

                ui.label("Layout");
                if let Some(node) = selected_node.as_mut() {
                    widget::node_layout_options(ui, &mut node.style.body.layout);
                }
                ui.end_row();

                ui.label("Outline");
                if let Some(node) = selected_node.as_mut() {
                    widget::nodui_stroke(ui, &mut node.style.outline);
                }
                ui.end_row();
            });

        ui.separator();

        if ui.button("➕").clicked() {
            if let Some(node) = selected_node.as_mut() {
                node.add_socket();
            }
        }

        egui::Grid::new(ui.id().with("socket node inspector grid"))
            .num_columns(6)
            .show(ui, |ui| {
                ui.label(""); // TODO: it is possible to skip a column?
                ui.label("Id");
                ui.label("Name");
                ui.label("Color");
                ui.label("Side");
                ui.label("Shape");
                ui.end_row();

                if let Some(node) = selected_node {
                    for socket in node.sockets_mut() {
                        if ui.button("🗙").clicked() {
                            socket_to_remove = Some(socket.id());
                        }

                        ui.add(egui::Label::new(socket.id().to_string()).truncate());

                        widget::text_ui(ui, &mut socket.style.name);

                        ui.color_edit_button_srgba_unmultiplied(socket.style.color.as_array_mut());

                        ui.add(widget::NodeSideButton::new(&mut socket.style.side));

                        widget::socket_shape_combo_box(
                            ui.id().with(socket.id()),
                            ui,
                            &mut socket.style.shape,
                        );

                        ui.end_row();
                    }
                }
            });

        if let Some(socket_id) = socket_to_remove {
            self.graph.remove_socket(socket_id);
        }
    }

    fn show_graph(&mut self, ui: &mut egui::Ui) {
        let graph = nodui::GraphEditor::new(&mut self.graph, "graph")
            .background_color(self.editor_bg_color)
            .grid_stroke(self.editor_grid_stroke)
            .context_menu(|ui, context| {
                if ui.button("New node").clicked() {
                    context.graph.new_node(context.pos);
                    ui.close_menu();
                }

                ui.add_enabled_ui(context.graph.clipboard.is_some(), |ui| {
                    if ui.button("Paste node").clicked() {
                        context.graph.paste_node(context.pos);
                        ui.close_menu();
                    }
                });
            })
            .node_context_menu(|ui, context| {
                if ui.button("Add socket").clicked() {
                    if let Some(node) = context.graph.get_node_mut(context.node_id) {
                        node.add_socket();
                    }
                    ui.close_menu();
                }

                if ui.button("Copy").clicked() {
                    context.graph.copy_node(context.node_id);
                    ui.close_menu();
                }

                ui.add_enabled_ui(context.graph.clipboard.is_some(), |ui| {
                    if ui.button("Paste style").clicked() {
                        context.graph.paste_node_settings_to(context.node_id);
                        ui.close_menu();
                    }

                    if ui.button("Paste sockets").clicked() {
                        context.graph.paste_sockets_to(context.node_id);
                        ui.close_menu();
                    }
                });

                if ui.button("Remove").clicked() {
                    context.graph.remove_node(context.node_id);
                    ui.close_menu();
                }
            });

        let response = graph.show(ui);

        if let Some(selected_node_id) = response.last_interacted_node_id {
            self.graph.selected_node = Some(selected_node_id);
        }
    }
}
