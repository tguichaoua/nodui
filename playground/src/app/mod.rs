mod adapter;

use egui::{Color32, Grid, Ui};
use nodui::{GraphEditor, Pos};
use serde::{Deserialize, Serialize};

use crate::graph::{self, DummyGraph, NodeId};

#[derive(Serialize, Deserialize)]
pub struct App {
    graph: DummyGraph,

    #[serde(skip)]
    look_at: Option<Pos>,
    #[serde(skip)]
    menu_look_at: Pos,

    #[serde(skip)]
    graph_pointer_pos: Option<Pos>,

    #[serde(skip)]
    last_interacted_node_id: Option<NodeId>,

    background_color: Color32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            graph: graph::make_dummy(),
            look_at: None,
            menu_look_at: Pos::default(),
            graph_pointer_pos: None,
            last_interacted_node_id: None,
            background_color: Color32::BLACK,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("TOP").show(ctx, |_ui| {
            // egui::menu::bar(ui, |ui| {
            //     ui.menu_button("File", |ui| {
            //         if ui.button("Open").clicked() {
            //             println!("OPEN !");
            //         }
            //     });
            // });
        });

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            ui.add(egui::DragValue::new(&mut self.menu_look_at.x).speed(0.1));
            ui.add(egui::DragValue::new(&mut self.menu_look_at.y).speed(0.1));

            if ui.button("look at").clicked() {
                self.look_at = Some(self.menu_look_at);
            }

            ui.separator();

            Grid::new("selected node").show(ui, |ui| {
                {
                    ui.label("node id");

                    let node_id = if let Some(node_id) = self.last_interacted_node_id {
                        if let Some(node) = self.graph.get_node(node_id) {
                            node.id().to_string()
                        } else {
                            String::from("node not found")
                        }
                    } else {
                        String::from("no node selected")
                    };

                    ui.label(node_id);
                }

                ui.end_row();
            });

            ui.separator();

            Grid::new("editor settings").show(ui, |ui| {
                ui.label("background color");
                ui.color_edit_button_srgba(&mut self.background_color);
                ui.end_row();
            });
        });

        egui::TopBottomPanel::bottom("bottom panel").show(ctx, |ui| {
            if let Some(last_pos) = self.graph_pointer_pos {
                ui.label(format!("[ {}, {} ]", last_pos.x, last_pos.y));
            } else {
                ui.label("[ ?, ? ]");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.show_graph_editor(ui);
        });
    }
}

impl App {
    fn show_graph_editor(&mut self, ui: &mut Ui) {
        let graph = GraphEditor::new(
            self::adapter::GraphAdapter::new(&mut self.graph),
            "graph editor",
        )
        .background_color(self.background_color)
        .context_menu(|ui, context| {
            ui.label(format!("Pos: {:?}", context.pos));

            ui.separator();

            if ui.button("Add node").clicked() {
                context.graph.graph.add_node(context.pos, ["In"], ["Out"]);
                ui.close_menu();
            }
        })
        .node_context_menu(|ui, context| {
            ui.label(format!("Node: {:?}", context.node_id));

            ui.separator();

            if ui.button("Remove").clicked() {
                context.graph.graph.remove_node(context.node_id);
                ui.close_menu();
            }
        })
        .socket_context_menu(|ui, context| {
            ui.label(format!("Socket: {:?}", context.socket_id));

            ui.separator();

            match context.socket_id {
                graph::SocketId::Input(socket_id) => {
                    context.graph.graph.connections_mut().disconnect(socket_id);
                }
                graph::SocketId::Output(socket_id) => {
                    context
                        .graph
                        .graph
                        .connections_mut()
                        .disconnect_all(socket_id);
                }
            }
        });

        let graph = if let Some(look_at) = self.look_at.take() {
            graph.look_at(look_at)
        } else {
            graph
        };

        let response = graph.show(ui);

        self.graph_pointer_pos = response.pointer_latest_pos();
        if let Some(last_interacted_node_id) = response.last_interacted_node_id {
            self.last_interacted_node_id = Some(last_interacted_node_id);
        }
    }
}
/* -------------------------------------------------------------------------- */
