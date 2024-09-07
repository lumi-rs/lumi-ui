use enum_dispatch::enum_dispatch;

use lumi2d::Objects;
pub use rectangle::Rectangle;

pub mod widget_builder;
pub mod root;
pub mod rectangle;

#[enum_dispatch(WidgetTrait)]
#[derive(Debug)]
pub enum Widget {
    Rectangle(Rectangle)
}


#[enum_dispatch]
pub trait WidgetTrait {
    fn expected_children(&self) -> usize { 1 }
    fn get_objects(&self) -> &Objects;
}