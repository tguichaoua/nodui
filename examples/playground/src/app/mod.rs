mod graph;
mod widget;

use graph::GraphApp;
use nodui::Pos;
use serde::{Deserialize, Serialize};

use crate::graph::{NodeId, SocketId};

#[derive(Serialize, Deserialize)]
pub struct App {
    graph: GraphApp,

    show_grid: bool,

    #[serde(skip)]
    viewport_position: Pos,
    #[serde(skip)]
    cursor_pos: Option<Pos>,
}

impl Default for App {
    #[inline]
    fn default() -> Self {
        Self {
            graph: GraphApp::default(),
            show_grid: false,
            viewport_position: Pos::default(),
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

        // Check that the selected node has the focus to avoid conflict with text field keyboard shortcut.
        let focused_node = self
            .graph
            .selected_node
            .and_then(|(node_id, id)| ctx.memory(|mem| mem.has_focus(id)).then_some(node_id));

        ctx.input_mut(|input| {
            for ev in &input.events {
                #[allow(clippy::wildcard_enum_match_arm)]
                match ev {
                    egui::Event::Copy => {
                        if let Some(node_id) = focused_node {
                            self.graph.copy_node(node_id);
                        }
                    }
                    egui::Event::Paste(_) => {
                        if let Some(pos) = self.cursor_pos {
                            self.graph.paste_node(pos);
                        }
                    }
                    egui::Event::Cut => {
                        if let Some(node_id) = focused_node {
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
                ui.label("Show Grid?");
                ui.add(egui::Checkbox::without_text(&mut self.show_grid));
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
                    ui.add(widget::maybe_color(&mut node.style.body.background_color));
                    ui.end_row();

                    ui.label("Layout");
                    ui.add(widget::node_layout(&mut node.style.body.layout));
                    ui.end_row();

                    ui.label("Outline");
                    ui.add(widget::maybe(&mut node.style.outline));
                    ui.end_row();
                });

            ui.separator();

            if ui.button("➕").clicked() {
                node.add_socket();
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                let len = node.sockets().len();
                for (i, socket) in node.sockets_mut().iter_mut().enumerate() {
                    let is_first = i == 0;
                    let is_last = i == len - 1;

                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.horizontal(|ui| {
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
                        });

                        egui::Grid::new(
                            ui.id().with("socket node inspector grid").with(socket.id()),
                        )
                        .striped(true)
                        .num_columns(2)
                        .min_col_width(0.0)
                        .min_row_height(25.0)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.horizontal(|ui| {
                                ui.add(widget::maybe_color(&mut socket.style.name_color));
                                ui.add(
                                    egui::TextEdit::singleline(&mut socket.style.name)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                            ui.end_row();

                            ui.label("Shape");
                            ui.horizontal(|ui| {
                                ui.add(widget::maybe_color(&mut socket.style.color));
                                ui.add(widget::socket_shape(
                                    ui.id().with(socket.id()),
                                    &mut socket.style.shape,
                                ));
                            });
                            ui.end_row();

                            ui.label("Side");
                            ui.add(widget::node_side(&mut socket.style.side));
                            ui.end_row();
                        });
                    });
                }
            });
        } else {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;

                    ui.label("Click on a node or ");
                    if ui.link("create one").clicked() {
                        self.graph.new_node(self.viewport_position);
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
            .show_grid(self.show_grid)
            .show(ui, |ui| {
                let mut node_command = NodeCommand::None;

                let crate::graph::ViewMut { connections, nodes } = self.graph.graph.view_mut();
                for node in nodes {
                    let mut pos = node.pos;

                    let node_response = ui.node(node.id(), &mut pos, |ui| {
                        match node.style.header.mode {
                            crate::graph::HeaderMode::None => {}
                            crate::graph::HeaderMode::Title => {
                                let text_color = node
                                    .style
                                    .header
                                    .title_color
                                    .get()
                                    .copied()
                                    .unwrap_or(egui::Color32::PLACEHOLDER);

                                let mut header = nodui::TitleHeader::new(
                                    egui::RichText::new(&node.style.header.title)
                                        .color(text_color)
                                        .monospace(),
                                );

                                if let Some(background) =
                                    node.style.header.background.get().copied()
                                {
                                    header = header.background_color(background);
                                }

                                ui.header(header);
                            }
                        }

                        if let Some(outline) = node.style.outline.get().copied() {
                            ui.outline(outline);
                        }

                        if let Some(background_color) =
                            node.style.body.background_color.get().copied()
                        {
                            ui.background_color(background_color);
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

                            let mut text = egui::RichText::new(name);

                            if let Some(name_color) = name_color.get().copied() {
                                text = text.color(name_color);
                            }

                            let mut socket = nodui::Socket::new(socket.id(), side)
                                .text(text)
                                .shape(shape)
                                .filled(is_connected);

                            if let Some(color) = color.get().copied() {
                                socket = socket.color(color);
                            }

                            ui.socket(socket);
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

                    if node_response.response.has_focus() {
                        self.graph.selected_node = Some((node.id(), node_response.response.id));
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
                let color = ui.preferred_color();

                ui.in_progress_connection_line_with_feedback(|_, target| {
                    if target.is_some() {
                        egui::Stroke::new(5.0, egui::Color32::GREEN)
                    } else {
                        egui::Stroke::new(3.0, color)
                    }
                });

                let connections = self.graph.connections();

                for (a, b) in connections.iter() {
                    ui.connect_line(&a, &b, (3.0, color));
                }
            });

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

        self.viewport_position = graph.position;

        self.cursor_pos = graph
            .response
            .ctx
            .pointer_latest_pos()
            .map(|pointer| graph.viewport.viewport_to_graph(pointer));

        {
            let mut ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(graph.response.rect)
                    .layout(egui::Layout::bottom_up(egui::Align::Min)),
            );

            ui.with_layer_id(
                egui::LayerId::new(egui::Order::Tooltip, graph.response.id),
                |ui| {
                    egui::Frame::group(ui.style())
                        .rounding(egui::Rounding::ZERO)
                        .fill(ui.visuals().extreme_bg_color)
                        .show(ui, |ui| {
                            ui.add(egui::Label::new(format!(
                                "{}, {}",
                                self.viewport_position.x, self.viewport_position.y
                            )));

                            ui.set_min_width(50.0);
                        });
                },
            );
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
