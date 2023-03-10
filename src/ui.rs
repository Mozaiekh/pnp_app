use druid::{Widget, WidgetExt, Env, EventCtx, Menu, MenuItem, Point, Event, Code, Color};
use druid::widget::{Label, Flex, TextBox, Button, List, Padding, Controller, Tabs, Axis, TabsEdge, TabsTransition};

use crate::data::{AppState, Character};
use crate::saver::Saver;

pub fn ui_builder() -> impl Widget<AppState> {
    let character_creation = Flex::column()
        .with_child(Padding::new(5.0, Label::new("Character Creation")))
        .with_child(Flex::row()
            .with_child(Padding::new(5.0, Label::new("Name: ")))
            .with_flex_child(TextBox::new().lens(AppState::chargen_name).expand_width().controller(Enter {}), 1.0)
            .with_child(Padding::new(5.0, Button::new("->")
                .on_click(|_ctx, data: &mut AppState, _env| {

                    // !!! REFACTOR INTO input_checker() !!!

                    if data.chargen_name.trim() != "" {
                        let name = data.chargen_name.clone();

                        // !!! ERROR HANDLING !!!

                        // let age: u32 = data.input_character_age.clone().parse().unwrap();

                        data.chargen_name = "".to_string();
                        data.characters.push_back(Character {name, age: 20, health: 100});
                    }
                })
            ))
            .with_child(Saver {})
        );

    let character_list = Flex::column()
        .with_child(Padding::new(5.0, Label::new("Character List")))
        .with_child(List::new(|| {
            let bg = Color::rgba8(0, 0, 0, 50);
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
                })).background(bg)
        }).lens(AppState::characters)
    );

    let characters_tabs = Tabs::new()
        .with_axis(Axis::Horizontal)
        .with_edge(TabsEdge::Leading)
        .with_transition(TabsTransition::Instant)
        .with_tab("View", character_list)
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

                if data.chargen_name.trim() != "" {
                    let name = data.chargen_name.clone();

                    // !!! ERROR HANDLING !!!

                    // let age: u32 = data.input_character_age.clone().parse().unwrap();

                    data.chargen_name = "".to_string();
                    data.characters.push_back(Character {name, age: 20, health: 100});
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