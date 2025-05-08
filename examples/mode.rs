/**
This is an example implementing a simple mode-based keybinding input using
conditional widget rendering. This simple example waits for the user to push 'k'
and then shows that they are in the 'keybindings' mode. Then, when pressing 'q'
the program will exit. The user can also go back to the 'none' mode by pressing 'esc'
*/
use std::io;

use ascii_forge::prelude::*;
use ascii_forge::widgets::border::Border;
use widgetui::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    None,
    Keybinds,
}

pub struct Data {
    state: AppState,
    should_exit: bool,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            state: AppState::None,
            should_exit: false,
        }
    }
}

// This test will update the state to exit when the user pushes 'q'
fn quit_test(w: &mut Window, d: &mut Data) {
    if event!(w, Event::Key(e) => e.code == KeyCode::Char('q')) {
        d.should_exit = true;
    }
}

// This test will enter the user into the correct mode when pressing 'k'
fn keybinds_test(w: &mut Window, d: &mut Data) {
    if event!(w, Event::Key(e) => e.code == KeyCode::Char('k')) {
        d.state = AppState::Keybinds;
    }
}

// This test will take the user back to the 'none' mode if they press a key
fn back_test(w: &mut Window, d: &mut Data) {
    if event!(w, Event::Key(e) => e.code == KeyCode::Esc) {
        d.state = AppState::None;
    }
}

// This will render the state info to the screen
fn render_state(w: &mut Window, d: &mut Data) {
    let text = match d.state {
        AppState::None => "None".stylize(),
        AppState::Keybinds => "Keybinds".green(),
    };

    let center = vec2(w.size().x / 2, w.size().y / 2);

    render!(w,
        vec2(center.x - 12, center.y - 5) => [Border::square(24, 10)],
        vec2(center.x - text.content().len() as u16 / 2, center.y) => [text]
    );
}

pub fn main() -> io::Result<()> {
    let window = Window::init()?;
    handle_panics();
    Scene::new(window, Data::default())
        .insert_widget(render_state)
        .insert_conditional_widgets(|_w, d| d.state == AppState::None, (keybinds_test,))
        .insert_conditional_widgets(
            |_w, d| d.state == AppState::Keybinds,
            (quit_test, back_test),
        )
        .run(100, |scene| !scene.data_mut().should_exit)?;
    Ok(())
}
