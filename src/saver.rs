use std::{path::Path, fs};

use directories::BaseDirs;
use druid::Widget;
use serde::{Deserialize, Serialize};

use crate::data::{AppState, Character};

pub struct Saver;

impl Widget<AppState> for Saver {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &druid::Env) {
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &druid::Env) {
    }

    fn update(&mut self, _ctx: &mut druid::UpdateCtx, old_data: &AppState, data: &AppState, _env: &druid::Env) {
        if data.characters != old_data.characters {
            if let Some(base_dir) = BaseDirs::new() {
                let config = format!("{}/{}", base_dir.config_dir().to_str().unwrap(), "pnp_data.json");
                let config_path = Path::new(&config);
                let characters = AppData { characters: data.characters.clone().into_iter().collect() };
                fs::write(config_path, serde_json::to_string(&characters).unwrap()).expect("Bad config path");
            }
        }
    }

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, _bc: &druid::BoxConstraints, _data: &AppState, _env: &druid::Env) -> druid::Size {
        druid::Size { width: 0.0, height: 0.0 }
    }

    fn paint(&mut self, _ctx: &mut druid::PaintCtx, _data: &AppState, _env: &druid::Env) {
    }
}

#[derive(Serialize, Deserialize)]
pub struct AppData {
    pub characters: Vec<Character>,
}

pub fn read_stored() -> AppData {
    if let Some(base_dir) = BaseDirs::new() {
        let config = format!("{}/{}", base_dir.config_dir().to_str().unwrap(), "pnp_data.json");
        let config_path = Path::new(&config);
        let saved_data = match fs::read_to_string(config_path) {
            Ok(a) => a,
            Err(_) => {
                return AppData { characters: Vec::new(), }
            },
        };
        match serde_json::from_str(&saved_data) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Saved data is corrupt or in the wrong format\nError: {}", e);
                return AppData { characters: Vec::new() };
            },
        }
    } else {
        AppData { characters: Vec::new() }
    }
}