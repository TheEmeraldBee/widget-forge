use std::collections::HashSet;

use ascii_forge::window::Window;

pub mod conditional_widgets;
pub mod function_widget;
pub mod widget_set;

pub trait Widget<D> {
    fn update(&mut self, window: &mut Window, data: &mut D);
}

pub trait SpecialWidget<D> {
    fn update(
        &mut self,
        window: &mut Window,
        data: &mut D,
        widgets: &mut [Box<dyn Widget<D>>],
        updated: &mut HashSet<usize>,
    );
}
