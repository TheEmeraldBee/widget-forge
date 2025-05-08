use crate::prelude::*;

pub trait WidgetSet<D> {
    fn insert(self, scene: &mut Scene<D>);
}

macro_rules! widget_set {
    ($($name:tt $num:tt)+) => {
        impl<Z: 'static, $($name: Widget<Z> + 'static),+> WidgetSet<Z> for ($($name,)+) {
            fn insert(self, scene: &mut Scene<Z>) {
                $(scene.insert_widget(self.$num);)+
            }
        }
    }
}

widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8  }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7  }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 G 6 }
widget_set! { A 0 B 1 C 2 D 3 E 4 F 5 }
widget_set! { A 0 B 1 C 2 D 3 E 4 }
widget_set! { A 0 B 1 C 2 D 3 }
widget_set! { A 0 B 1 C 2 }
widget_set! { A 0 B 1 }
widget_set! { A 0 }
