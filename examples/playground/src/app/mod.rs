mod adapter;

use adapter::GraphAdapter;
use egui::DragValue;
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

                ui.label("Grid Color");
                ui.color_edit_button_srgba(&mut self.editor_grid_stroke.color);
                ui.end_row();

                ui.label("Grid Width");
                ui.add(DragValue::new(&mut self.editor_grid_stroke.width));
                ui.end_row();
            });

        ui.separator();

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
                    ui.text_edit_singleline(&mut node.header.title.text);
                }
                ui.end_row();

                ui.label("Header Color");
                if let Some(node) = selected_node.as_mut() {
                    ui.color_edit_button_srgba_unmultiplied(node.header.background.as_array_mut());
                }
                ui.end_row();

                ui.label("Layout");
                if let Some(node) = selected_node.as_mut() {
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                node.body.layout == nodui::ui::NodeLayout::Single,
                                "Single",
                            )
                            .clicked()
                        {
                            node.body.layout = nodui::ui::NodeLayout::Single;
                        }
                        if ui
                            .selectable_label(
                                node.body.layout == nodui::ui::NodeLayout::Double,
                                "Double",
                            )
                            .clicked()
                        {
                            node.body.layout = nodui::ui::NodeLayout::Double;
                        }
                    });
                }
                ui.end_row();
            });

        ui.label("Sockets");

        egui::Grid::new(ui.id().with("socket node inspector grid"))
            .num_columns(6)
            .show(ui, |ui| {
                ui.label("Id");
                ui.label("Name");
                ui.label("Name Color");
                ui.label("Color");
                ui.label("Left");
                ui.label("Right");
                ui.end_row();

                if let Some(node) = selected_node {
                    for socket in node.sockets_mut() {
                        ui.add(egui::Label::new(socket.id().to_string()).truncate());
                        ui.text_edit_singleline(&mut socket.name.text);
                        ui.color_edit_button_srgba_unmultiplied(
                            socket
                                .name
                                .color
                                .get_or_insert(nodui::ui::Color::WHITE)
                                .as_array_mut(),
                        );

                        ui.color_edit_button_srgba_unmultiplied(socket.color.as_array_mut());

                        if ui
                            .selectable_label(socket.side == nodui::ui::NodeSide::Left, "Left")
                            .clicked()
                        {
                            socket.side = nodui::ui::NodeSide::Left;
                        }
                        if ui
                            .selectable_label(socket.side == nodui::ui::NodeSide::Right, "Right")
                            .clicked()
                        {
                            socket.side = nodui::ui::NodeSide::Right;
                        }

                        ui.end_row();
                    }
                }
            });
    }

    fn show_graph(&mut self, ui: &mut egui::Ui) {
        let graph = nodui::GraphEditor::new(&mut self.graph, "graph")
            .background_color(self.editor_bg_color)
            .grid_stroke(self.editor_grid_stroke)
            .context_menu(|ui, context| {
                if ui.button("new node").clicked() {
                    context.graph.add_node(context.pos);
                    ui.close_menu();
                }
            })
            // .socket_context_menu(|ui, context| {
            //     if ui.button("Disconnect").clicked() {
            //         context
            //             .graph
            //             .graph
            //             .connections_mut()
            //             .disconnect(context.socket_id);
            //         ui.close_menu();
            //     }
            // })
            .node_context_menu(|ui, context| {
                if ui.button("add socket").clicked() {
                    if let Some(node) = context.graph.get_node_mut(context.node_id) {
                        node.add_socket();
                    }

                    ui.close_menu();
                }

                // if ui.button("Select").clicked() {
                //     context.graph.set_selected_node(context.node_id);
                //     ui.close_menu();
                // }

                // ui.separator();

                // if ui.button("Remove").clicked() {
                //     context.graph.remove_node(context.node_id);
                //     ui.close_menu();
                // }
            });

        let response = graph.show(ui);

        if let Some(selected_node_id) = response.last_interacted_node_id {
            self.graph.selected_node = Some(selected_node_id);
        }
    }
}
