use druid::{Widget, WidgetExt, Env, EventCtx, Menu, MenuItem, Point, Event, Code, Color};
use druid::widget::{Label, Flex, TextBox, Button, List, Padding, Controller, Tabs, Axis, TabsEdge, TabsTransition};

use crate::data::{AppState, Character};
use crate::saver::Saver;

pub fn ui_builder() -> impl Widget<AppState> {
    let character_creation = Flex::column()
        .with_child(Padding::new(5.0, Label::new("Character Creation")))
        .with_child(Flex::row()
            .with_child(Flex::column()
                .with_child(Flex::row()
                    .with_flex_spacer(0.1)
                    .with_child(Padding::new(5.0, Label::new("Name: ")))
                    .with_child(TextBox::new().lens(AppState::chargen_name).controller(Enter {}).fix_width(200.0))
                    .padding(1.0)
                )
                .with_child(Flex::row()
                    .with_flex_spacer(0.1)
                    .with_child(Padding::new(5.0, Label::new("Age: ")))
                    .with_child(TextBox::new().lens(AppState::chargen_age).controller(Enter {}).fix_width(200.0))
                    .padding(1.0)
                )
                .with_child(Flex::row()
                    .with_flex_spacer(0.1)
                    .with_child(Padding::new(5.0, Label::new("Sex: ")))
                    .with_child(TextBox::new().lens(AppState::chargen_sex).controller(Enter {}).fix_width(200.0))
                    .padding(1.0)
                )
                .with_child(Flex::row()
                    .with_flex_spacer(0.1)
                    .with_child(Padding::new(5.0, Label::new("Race: ")))
                    .with_child(TextBox::new().lens(AppState::chargen_race).controller(Enter {}).fix_width(200.0))
                    .padding(1.0)
                )
                .with_child(Flex::row()
                    .with_flex_spacer(0.1)
                    .with_child(Padding::new(5.0, Label::new("Occupation: ")))
                    .with_child(TextBox::new().lens(AppState::chargen_occupation).controller(Enter {}).fix_width(200.0))
                    .padding(1.0)
                ).fix_width(300.0)
            )
            .with_child(Padding::new(5.0, Button::new("->")
                .on_click(|_ctx, data: &mut AppState, _env| {

                    // !!! REFACTOR INTO input_checker() !!!

                    if data.chargen_name.trim() != "" && data.chargen_age.trim() != "" && data.chargen_sex.trim() != "" && data.chargen_race.trim() != "" && data.chargen_occupation.trim() != "" {
                        let name = data.chargen_name.clone();
                        let age: u32 = data.chargen_age.clone().parse().unwrap();
                        let sex = data.chargen_sex.clone();
                        let race = data.chargen_race.clone();
                        let occupation = data.chargen_occupation.clone();
    
                        // !!! ERROR HANDLING !!!
    
                        // let age: u32 = data.input_character_age.clone().parse().unwrap();
    
                        data.chargen_name = "".to_string();
                        data.chargen_age = "".to_string();
                        data.chargen_sex = "".to_string();
                        data.chargen_race = "".to_string();
                        data.chargen_occupation = "".to_string();
    
                        data.characters.push_back(Character {
                            name,
                            age,
                            sex,
                            race,
                            occupation,
                            action: 15,
                            knowledge: 15,
                            social: 10,
                            health: 100,
                        });
                    }
                }).fix_height(50.0)
            ))
            .with_child(Saver {})
        );

    let character_list = List::new(|| {
        let bg = Color::rgba8(0, 0, 0, 50);
        Flex::row()
            .with_child(Flex::column()
                .with_child(Flex::row()
                    .with_child(Label::new(|data: &Character, _: &Env| data.name.clone() ).with_text_color(color_picker("lime_green")))
                    .with_child(Label::new(|data: &Character, _: &Env| data.sex.to_string() ))
                    .with_child(Label::new(|data: &Character, _: &Env| data.age.to_string() ))
                    .with_flex_spacer(0.1)
                )
                .with_child(Flex::row()
                    .with_child(Label::new(|data: &Character, _: &Env| data.race.clone()))
                    .with_child(Label::new(|data: &Character, _: &Env| data.occupation.clone()))
                    .with_flex_spacer(0.1)
                )
                .with_child(Flex::row()
                    .with_child(Label::new(|data: &Character, _: &Env| data.action.to_string() ))
                    .with_child(Label::new(|data: &Character, _: &Env| data.knowledge.to_string() ))
                    .with_child(Label::new(|data: &Character, _: &Env| data.social.to_string() ))
                    .with_flex_spacer(0.1)
                ).fix_width(350.0).padding(5.0)
            ).with_flex_spacer(0.1)
            .with_child(Button::new("...").on_click(|ctx: &mut EventCtx, data: &mut Character, _env| {
                let data_clone = data.clone();
                let menu: Menu<AppState> = Menu::empty()
                    .entry(MenuItem::new("Remove").on_activate(move |_ctx, main_data: &mut AppState, _env| {
                        let location = main_data.characters.index_of(&data_clone).unwrap();
                        main_data.characters.remove(location);
                    }));
                ctx.show_context_menu(menu, Point::new(0.0, 0.0))
            })).fix_width(400.0).background(bg).padding(5.0)
    }).lens(AppState::characters);

    let characters_tabs = Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .with_tab("View", Flex::column().with_flex_child(character_list, 1.0))
        .with_tab("New", character_creation)
        .with_tab_index(0);

    Tabs::new()
        .with_axis(Axis::Vertical)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .with_tab("Characters", characters_tabs)
        .with_tab("Abilities", Label::new("Abilities"))
        .with_tab("Items", Label::new("Items"))
        .with_tab_index(0)
    
}


// !!! IMPLEMENT GENERICS for Enter!!!

struct Enter;

impl<W: Widget<AppState>> Controller<AppState, W> for Enter {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &druid::Event, data: &mut AppState, env: &Env) {
        if let Event::KeyUp(key) = event {
            if key.code == Code::Enter {

                // !!! REFACTOR INTO input_checker() !!!

                if data.chargen_name.trim() != "" && data.chargen_age.trim() != "" && data.chargen_sex.trim() != "" && data.chargen_race.trim() != "" && data.chargen_occupation.trim() != "" {
                    let name = data.chargen_name.clone();
                    let age: u32 = data.chargen_age.clone().parse().unwrap();
                    let sex = data.chargen_sex.clone();
                    let race = data.chargen_race.clone();
                    let occupation = data.chargen_occupation.clone();

                    // !!! ERROR HANDLING !!!

                    // let age: u32 = data.input_character_age.clone().parse().unwrap();

                    data.chargen_name = "".to_string();
                    data.chargen_age = "".to_string();
                    data.chargen_sex = "".to_string();
                    data.chargen_race = "".to_string();
                    data.chargen_occupation = "".to_string();

                    data.characters.push_back(Character {
                        name,
                        age,
                        sex,
                        race,
                        occupation,
                        action: 15,
                        knowledge: 15,
                        social: 10,
                        health: 100,
                    });
                }
            }
        }

        child.event(ctx, event, data, env)
    }

    fn lifecycle(
        &mut self,
        child: &mut W,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &AppState,
        env: &Env,
    ) {
        child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        child.update(ctx, old_data, data, env)
    }
}

pub fn color_picker(color: &str) -> Color {
    match color {
        "dark_blue" => Color::rgba8(15, 17, 26, 100),
        "lime_green" => Color::rgba8(90, 175, 75, 100),
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        "yellow" => Color::YELLOW,
        "black" => Color::BLACK,
        "white" => Color::WHITE,
        "grey" => Color::GRAY,
        "purple" => Color::PURPLE,
        _ => Color::BLACK,
    }
}