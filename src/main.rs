use data::AppState;
use druid::{WindowDesc, AppLauncher};
use im::Vector;
use saver::read_stored;
use ui::ui_builder;

mod ui;
mod data;
mod saver;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("PnP App")
        .window_size((400.0, 400.0));

    let stored_data = read_stored();
    let default_state = AppState {
        characters: Vector::from(stored_data.characters),
        ..Default::default()
    };

    AppLauncher::with_window(main_window)
        .launch(default_state)
        .expect("Failed to launch application");
}
