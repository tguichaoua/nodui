//! Graph adapter for a math graph.

mod adapter;

use std::collections::HashMap;

use nodui::{Pos, Socket};

use crate::graph::{
    BinaryOp, Graph, Input, InputId, InputSocketId, IntoOutputSocketId, NodeId, Op, OpNodeId,
    OutputSocketId, SocketId, UnaryOp,
};

/* -------------------------------------------------------------------------- */

/// The result of the computation of the expression from the selected node of the graph.
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ExprResult {
    /// There is no expression computed, yet.
    None,
    /// The computed expression.
    Expr(Expr),
    /// The computation of the expression fails due to a loop in the expression.
    LoopError,
}

/// The adapter for the math graph that will render into the visual editor.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GraphApp {
    /// The math graph.
    graph: Graph,
    /// The position of the node of the graph.
    positions: HashMap<NodeId, Pos>,

    /// The currently selected node.
    selected_node: Option<NodeId>,

    /// Whether or not we need to rebuild the expr.
    may_need_to_rebuild_expr: bool,
    /// The last built expression.
    expr: ExprResult,
}

/// An expression from the graph.
///
/// We use it to not having to recompute it at every frame.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Expr {
    /// The math expression.
    #[allow(clippy::struct_field_names)]
    pub expr: crate::graph::Expr,
    /// The string representation of the expression.
    pub formula: String,
    /// The last computed value of the expression.
    pub value: f32,
}

impl GraphApp {
    /// Creates an empty [`GraphApp`].
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            positions: HashMap::new(),
            selected_node: None,

            may_need_to_rebuild_expr: false,
            expr: ExprResult::None,
        }
    }

    /// Sets the selected node.
    pub fn set_selected_node(&mut self, node_id: impl Into<NodeId>) {
        self.selected_node = Some(node_id.into());
        self.may_need_to_rebuild_expr = true;
    }

    /// Get the expression of the currently selected node, if any.
    ///
    /// Rebuild the expression if dirty.
    pub fn rebuild_and_get_expr(&mut self) -> &ExprResult {
        self.rebuild_expr();

        &self.expr
    }

    /// Get the expression of the currently selected node, if any.
    ///
    /// Rebuild the expression if dirty, and force the recomputation of the value.
    pub fn rebuild_recompute_and_get_expr(&mut self) -> &ExprResult {
        if !self.rebuild_expr() {
            if let ExprResult::Expr(Expr { expr, value, .. }) = &mut self.expr {
                *value = compute_value(expr, &self.graph);
            }
        }

        &self.expr
    }

    /// Rebuild the expression if dirty.
    ///
    /// Returns `true` if the expression has been rebuild, `false` otherwise.
    fn rebuild_expr(&mut self) -> bool {
        if self.may_need_to_rebuild_expr {
            if let Some(selected_socket_id) = self.selected_node {
                let expr = self.graph.build_expr_from(selected_socket_id);

                match expr {
                    Ok(expr) => {
                        let formula = expr
                            .display(|input_id, f| {
                                if let Some(input) = self.graph.get_input(input_id) {
                                    f.write_str(input.name())
                                } else {
                                    f.write_str("?")
                                }
                            })
                            .to_string();

                        let value = compute_value(&expr, &self.graph);

                        self.expr = ExprResult::Expr(Expr {
                            expr,
                            formula,
                            value,
                        });
                        self.may_need_to_rebuild_expr = false;

                        return true;
                    }
                    Err(crate::graph::BuildExprError::NodeNotFound) => {
                        self.selected_node = None;
                        self.expr = ExprResult::None;
                    }
                    Err(crate::graph::BuildExprError::Loop) => {
                        self.expr = ExprResult::LoopError;
                    }
                }
            }
        }

        false
    }
}

/// Computes the value of the expression.
fn compute_value(expr: &crate::graph::Expr, graph: &Graph) -> f32 {
    expr.eval(|input_id| {
        input_id
            .and_then(|input_id| graph.get_input(input_id).map(Input::value))
            .unwrap_or(0.0)
    })
}

impl GraphApp {
    /// A mutable references to the inputs of the graph.
    pub fn inputs_mut(&mut self) -> &mut [Input] {
        self.graph.inputs_mut()
    }

    /// Creates a new input.
    pub fn add_input(&mut self, pos: Pos, name: impl Into<String>, value: f32) -> InputId {
        let id = self.graph.add_input(name, value);
        self.positions.insert(id.into(), pos);
        id
    }
}

impl GraphApp {
    /// Creates an operation node.
    pub fn add_op_node(&mut self, pos: Pos, op: Op) -> OpNodeId {
        let id = self.graph.add_op_node(op);
        self.positions.insert(id.into(), pos);
        id
    }

    /// Creates an unary operation node and connect its input to the specified socket.
    pub fn add_unary_op_node_and_connect_input(
        &mut self,
        pos: Pos,
        op: UnaryOp,
        socket_to_connect: impl IntoOutputSocketId,
    ) -> OpNodeId {
        let id = self
            .graph
            .add_unary_op_node_and_connect_input(op, socket_to_connect);
        self.positions.insert(id.into(), pos);
        self.may_need_to_rebuild_expr = true;
        id
    }

    /// Creates an binary operation node and connect its inputs to the specified sockets.
    pub fn add_binary_op_node_and_connect_input(
        &mut self,
        pos: Pos,
        op: BinaryOp,
        socket_to_connect_a: impl IntoOutputSocketId,
        socket_to_connect_b: impl IntoOutputSocketId,
    ) -> OpNodeId {
        let id = self.graph.add_binary_op_node_and_connect_input(
            op,
            socket_to_connect_a,
            socket_to_connect_b,
        );
        self.positions.insert(id.into(), pos);
        self.may_need_to_rebuild_expr = true;
        id
    }
}

impl GraphApp {
    /// Removes a node.
    pub fn remove_node(&mut self, id: NodeId) {
        self.graph.remove(id);
        self.may_need_to_rebuild_expr = true;
    }

    /// Disconnect an input socket.
    pub fn disconnect(&mut self, socket: InputSocketId) {
        self.graph.connections_mut().disconnect(socket);
        self.may_need_to_rebuild_expr = true;
    }

    /// Disconnect an output socket.
    pub fn disconnect_all(&mut self, socket: OutputSocketId) {
        self.graph.connections_mut().disconnect_all(socket);
        self.may_need_to_rebuild_expr = true;
    }
}

/* -------------------------------------------------------------------------- */

impl GraphApp {
    #[allow(clippy::missing_docs_in_private_items)] // TODO: docs
    #[allow(clippy::too_many_lines)]
    pub fn show_nodes(&mut self, ui: &mut nodui::GraphUi<SocketId>) {
        enum Command {
            None,
            Remove(NodeId),
            Select(NodeId),
            Disconnect(SocketId),
        }

        let crate::graph::ViewMut {
            nodes,
            inputs,
            connections,
        } = self.graph.view_mut();

        let mut command = Command::None;

        let mut handle_node_response =
            |node_id: NodeId, node_response: nodui::NodeResponse<'_, (), SocketId>| {
                for socket in node_response.sockets {
                    socket.response.context_menu(|ui| {
                        if ui.button("Disconnect").clicked() {
                            command = Command::Disconnect(socket.id);
                            ui.close_menu();
                        }
                    });
                }

                node_response.response.context_menu(|ui| {
                    if ui.button("Select").clicked() {
                        command = Command::Select(node_id);

                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Remove").clicked() {
                        command = Command::Remove(node_id);
                        ui.close_menu();
                    }
                });
            };

        {
            for node in nodes.iter() {
                let pos = self.positions.entry(node.id().into()).or_default();

                let node_response = ui.node(node.id(), pos, |ui| {
                    if self.selected_node == Some(NodeId::from(node.id())) {
                        ui.outline((2.0, egui::Color32::RED));
                    } else {
                        ui.outline((1.0, egui::Color32::WHITE));
                    }

                    let input_sockets = node.input_socket_ids();

                    for socket in input_sockets {
                        ui.socket(
                            Socket::new(socket.into(), nodui::ui::NodeSide::Left)
                                .filled(connections.is_connected(socket.into()))
                                .text(socket.name()),
                        );
                    }

                    {
                        let output_id = node.output_socket().into();

                        let output_name = match node.op() {
                            Op::Unary(UnaryOp::Neg) => "-A",
                            Op::Binary(BinaryOp::Add) => "A+B",
                            Op::Binary(BinaryOp::Sub) => "A-B",
                            Op::Binary(BinaryOp::Mul) => "A*B",
                            Op::Binary(BinaryOp::Div) => "A/B",
                        };

                        ui.socket(
                            Socket::new(output_id, nodui::ui::NodeSide::Right)
                                .filled(connections.is_connected(output_id))
                                .text(output_name),
                        );
                    }
                });

                handle_node_response(node.id().into(), node_response);
            }
        }

        {
            for node in inputs {
                let pos = self.positions.entry(node.id().into()).or_default();

                let node_response = ui.node(node.id(), pos, |ui| {
                    if self.selected_node == Some(NodeId::from(node.id())) {
                        ui.outline((2.0, egui::Color32::RED));
                    } else {
                        ui.outline((1.0, egui::Color32::WHITE));
                    }

                    let socket_id = node.output_socket_id().into();
                    ui.socket(
                        Socket::new(socket_id, nodui::ui::NodeSide::Right)
                            .filled(connections.is_connected(socket_id))
                            .text(node.name()),
                    );
                });

                handle_node_response(node.id().into(), node_response);
            }
        }

        match command {
            Command::None => {}
            Command::Remove(node_id) => {
                self.remove_node(node_id);
            }
            Command::Select(node_id) => {
                self.set_selected_node(node_id);
            }
            Command::Disconnect(socket_id) => match socket_id {
                SocketId::Output(output_socket_id) => self.disconnect_all(output_socket_id),
                SocketId::Input(input_socket_id) => self.disconnect(input_socket_id),
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
