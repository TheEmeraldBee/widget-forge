/**
This is a super basic implementation of a simple widgetui application
All this does is render hello world with the 'hello' widget, then
handles a press of the 'q' key to quit the application
*/
use ascii_forge::prelude::*;
use std::io;
use widgetui::prelude::*;

#[derive(Default)]
pub struct AppData {
    should_exit: bool,
}

fn hello(w: &mut Window, _d: &mut AppData) {
    let center = {
        let s = w.size();
        vec2(s.x / 2, s.y / 2)
    };

    render!(w, (center.x - 13/2, center.y) => [ "Hello, World!".green() ]);
}

fn quit_handler(w: &mut Window, d: &mut AppData) {
    if event!(w, Event::Key(e) => e.code == KeyCode::Char('q')) {
        d.should_exit = true;
    }
}

fn main() -> io::Result<()> {
    let window = Window::init()?;
    handle_panics();
    Scene::new(window, AppData::default())
        // Insert the 2 widgets to run (hello will run first, then quit_handler)
        .insert_widgets((hello, quit_handler))
        // Run the application with a poll time of 100ms and a exit condition where AppData::should_exit is true
        .run(100, |scene| !scene.data().should_exit)?;
    Ok(())
}
