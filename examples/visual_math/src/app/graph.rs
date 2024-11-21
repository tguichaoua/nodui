//! Graph adapter for a math graph.

mod adapter;

use std::collections::HashMap;

use nodui::Pos;

use crate::graph::{
    BinaryOp, Graph, Input, InputId, InputSocketId, IntoOutputSocketId, NodeId, Op, OpNodeId,
    OutputSocketId, UnaryOp,
};

/* -------------------------------------------------------------------------- */

/// The adapter for the math graph that will render into the visual editor.
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
    expr: Option<Expr>,
}

/// An expression from the graph.
///
/// We use it to not having to recompute it at every frame.
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
            expr: None,
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
    pub fn rebuild_and_get_expr(&mut self) -> Option<&Expr> {
        self.rebuild_expr();

        self.expr.as_ref()
    }

    /// Get the expression of the currently selected node, if any.
    ///
    /// Rebuild the expression if dirty, and force the recomputation of the value.
    pub fn rebuild_recompute_and_get_expr(&mut self) -> Option<&Expr> {
        if !self.rebuild_expr() {
            if let Some(Expr { expr, value, .. }) = self.expr.as_mut() {
                *value = compute_value(expr, &self.graph);
            }
        }

        self.expr.as_ref()
    }

    /// Rebuild the expression if dirty.
    ///
    /// Returns `true` if the expression has been rebuild, `false` otherwise.
    fn rebuild_expr(&mut self) -> bool {
        if self.may_need_to_rebuild_expr {
            if let Some(selected_socket_id) = self.selected_node {
                let expr = self.graph.build_expr_from(selected_socket_id);

                if let Ok(expr) = expr {
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

                    self.expr = Some(Expr {
                        expr,
                        formula,
                        value,
                    });
                    self.may_need_to_rebuild_expr = false;

                    return true;
                }

                self.selected_node = None;
                self.expr = None;
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
