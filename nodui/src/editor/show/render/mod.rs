//! Node rendering.

pub(super) mod body;
pub(super) mod header;
pub(super) mod socket;

/// The width of the socket's handle.
const SOCKET_WIDTH: f32 = 10.0;

/// Space between socket's name its socket shape.
const SOCKET_NAME_GAP: f32 = 5.0;

/// Space between the two columns of a node.
const DOUBLE_COLUMNS_GAP: f32 = 5.0;

// TODO: use ui.spacing instead
/// The vertical space between each socket.
const SOCKET_VERTICAL_GAP: f32 = 5.0;
