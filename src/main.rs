use data::AppState;
use druid::{WindowDesc, AppLauncher, Color};
use druid::theme::{BUTTON_DARK, BUTTON_LIGHT, WINDOW_BACKGROUND_COLOR};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod ui;
mod data;
mod saver;

const _SCREEN_SMALL: (f64, f64) = (480.0, 854.0);
const SCREEN_MEDIUM: (f64, f64) = (960.0, 540.0);
const _SCREEN_LARGE: (f64, f64) = (1366.0, 768.0);

fn main() {

    let main_window = WindowDesc::new(ui_builder())
        .title("PnP App")
        .window_size(SCREEN_MEDIUM);

    let stored_data = read_stored();
    let default_state = AppState {
        characters: Vector::from(stored_data.characters),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .configure_env(|env, _state| {
            env.set(BUTTON_DARK, Color::rgba8(100, 100, 120, 0));
            env.set(BUTTON_LIGHT, Color::rgba8(100, 100, 100, 100));
            env.set(WINDOW_BACKGROUND_COLOR, Color::rgba8(15, 17, 26, 0))
        })
        .launch(default_state)
        .expect("Failed to launch application");
}
