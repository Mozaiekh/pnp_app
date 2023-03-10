use druid::{Widget, WidgetExt, Env};
use druid::widget::{Label, Flex, TextBox, Button, List};

use crate::data::{AppState, Character};

pub fn ui_builder() -> impl Widget<AppState> {
    let character_creation = Flex::column()
        .with_child(Label::new("Character Creation"))
        .with_child(Flex::row()
            .with_flex_child(TextBox::new().lens(AppState::new_character).expand_width(), 1.0)
            .with_child(Button::new("->")
                .on_click(|_ctx, data: &mut AppState, _env| {
                    if data.new_character.trim() != "" {
                        let name = data.new_character.clone();
                        data.new_character = "".to_string();
                        data.characters.push_back(Character {name: name.clone(), age: 20, health: 100});
                        println!("Character {} created", name)
                    }
                })
            )
    );

    let character_list = Flex::column()
        .with_child(List::new(|| {
            Flex::row()
                .with_child(Label::new(|data: &Character, _: &Env| data.name.clone() ))
                .with_child(Label::new(|data: &Character, _: &Env| data.age.to_string() ))
                .with_child(Label::new(|data: &Character, _: &Env| data.health.to_string() ))
        }).lens(AppState::characters)
    );

    Flex::column()
        .with_child(character_creation)
        .with_child(character_list)
}