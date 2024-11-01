use enum_dispatch::enum_dispatch;

use lumi2d::Object;

use crate::signals::SignalRef;

pub mod widget_builder;
pub mod root;
pub mod rectangle;
pub mod text;

#[enum_dispatch(WidgetTrait)]
#[derive(Debug)]
pub enum Widget {
    Rectangle(rectangle::Rectangle),
    Text(text::Text)
}


#[enum_dispatch]
pub trait WidgetTrait {
    fn expected_children(&self) -> usize { 1 }
    fn get_objects(&self) -> SignalRef<Object>;
}