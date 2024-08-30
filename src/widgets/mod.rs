use enum_dispatch::enum_dispatch;

pub use rectangle::Rectangle;
use window::WindowRef;

pub mod widget_builder;
pub mod window;
pub mod rectangle;

#[enum_dispatch(WidgetTrait)]
#[derive(Debug)]
pub enum Widget {
    Window(WindowRef),
    Rectangle(Rectangle)
}


#[enum_dispatch]
pub trait WidgetTrait {
    fn expected_children(&self) -> usize { 1 }
}