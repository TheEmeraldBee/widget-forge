use std::{collections::HashSet, marker::PhantomData};

use ascii_forge::window::Window;

use super::{SpecialWidget, Widget};

pub struct ConditionalWidgets<D, C: Fn(&mut Window, &mut D) -> bool>(C, Vec<usize>, PhantomData<D>);

impl<D, C: Fn(&mut Window, &mut D) -> bool> ConditionalWidgets<D, C> {
    pub fn new(cond: C, ids: Vec<usize>) -> Self {
        Self(cond, ids, PhantomData {})
    }

    pub fn with_widget_id(mut self, id: usize) -> Self {
        self.1.push(id);
        self
    }
}

impl<D, C: Fn(&mut Window, &mut D) -> bool> SpecialWidget<D> for ConditionalWidgets<D, C> {
    fn update(
        &mut self,
        window: &mut Window,
        data: &mut D,
        widgets: &mut [Box<dyn Widget<D>>],
        updated: &mut HashSet<usize>,
    ) {
        let cond = self.0(window, data);
        for idx in &self.1 {
            if updated.contains(idx) {
                continue;
            }

            if cond {
                widgets[*idx].update(window, data);
            }

            updated.insert(*idx);
        }
    }
}
