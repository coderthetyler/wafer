mod slider;

pub use slider::Slider;

pub enum Widget {
    Slider(Slider),
}
