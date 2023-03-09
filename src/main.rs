use druid::{WindowDesc, AppLauncher};
use ui::ui_builder;

mod ui;
mod data;

fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title("PnP App")
        .window_size((400.0, 400.0));
    AppLauncher::with_window(main_window)
        .launch(data::AppState::default())
        .expect("Failed to launch application");
}
