use crate::{prelude::*, widget::conditional_widgets::ConditionalWidgets};

use ascii_forge::prelude::*;

use std::{collections::HashSet, io, time::Duration};

pub struct Scene<D> {
    window: Window,
    data: D,
    widgets: Vec<Box<dyn Widget<D>>>,
    special_widgets: Vec<Box<dyn SpecialWidget<D>>>,
}

impl<D: 'static> Scene<D> {
    pub fn new(window: Window, data: D) -> Self {
        Self {
            window,
            data,
            widgets: vec![],
            special_widgets: vec![],
        }
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }

    pub fn insert_widget(&mut self, widget: impl Widget<D> + 'static) -> &mut Self {
        self.widgets.push(Box::new(widget));
        self
    }

    pub fn insert_widgets(&mut self, set: impl WidgetSet<D>) -> &mut Self {
        set.insert(self);
        self
    }

    pub fn insert_special_widget(&mut self, widget: impl SpecialWidget<D> + 'static) -> &mut Self {
        self.special_widgets.push(Box::new(widget));
        self
    }

    pub fn insert_conditional_widgets(
        &mut self,
        cond: impl Fn(&mut Window, &mut D) -> bool + 'static,
        widgets: impl WidgetSet<D>,
    ) -> &mut Self {
        let start = self.widgets.len();
        widgets.insert(self);
        let end = self.widgets.len();

        self.insert_special_widget(ConditionalWidgets::new(cond, (start..end).collect()));
        self
    }

    pub fn take_all(self) -> (Window, D) {
        (self.window, self.data)
    }

    pub fn update(&mut self) {
        let mut updated = HashSet::default();
        for special in &mut self.special_widgets {
            special.update(
                &mut self.window,
                &mut self.data,
                &mut self.widgets,
                &mut updated,
            );
        }
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if updated.contains(&i) {
                continue;
            }

            widget.update(&mut self.window, &mut self.data);
        }
    }

    pub fn run(
        &mut self,
        update_millis: u64,
        run_cond: impl Fn(&mut Self) -> bool,
    ) -> io::Result<&mut Self> {
        while run_cond(self) {
            self.update();

            self.window.update(Duration::from_millis(update_millis))?;
        }
        Ok(self)
    }
}
