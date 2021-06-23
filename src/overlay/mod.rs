use crate::generation::{GenerationalIndex, GenerationalIndexAllocator, GenerationalIndexVec};

use self::brush::Brush;

mod brush;

pub type WidgetId = GenerationalIndex;
pub type WidgetVec<T> = GenerationalIndexVec<T>;

pub trait Widget {
    fn draw(&mut self, brush: &mut Brush);
}

pub struct Overlay {
    allocator: GenerationalIndexAllocator,
    widgets: WidgetVec<Box<dyn Widget>>,
}

impl Overlay {
    fn draw(&mut self, brush: &mut Brush) {
        for widget in self.allocator.iter() {
            if let Some(widget) = self.widgets.get_mut(widget) {
                widget.draw(brush);
            };
        }
    }
}

pub struct Label<'txt> {
    position: [f32; 2],
    bounds: [f32; 2],
    color: [f32; 4],
    scale: f32,
    text: &'txt str,
}

impl<'txt> Widget for Label<'txt> {
    fn draw(&mut self, brush: &mut Brush) {
        todo!()
    }
}

// TODO label
// TODO text field
