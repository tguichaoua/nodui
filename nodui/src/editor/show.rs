//! The rendering of the viewport.

use egui::{
    epaint::RectShape, pos2, vec2, Color32, NumExt, Rect, Rounding, Shape, Stroke, UiBuilder,
};

use crate::{misc::collector::Collector, viewport::Viewport, Pos};

use super::{stages, EditorState, GraphEditor, GraphUi};

/* -------------------------------------------------------------------------- */

impl GraphEditor<stages::Settings> {
    /// Move the viewport to make `pos` on the center of the viewport.
    #[inline]
    #[must_use]
    pub fn look_at(mut self, pos: Pos) -> Self {
        self.stage.look_at = Some(pos);
        self
    }

    /// The stroke used to render the background grid.
    ///
    /// Use [`Stroke::NONE`] to disable the grid.
    #[inline]
    #[must_use]
    pub fn grid_stroke(mut self, stroke: impl Into<Stroke>) -> Self {
        self.stage.grid_stroke = stroke.into();
        self
    }

    /// The color of the editor's background.
    #[inline]
    #[must_use]
    pub fn background_color(mut self, background_color: impl Into<Color32>) -> Self {
        self.stage.background_color = background_color.into();
        self
    }

    /// `width / height` ratio of the editor region.
    ///
    /// By default no fixed aspect ratio is set (and width/height will fill the ui it is in).
    #[inline]
    #[must_use]
    pub fn view_aspect(mut self, view_aspect: f32) -> Self {
        self.stage.view_aspect = Some(view_aspect);
        self
    }

    /// Width of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the width can be calculated from the height.
    #[inline]
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.stage.min_size.x = width;
        self.stage.width = Some(width);
        self
    }

    /// Height of the editor. By default it will fill the ui it is in.
    ///
    /// If you set [`Self::view_aspect`], the height can be calculated from the width.
    #[inline]
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.stage.min_size.y = height;
        self.stage.height = Some(height);
        self
    }

    /// Shows the viewport of the editor.
    #[inline]
    #[allow(clippy::too_many_lines)] // TODO: refactorize
    pub fn show<S>(
        self,
        ui: &mut egui::Ui,
        build_fn: impl FnOnce(&mut GraphUi<S>),
    ) -> GraphEditor<stages::Connections<S>>
    where
        S: PartialEq + Send + Sync + Clone + 'static,
    {
        let Self {
            id,
            stage:
                stages::Settings {
                    grid_stroke,
                    background_color,
                    look_at,
                    width,
                    height,
                    view_aspect,
                    min_size,
                },
        } = self;

        // TODO
        let can_connect_socket = true;

        /* ---- */

        let pos = ui.available_rect_before_wrap().min;

        let size = {
            let width = width
                .unwrap_or_else(|| {
                    if let (Some(height), Some(aspect)) = (height, view_aspect) {
                        height * aspect
                    } else {
                        ui.available_size_before_wrap().x
                    }
                })
                .at_least(min_size.x);

            let height = height
                .unwrap_or_else(|| {
                    if let Some(aspect) = view_aspect {
                        width / aspect
                    } else {
                        ui.available_size_before_wrap().y
                    }
                })
                .at_least(min_size.y);

            vec2(width, height)
        };

        let rect = Rect::from_min_size(pos, size);

        ui.advance_cursor_after_rect(rect);

        let mut ui = ui.new_child(UiBuilder::new().id_salt(id).max_rect(rect));
        ui.set_clip_rect(rect);

        /* ---- */

        let mut state = EditorState::<S>::load(ui.ctx(), id);

        /* ---- */

        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if response.dragged() {
            response.ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            state.viewport_position -= response.drag_delta();
        }

        let viewport = {
            if let Some(look_at) = look_at {
                let pos = state.grid.graph_to_canvas(look_at);
                state.viewport_position = pos;
            }

            Viewport {
                position: rect.center().to_vec2() - state.viewport_position.to_vec2(),
                grid: state.grid.clone(),
            }
        };

        /* ---- */

        // Paint the background
        ui.painter()
            .add(RectShape::filled(rect, Rounding::ZERO, background_color));

        /* ---- */

        // Paint the grid
        if !grid_stroke.is_empty() {
            let dx = state.viewport_position.to_vec2().x % state.grid.size;
            let dy = state.viewport_position.to_vec2().y % state.grid.size;

            let center = rect.center() - vec2(dx, dy);

            #[allow(clippy::cast_possible_truncation)]
            let n = (size.x / state.grid.size) as i32 / 2;
            #[allow(clippy::cast_possible_truncation)]
            let m = (size.y / state.grid.size) as i32 / 2;

            for x in (-n)..(n + 2) {
                #[allow(clippy::cast_precision_loss)]
                let x = x as f32;
                let x = x.mul_add(state.grid.size, center.x);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(x, rect.min.y), pos2(x, rect.max.y)],
                    stroke: grid_stroke.into(),
                });
            }

            for y in (-m)..(m + 2) {
                #[allow(clippy::cast_precision_loss)]
                let y = y as f32;
                let y = y.mul_add(state.grid.size, center.y);

                ui.painter().add(Shape::LineSegment {
                    points: [pos2(rect.min.x, y), pos2(rect.max.x, y)],
                    stroke: grid_stroke.into(),
                });
            }

            // Outline around the viewport
            ui.painter().add(RectShape::stroke(
                rect,
                Rounding::ZERO,
                (1.0, grid_stroke.color),
            ));
        }

        /* ---- */

        let mut graph_ui = GraphUi {
            ui,
            graph_id: id,
            dragged_node: state.dragged_node,
            viewport,
            rendered_sockets: Collector::new(),
        };

        build_fn(&mut graph_ui);

        let GraphUi {
            graph_id: _,
            dragged_node,
            viewport,
            ui,
            rendered_sockets,
        } = graph_ui;

        state.dragged_node = dragged_node;

        let sockets = rendered_sockets.into_vec();

        let socket_interaction = if can_connect_socket {
            crate::socket::handle_socket_responses(&mut state.dragged_socket, &sockets)
        } else {
            // Stop the currently dragged socket if creating connection is disabled.
            state.dragged_socket = None;
            crate::socket::SocketInteraction::None
        };

        /* ---- */

        GraphEditor {
            id,
            stage: stages::Connections {
                ui,
                state,
                viewport,
                response,
                sockets,
                socket_interaction,
            },
        }
    }
}

/* -------------------------------------------------------------------------- */
