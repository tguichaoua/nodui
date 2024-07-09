//! The visual math app.

mod graph;

use egui::{CentralPanel, DragValue, Grid, SidePanel, Ui};
use nodui::Pos;

use crate::graph::{BinaryOp, Op, SocketId, UnaryOp};

use self::graph::GraphApp;

/// The visual math app.
pub struct App {
    /// The math graph.
    graph: GraphApp,

    /// The current graph position of the viewport.
    current_graph_pos: Pos,
}

impl App {
    /// Creates the [`App`].
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            graph: {
                // Build a graph with some nodes.

                let mut graph = GraphApp::new();

                let x = graph.add_input(Pos::new(-10, 5), "x", 3.0);
                let y = graph.add_input(Pos::new(-10, 0), "y", 2.0);
                let z = graph.add_input(Pos::new(-10, -5), "z", 2.0);

                let x_plus_y =
                    graph.add_binary_op_node_and_connect_input(Pos::new(0, 5), BinaryOp::Add, x, y);

                let neg_z =
                    graph.add_unary_op_node_and_connect_input(Pos::new(0, -5), UnaryOp::Neg, z);

                let x_plus_y_mul_neg_z = graph.add_binary_op_node_and_connect_input(
                    Pos::new(10, 5),
                    BinaryOp::Mul,
                    x_plus_y,
                    neg_z,
                );

                graph.set_selected_node(x_plus_y_mul_neg_z);

                graph
            },

            current_graph_pos: Pos::default(),
        }
    }
}

/* -------------------------------------------------------------------------- */

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("LEFT PANEL")
            .resizable(true)
            .show(ctx, |ui| {
                self.side_panel(ui);
            });

        CentralPanel::default().show(ctx, |ui| {
            self.graph_editor(ui);
        });
    }
}

impl App {
    /// Render the left panel.
    fn side_panel(&mut self, ui: &mut Ui) {
        let any_input_changed = Grid::new("INPUTS GRID")
            .min_col_width(100.0)
            .num_columns(2)
            .show(ui, |ui| {
                let mut any_changed = false;
                for input in self.graph.inputs_mut() {
                    ui.text_edit_singleline(input.name_mut());
                    any_changed |= ui.add(DragValue::new(input.value_mut())).changed();
                    ui.end_row();
                }
                any_changed
            })
            .inner;

        if ui.button("Add input").clicked() {
            self.graph.add_input(self.current_graph_pos, "x", 0.0);
        }

        ui.separator();

        let expr = if any_input_changed {
            self.graph.rebuild_recompute_and_get_expr()
        } else {
            self.graph.rebuild_and_get_expr()
        };

        if let Some(expr) = expr {
            ui.label(format!("{} = {}", expr.formula, expr.value));
        }
    }

    /// Render the visual graph editor.
    fn graph_editor(&mut self, ui: &mut Ui) {
        let graph = nodui::GraphEditor::new(&mut self.graph, "graph")
            /* ---------------------------------------------------------------------------------- */
            /* The main context menu, when the user right click on the background of the editor.  */
            /* ---------------------------------------------------------------------------------- */
            .context_menu(|ui, context| {
                new_node_menu(ui, |op| {
                    context.graph.add_op_node(context.pos, op);
                });
            })
            /* ---------------------------------------------------------------------------------- */
            /* The socket context menu, when the user right click on a socket.                    */
            /* ---------------------------------------------------------------------------------- */
            .socket_context_menu(|ui, context| match context.socket_id {
                SocketId::Output(output_socket_id) => {
                    if ui.button("Disconnect all").clicked() {
                        context.graph.disconnect_all(output_socket_id);
                        ui.close_menu();
                    }
                }
                SocketId::Input(input_socket_id) => {
                    if ui.button("Disconnect").clicked() {
                        context.graph.disconnect(input_socket_id);
                        ui.close_menu();
                    }
                }
            })
            /* ---------------------------------------------------------------------------------- */
            /* The node context menu, when the user right click on a node.                        */
            /* ---------------------------------------------------------------------------------- */
            .node_context_menu(|ui, context| {
                if ui.button("Select").clicked() {
                    context.graph.set_selected_node(context.node_id);
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Remove").clicked() {
                    context.graph.remove_node(context.node_id);
                    ui.close_menu();
                }
            });

        let response = graph.show(ui);

        self.current_graph_pos = response.position;
    }
}

/// A sub menu with all option to create a new node.
///
/// Call `on_clicked` with the user's choice when an option is selected.
fn new_node_menu(ui: &mut Ui, mut on_clicked: impl FnMut(Op)) {
    ui.menu_button("New", |ui| {
        if ui.button("Neg").clicked() {
            on_clicked(UnaryOp::Neg.into());
            ui.close_menu();
        }

        if ui.button("Add").clicked() {
            on_clicked(BinaryOp::Add.into());
            ui.close_menu();
        }

        if ui.button("Sub").clicked() {
            on_clicked(BinaryOp::Sub.into());
            ui.close_menu();
        }

        if ui.button("Mul").clicked() {
            on_clicked(BinaryOp::Mul.into());
            ui.close_menu();
        }

        if ui.button("Div").clicked() {
            on_clicked(BinaryOp::Div.into());
            ui.close_menu();
        }
    });
}
