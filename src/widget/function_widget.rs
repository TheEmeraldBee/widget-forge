use std::collections::HashSet;

use ascii_forge::window::Window;

use super::{SpecialWidget, Widget};

impl<D, F: Fn(&mut Window, &mut D)> Widget<D> for F {
    fn update(&mut self, window: &mut Window, data: &mut D) {
        self(window, data)
    }
}

impl<D, F: Fn(&mut Window, &mut D, &mut [Box<dyn Widget<D>>], &mut HashSet<usize>)> SpecialWidget<D>
    for F
{
    fn update(
        &mut self,
        window: &mut Window,
        data: &mut D,
        widgets: &mut [Box<dyn Widget<D>>],
        updated: &mut HashSet<usize>,
    ) {
        self(window, data, widgets, updated)
    }
}
