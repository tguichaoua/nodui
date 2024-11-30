use super::{CanvasPos, Grid};

/// The state of the editor saved from on frame to another.
#[derive(Clone)]
pub(super) struct EditorState<S> {
    /// The current viewport position.
    pub(super) viewport_position: CanvasPos,
    /// The grid of the editor.
    pub(super) grid: Grid,

    /// The node currently being dragged and the delta position form it's current position.
    pub(super) dragged_node: Option<(egui::Id, egui::Vec2)>,
    /// The socket currently being dragged.
    pub(super) dragged_socket: Option<S>,
}

impl<S> Default for EditorState<S> {
    fn default() -> Self {
        Self {
            viewport_position: CanvasPos::ZERO,
            grid: Grid { size: 10.0 },
            dragged_node: None,
            dragged_socket: None,
        }
    }
}

impl<S> EditorState<S>
where
    Self: Clone + Send + Sync + 'static,
{
    /// Loads the editor state.
    pub(super) fn load(ctx: &egui::Context, id: egui::Id) -> Self {
        ctx.data(|data| data.get_temp(id).unwrap_or_default())
    }

    /// Store the editor state.
    pub(super) fn store(self, ctx: &egui::Context, id: egui::Id) {
        ctx.data_mut(|data| data.insert_temp(id, self));
    }
}
