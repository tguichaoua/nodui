#![allow(missing_docs, clippy::missing_docs_in_private_items)] // TODO: docs

use crate::{
    ui::{NodeUI, SocketUI},
    Id, Pos,
};

/* -------------------------------------------------------------------------- */

pub trait GraphAdapter {
    type NodeId: Id;
    type SocketId: Id;

    fn accept<'graph, V>(&'graph mut self, visitor: V)
    where
        V: GraphVisitor<'graph, Self::NodeId, Self::SocketId>;
}

pub trait NodeAdapter {
    type NodeId: Id;
    type SocketId: Id;

    fn id(&self) -> Self::NodeId;

    fn pos(&self) -> Pos;

    fn set_pos(&mut self, pos: Pos);

    #[inline]
    fn ui(&self) -> NodeUI {
        NodeUI::default()
    }

    fn accept<'node, V>(&'node mut self, visitor: V)
    where
        V: NodeVisitor<'node, Self::SocketId>;
}

/* -------------------------------------------------------------------------- */

pub trait GraphVisitor<'graph, N, S> {
    fn nodes(&mut self, size_hint: SizeHint) -> impl NodeSeq<'graph, N, S>;
}

pub trait NodeVisitor<'node, S> {
    fn sockets(&mut self, size_hint: SizeHint) -> impl SocketSeq<'node, S>;
}

pub trait NodeSeq<'graph, N, S> {
    fn visit_node(&mut self, node: impl NodeAdapter<NodeId = N, SocketId = S>);
}

pub trait SocketSeq<'node, S> {
    // TODO: make `field` generic
    fn visit_socket(&mut self, id: S, ui: SocketUI, field: Option<&'node mut f32>);
}

/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone, Copy)]
pub struct SizeHint {
    min: usize,
    max: Option<usize>,
}

impl SizeHint {
    #[must_use]
    #[inline]
    pub fn min(&self) -> usize {
        self.min
    }
    #[must_use]
    #[inline]
    pub fn max(&self) -> Option<usize> {
        self.max
    }

    #[must_use]
    #[inline]
    pub fn of<T>(x: &[T]) -> Self {
        Self::exact(x.len())
    }

    #[must_use]
    #[inline]
    pub fn of_iter<I: Iterator>(iter: &I) -> Self {
        let (min, max) = iter.size_hint();
        Self { min, max }
    }

    #[must_use]
    #[inline]
    pub fn exact(count: usize) -> Self {
        Self {
            min: count,
            max: Some(count),
        }
    }

    #[must_use]
    #[inline]
    pub fn at_least(count: usize) -> Self {
        Self {
            min: count,
            max: None,
        }
    }

    #[must_use]
    #[inline]
    pub fn at_most(count: usize) -> Self {
        Self {
            min: 0,
            max: Some(count),
        }
    }
}

/* -------------------------------------------------------------------------- */
