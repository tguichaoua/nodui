mod graph;
mod widget;

use graph::GraphApp;
use nodui::Pos;
use serde::{Deserialize, Serialize};

use crate::graph::{NodeId, SocketId};

#[derive(Serialize, Deserialize)]
pub struct App {
    graph: GraphApp,

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
            graph: GraphApp::default(),
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
            // self.show_graph(ui);
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
                    ui.color_edit_button_srgba(&mut node.style.body.background_color);
                    ui.end_row();

                    ui.label("Layout");
                    ui.add(widget::node_layout(&mut node.style.body.layout));
                    ui.end_row();

                    ui.label("Padding");
                    ui.add(&mut node.style.body.padding);
                    ui.end_row();

                    ui.label("Outline");
                    ui.add(&mut node.style.outline);
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

                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut socket.style.name);
                            ui.color_edit_button_srgba(&mut socket.style.name_color);
                        });

                        ui.color_edit_button_srgba(&mut socket.style.color);

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
}

impl App {
    #[expect(clippy::too_many_lines)]
    fn show_graph(&mut self, ui: &mut egui::Ui) {
        let graph = nodui::GraphEditor::new("graph")
            .show(ui, |ui| {
                let mut node_command = NodeCommand::None;

                let crate::graph::ViewMut { connections, nodes } = self.graph.graph.view_mut();
                for node in nodes {
                    let mut pos = node.pos;

                    let node_response = ui.node(node.id(), &mut pos, |ui| {
                        match node.style.header.mode {
                            crate::graph::HeaderMode::None => {}
                            crate::graph::HeaderMode::Title => {
                                ui.header_title(
                                    &node.style.header.title,
                                    node.style.header.title_color,
                                    node.style.header.background,
                                );
                            }
                        }

                        ui.layout(node.style.body.layout);

                        for socket in node.sockets() {
                            let crate::graph::SocketStyle {
                                side,
                                ref name,
                                name_color,
                                shape,
                                color,
                            } = socket.style;

                            let is_connected = connections.is_connected(socket.id());

                            ui.socket(
                                nodui::Socket::new(socket.id(), side)
                                    .text(name)
                                    .text_color(name_color)
                                    .filled(is_connected)
                                    .shape(shape)
                                    .color(color),
                            );
                        }
                    });

                    node.pos = pos;

                    for socket in node_response.sockets {
                        socket.response.context_menu(|ui| {
                            if ui.button("Disconnect").clicked() {
                                connections.disconnect(socket.id);
                                ui.close_menu();
                            }
                        });
                    }

                    node_response.response.context_menu(|ui| {
                        if ui.button("Add socket").clicked() {
                            node.add_socket();
                            ui.close_menu();
                        }

                        if ui.button("Copy").clicked() {
                            self.graph.clipboard = Some((
                                node.style.clone(),
                                node.sockets().iter().map(|s| s.style.clone()).collect(),
                            ));
                            ui.close_menu();
                        }

                        ui.add_enabled_ui(self.graph.clipboard.is_some(), |ui| {
                            if ui.button("Paste style").clicked() {
                                node_command = NodeCommand::PasteStyle(node.id());
                                ui.close_menu();
                            }

                            if ui.button("Paste sockets").clicked() {
                                node_command = NodeCommand::PasteSockets(node.id());

                                ui.close_menu();
                            }
                        });

                        if ui.button("Remove").clicked() {
                            node_command = NodeCommand::Remove(node.id());
                            ui.close_menu();
                        }
                    });

                    if node_response.response.clicked() {
                        self.graph.selected_node = Some(node.id());
                    }
                }

                match node_command {
                    NodeCommand::None => { /* nothing */ }
                    NodeCommand::Remove(node_id) => {
                        self.graph.remove_node(node_id);
                    }
                    NodeCommand::PasteStyle(node_id) => {
                        self.graph.paste_node_settings_to(node_id);
                    }
                    NodeCommand::PasteSockets(node_id) => {
                        self.graph.paste_sockets_to(node_id);
                    }
                }
            })
            .show_connections(|ui| {
                ui.in_progress_connection_line_with_feedback(|_, target| {
                    if target.is_some() {
                        egui::Stroke::new(5.0, egui::Color32::GREEN)
                    } else {
                        egui::Stroke::new(3.0, egui::Color32::WHITE)
                    }
                });

                let connections = self.graph.connections();

                for (a, b) in connections.iter() {
                    ui.connect_line(&a, &b, (3.0, egui::Color32::WHITE));
                }
            })
            .finish();

        graph.response.context_menu(|ui| {
            let pos = graph.viewport.viewport_to_graph(ui.min_rect().left_top());

            if ui.button("New node").clicked() {
                self.graph.new_node(pos);
                ui.close_menu();
            }

            ui.add_enabled_ui(self.graph.clipboard.is_some(), |ui| {
                if ui.button("Paste node").clicked() {
                    self.graph.paste_node(pos);
                    ui.close_menu();
                }
            });
        });

        if let Some((a, b)) = graph.connection {
            self.graph.connections_mut().connect(a, b);
        }
    }
}

enum NodeCommand {
    None,
    Remove(NodeId),
    PasteStyle(NodeId),
    PasteSockets(NodeId),
}

/* -------------------------------------------------------------------------- */

enum SocketAction {
    None,
    Remove(SocketId),
    MoveUp(SocketId),
    MoveDown(SocketId),
}

/* -------------------------------------------------------------------------- */
