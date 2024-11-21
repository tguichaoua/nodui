use super::{NodeAdapter, SizeHint, SocketData};

pub trait GraphVisitor<'graph, N, S> {
    fn nodes(&mut self, size_hint: SizeHint) -> impl NodeSeq<'graph, N, S>;
}

pub trait NodeSeq<'graph, N, S> {
    fn visit_node(&mut self, node: impl NodeAdapter<NodeId = N, SocketId = S>);
}

pub trait NodeVisitor<'node, S> {
    fn sockets(&mut self, size_hint: SizeHint) -> impl SocketSeq<'node, S>;
}

pub trait SocketSeq<'node, S> {
    // fn visit_socket(&mut self, id: S, ui: SocketUI, field: Option<&'node mut f32>);
    fn visit_socket(&mut self, socket: SocketData<'node, S>);
}
