pub(crate) use self::calculation::Pane;

use egui::{Vec2, vec2};

const MARGIN: Vec2 = vec2(4.0, 2.0);

pub(crate) mod behavior;
pub(crate) mod calculation;
