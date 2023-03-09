use druid::{Widget};
use druid::widget::{Label, Flex};

use crate::data::AppState;

pub fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
}