use crate::{backend::Backend, elements::window::Window};

use super::{widget_builder::WidgetBuilderTrait, Widget};

#[derive(Debug)]
pub struct RootBuilder();

impl WidgetBuilderTrait for RootBuilder {
    fn build(self, _: &Backend, _: Option<&Window>) -> Widget {
        unreachable!();
    }
}