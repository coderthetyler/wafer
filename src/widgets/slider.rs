use crate::types::{Extent, Point};

use super::Widget;

pub struct Slider {
    pub location: Point,
    pub extent: Extent,
    pub progress: f32,

    /// Indicates if user is currently dragging the slider knob.
    pub is_sliding: bool,
    /// Progress prior to ongoing slide event.
    pub progress_origin: f32,
}

impl Slider {}

impl Default for Slider {
    fn default() -> Self {
        Self {
            location: Point { x: 10.0, y: 100.0 },
            extent: Extent {
                width: 400.0,
                height: 50.0,
            },
            progress: 0.2,

            is_sliding: false,
            progress_origin: -1.0,
        }
    }
}

impl From<Slider> for Widget {
    fn from(slider: Slider) -> Widget {
        Widget::Slider(slider)
    }
}
