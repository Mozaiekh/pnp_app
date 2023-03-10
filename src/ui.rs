use druid::{Widget, WidgetExt, Env, EventCtx, Menu, MenuItem, Point};
use druid::widget::{Label, Flex, TextBox, Button, List};

use crate::data::{AppState, Character};

pub fn ui_builder() -> impl Widget<AppState> {
    let character_creation = Flex::column()
        .with_child(Label::new("Character Creation"))
        .with_child(Flex::row()
            .with_child(Label::new("Name: "))
            .with_flex_child(TextBox::new().lens(AppState::chargen_name).expand_width(), 1.0)
            .with_child(Button::new("->")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    if data.chargen_name.trim() != "" {
                        let name = data.chargen_name.clone();

                        // !!! ERROR HANDLING !!!

                        // let age: u32 = data.input_character_age.clone().parse().unwrap();

                        data.chargen_name = "".to_string();
                        data.characters.push_back(Character {name, age: 20, health: 100});
                    }
                })
            )
        );

    let character_list = Flex::column()
        .with_child(Label::new("Character List"))
        .with_child(List::new(|| {
            Flex::row()
                .with_child(Label::new(|data: &Character, _: &Env| data.name.clone() ))
                .with_child(Label::new(|data: &Character, _: &Env| data.age.to_string() ))
                .with_child(Label::new(|data: &Character, _: &Env| data.health.to_string() ))
                .with_flex_spacer(0.1)
                .with_child(Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut Character, _env| {
                    let data_clone = data.clone();
                    let menu: Menu<AppState> = Menu::empty()
                        .entry(MenuItem::new("Remove").on_activate(move |_ctx, main_data: &mut AppState, _env| {
                            let location = main_data.characters.index_of(&data_clone).unwrap();
                            main_data.characters.remove(location);
                        }));
                    ctx.show_context_menu(menu, Point::new(0.0, 0.0))
                }))
        }).lens(AppState::characters)
    );

    Flex::column()
        .with_child(character_creation)
        .with_child(character_list)
}