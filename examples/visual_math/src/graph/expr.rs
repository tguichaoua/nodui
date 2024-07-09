//! Math expression.

use std::fmt::{Debug, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::id::{InputId, OpNodeId, SocketIndex};
use super::node::{BinaryOp, UnaryOp};
use super::{Graph, NodeId, Op};

/// A math expression that can be built from a graph.
pub enum Expr {
    /// The value is connected to anything.
    Unconnected,
    /// The value is an input.
    Input(InputId),
    /// The negative operation.
    Neg(Box<Expr>),
    /// The addition operation.
    Add(Box<Expr>, Box<Expr>),
    /// The subtract operation.
    Sub(Box<Expr>, Box<Expr>),
    /// The multiplication operation.
    Mul(Box<Expr>, Box<Expr>),
    /// The division operation.
    Div(Box<Expr>, Box<Expr>),
}

/// An error that can occurs when the building of an [`Expr`] fails.
pub struct BuildExprError(());

impl Debug for BuildExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BuildExprError")
    }
}

impl Display for BuildExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("The provided socket didn't not exists in the graph")
    }
}

impl std::error::Error for BuildExprError {}

impl Graph {
    /// Build the [`Expr`] from the specified node.
    pub fn build_expr_from(&self, node_id: NodeId) -> Result<Expr, BuildExprError> {
        self.build_expr_from_inner(Some(node_id))
    }

    /// Build the [`Expr`] from the specified node.
    fn build_expr_from_inner(&self, node_id: Option<NodeId>) -> Result<Expr, BuildExprError> {
        let Some(node_id) = node_id else {
            return Ok(Expr::Unconnected);
        };

        match node_id {
            NodeId::Op(node_id) => {
                let Some(node) = self.get_op_node(node_id) else {
                    return Err(BuildExprError(()));
                };

                match node.op() {
                    Op::Unary(UnaryOp::Neg) => self.build_expr_unary(node.id(), Expr::Neg),
                    Op::Binary(BinaryOp::Add) => self.build_expr_binary(node.id(), Expr::Add),
                    Op::Binary(BinaryOp::Sub) => self.build_expr_binary(node.id(), Expr::Sub),
                    Op::Binary(BinaryOp::Mul) => self.build_expr_binary(node.id(), Expr::Mul),
                    Op::Binary(BinaryOp::Div) => self.build_expr_binary(node.id(), Expr::Div),
                }
            }
            NodeId::Input(node_id) => {
                let Some(node) = self.get_input(node_id) else {
                    return Err(BuildExprError(()));
                };

                Ok(Expr::Input(node.id()))
            }
        }
    }

    /// Build an unary operation expression.
    fn build_expr_unary(
        &self,
        node_id: OpNodeId,
        expr: impl FnOnce(Box<Expr>) -> Expr,
    ) -> Result<Expr, BuildExprError> {
        Ok(expr(Box::new(
            self.build_expr_from_inner(
                self.connections
                    .get(node_id.input_socket_id(SocketIndex::A))
                    .map(|socket_id| socket_id.node_id),
            )?,
        )))
    }

    /// Build an binary operation expression.
    fn build_expr_binary(
        &self,
        node_id: OpNodeId,
        expr: impl FnOnce(Box<Expr>, Box<Expr>) -> Expr,
    ) -> Result<Expr, BuildExprError> {
        Ok(expr(
            Box::new(
                self.build_expr_from_inner(
                    self.connections
                        .get(node_id.input_socket_id(SocketIndex::A))
                        .map(|socket_id| socket_id.node_id),
                )?,
            ),
            Box::new(
                self.build_expr_from_inner(
                    self.connections
                        .get(node_id.input_socket_id(SocketIndex::B))
                        .map(|socket_id| socket_id.node_id),
                )?,
            ),
        ))
    }
}

impl Expr {
    /// Evaluates the expression.
    ///
    /// The callback is used to provided the value for the inputs.
    /// It receives `Some(input_id)` to evaluate the value of a input, and
    /// `None` to evaluate the value for [`Expr::Unconnected`].
    pub fn eval<T, F>(&self, eval_input: F) -> T
    where
        F: Clone + Fn(Option<InputId>) -> T,
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        match self {
            Expr::Unconnected => eval_input(None),
            Expr::Input(input) => eval_input(Some(*input)),
            Expr::Neg(a) => -(a.eval(eval_input)),
            Expr::Add(lhs, rhs) => lhs.eval(eval_input.clone()) + rhs.eval(eval_input),
            Expr::Sub(lhs, rhs) => lhs.eval(eval_input.clone()) - rhs.eval(eval_input),
            Expr::Mul(lhs, rhs) => lhs.eval(eval_input.clone()) * rhs.eval(eval_input),
            Expr::Div(lhs, rhs) => lhs.eval(eval_input.clone()) / rhs.eval(eval_input),
        }
    }

    /// Makes this expression displayable.
    ///
    /// The callback is used to display the [`Expr::Input`].
    pub fn display<'a, F>(&'a self, fmt_input: F) -> impl Display + 'a
    where
        F: Fn(InputId, &mut std::fmt::Formatter<'_>) -> std::fmt::Result + 'a,
    {
        /// A struct that implement [`Display`].
        struct Display<'a, F> {
            /// The [`Expr`] to display.
            expr: &'a Expr,
            /// The callback used to display [`Expr::Input`].
            fmt_input: F,
        }

        impl<F> std::fmt::Display for Display<'_, F>
        where
            F: Fn(InputId, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let Self { expr, fmt_input } = self;
                expr.display_with(f, fmt_input)
            }
        }

        Display {
            expr: self,
            fmt_input,
        }
    }

    /// Display this expression.
    fn display_with<F>(&self, f: &mut std::fmt::Formatter<'_>, fmt_input: &F) -> std::fmt::Result
    where
        F: Fn(InputId, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
    {
        match self {
            Expr::Unconnected => f.write_str("?"),
            Expr::Input(input_id) => fmt_input(*input_id, f),
            Expr::Neg(x) => {
                f.write_str("-")?;

                if matches!(
                    &**x,
                    Expr::Add(_, _) | Expr::Sub(_, _) | Expr::Mul(_, _) | Expr::Div(_, _)
                ) {
                    f.write_str("( ")?;
                    x.display_with(f, fmt_input)?;
                    f.write_str(" )")
                } else {
                    x.display_with(f, fmt_input)
                }
            }
            Expr::Add(lhs, rhs) => {
                lhs.display_with(f, fmt_input)?;
                f.write_str(" + ")?;
                rhs.display_with(f, fmt_input)
            }
            Expr::Sub(lhs, rhs) => {
                lhs.display_with(f, fmt_input)?;
                f.write_str(" - ")?;
                rhs.display_with(f, fmt_input)
            }
            Expr::Mul(lhs, rhs) => {
                f.write_str("( ")?;
                lhs.display_with(f, fmt_input)?;
                f.write_str(" ) * ( ")?;
                rhs.display_with(f, fmt_input)?;
                f.write_str(" )")
            }
            Expr::Div(lhs, rhs) => {
                f.write_str("( ")?;
                lhs.display_with(f, fmt_input)?;
                f.write_str(" ) / ( ")?;
                rhs.display_with(f, fmt_input)?;
                f.write_str(" )")
            }
        }
    }
}
