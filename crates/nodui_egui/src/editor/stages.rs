pub struct Viewport {
    pub(super) width: Option<f32>,
    pub(super) height: Option<f32>,
    pub(super) view_aspect: Option<f32>,
    pub(super) min_size: egui::Vec2,
}

pub struct Nodes<S> {
    pub(super) ui: egui::Ui,
    pub(super) state: super::EditorState<S>,
    pub(super) viewport: super::Viewport,
    pub(super) response: egui::Response,
}

pub struct Connections<S> {
    pub(super) ui: egui::Ui,
    pub(super) state: super::EditorState<S>,
    pub(super) viewport: super::Viewport,
    pub(super) response: egui::Response,
    pub(super) sockets: Vec<super::RenderedSocket<S>>,
    pub(super) socket_interaction: crate::socket::SocketInteraction<S>,
}

pub struct End<S> {
    pub(super) ui: egui::Ui,
    pub(super) state: super::EditorState<S>,
    pub(super) viewport: super::Viewport,
    pub(super) response: egui::Response,
    pub(super) sockets: Vec<super::RenderedSocket<S>>,
    pub(super) connection: Option<(S, S)>,
}
