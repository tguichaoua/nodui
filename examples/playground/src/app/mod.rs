mod adapter;
mod widget;

use adapter::GraphAdapter;
use nodui::Pos;
use serde::{Deserialize, Serialize};

use crate::graph::SocketId;

#[derive(Serialize, Deserialize)]
pub struct App {
    graph: GraphAdapter,

    editor_bg_color: egui::Color32,
    editor_grid_stroke: egui::Stroke,

    #[serde(skip)]
    editor_pos: Pos,
    #[serde(skip)]
    cursor_pos: Option<Pos>,
}

impl Default for App {
    #[inline]
    fn default() -> Self {
        Self {
            graph: GraphAdapter::default(),
            editor_bg_color: egui::Color32::BLACK,
            editor_grid_stroke: egui::Stroke::new(0.5, egui::Color32::DARK_GRAY),
            editor_pos: Pos::default(),
            cursor_pos: None,
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
            ui.vertical_centered_justified(|ui| {
                self.show_inspector(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_graph(ui);
        });

        ctx.input_mut(|input| {
            for ev in &input.events {
                #[allow(clippy::wildcard_enum_match_arm)]
                match ev {
                    egui::Event::Copy => {
                        if let Some(node_id) = self.graph.selected_node {
                            self.graph.copy_node(node_id);
                        }
                    }
                    egui::Event::Paste(_) => {
                        if let Some(pos) = self.cursor_pos {
                            self.graph.paste_node(pos);
                        }
                    }
                    egui::Event::Cut => {
                        if let Some(node_id) = self.graph.selected_node {
                            self.graph.copy_node(node_id);
                            self.graph.remove_node(node_id);
                        }
                    }
                    _ => {}
                }
            }

            if input.consume_key(egui::Modifiers::NONE, egui::Key::Delete) {
                self.graph.delete_selected_node();
            }
        });
    }
}

impl App {
    #[allow(clippy::too_many_lines)]
    fn show_inspector(&mut self, ui: &mut egui::Ui) {
        ui.heading("Editor Settings");
        egui::Grid::new(ui.id().with("graph settings grid"))
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Background Color");
                ui.color_edit_button_srgba(&mut self.editor_bg_color);
                ui.end_row();

                ui.label("Grid");
                ui.add(&mut self.editor_grid_stroke);
                ui.end_row();
            });

        ui.separator();

        let mut socket_action = SocketAction::None;
        if let Some(node) = self.graph.selected_node() {
            egui::Grid::new(ui.id().with("node inspector grid"))
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.add(egui::Label::new(node.id().to_string()).truncate());
                    ui.end_row();

                    ui.label("Header");
                    ui.add(widget::node_header_style(
                        ui.id().with("header"),
                        &mut node.style.header,
                    ));
                    ui.end_row();

                    ui.label("Background");
                    ui.add(widget::nodui_color(&mut node.style.body.background_color));
                    ui.end_row();

                    ui.label("Layout");
                    ui.add(widget::node_layout(&mut node.style.body.layout));
                    ui.end_row();

                    ui.label("Padding");
                    ui.add(widget::nodui_padding(&mut node.style.body.padding));
                    ui.end_row();

                    ui.label("Outline");
                    ui.add(widget::nodui_stroke(&mut node.style.outline));
                    ui.end_row();
                });

            ui.separator();

            if ui.button("➕").clicked() {
                node.add_socket();
            }

            egui::Grid::new(ui.id().with("socket node inspector grid"))
                .num_columns(8)
                .min_col_width(0.0)
                .show(ui, |ui| {
                    ui.horizontal(|_| {});
                    ui.horizontal(|_| {});
                    ui.horizontal(|_| {});
                    ui.label("Id");
                    ui.label("Name");
                    ui.label("Color");
                    ui.label("Side");
                    ui.label("Shape");
                    ui.end_row();

                    let len = node.sockets().len();

                    for (i, socket) in node.sockets_mut().iter_mut().enumerate() {
                        let is_first = i == 0;
                        let is_last = i == len - 1;

                        if ui.button("🗙").clicked() {
                            socket_action = SocketAction::Remove(socket.id());
                        }

                        ui.add_enabled_ui(!is_first, |ui| {
                            if ui.button("⏶").clicked() {
                                socket_action = SocketAction::MoveUp(socket.id());
                            }
                        });

                        ui.add_enabled_ui(!is_last, |ui| {
                            if ui.button("⏷").clicked() {
                                socket_action = SocketAction::MoveDown(socket.id());
                            }
                        });

                        ui.add(egui::Label::new(socket.id().to_string()).truncate());

                        widget::text_ui(ui, &mut socket.style.name);

                        ui.color_edit_button_srgba_unmultiplied(socket.style.color.as_array_mut());

                        ui.add(widget::node_side(&mut socket.style.side));

                        ui.add(widget::socket_shape(
                            ui.id().with(socket.id()),
                            &mut socket.style.shape,
                        ));

                        ui.end_row();
                    }
                });
        } else {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    ui.label("Click on a node or ");
                    if ui.link("create one").clicked() {
                        self.graph.new_node(self.editor_pos);
                    }
                    ui.label(" to edit it.");

                    ui.add_space(ui.available_width());
                });
            });
        }

        match socket_action {
            SocketAction::None => {}
            SocketAction::Remove(socket_id) => self.graph.remove_socket(socket_id),
            SocketAction::MoveUp(socket_id) => self.graph.move_socket_up(socket_id),
            SocketAction::MoveDown(socket_id) => self.graph.move_socket_down(socket_id),
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
            })
            .socket_context_menu(|ui, context| {
                if ui.button("Disconnect").clicked() {
                    context
                        .graph
                        .connections_mut()
                        .disconnect(context.socket_id);
                    ui.close_menu();
                }
            });

        let response = graph.show(ui);

        self.editor_pos = response.position;
        self.cursor_pos = response.pointer_latest_pos();

        if let Some(selected_node_id) = response.last_interacted_node_id {
            self.graph.selected_node = Some(selected_node_id);
        }
    }
}

/* -------------------------------------------------------------------------- */

enum SocketAction {
    None,
    Remove(SocketId),
    MoveUp(SocketId),
    MoveDown(SocketId),
}

/* -------------------------------------------------------------------------- */
