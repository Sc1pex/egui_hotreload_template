use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct State {
    x: i32,
    y: String,
}

impl State {
    pub fn new(storage: &mut dyn eframe::Storage) -> Self {
        eframe::get_value(storage, "dyn_lib_state").unwrap_or_default()
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "dyn_lib_state", self);
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(format!("x is {}", self.x));

                    if ui.button("Inc").clicked() {
                        self.x += 1;
                    }
                    if ui.button("Dec").clicked() {
                        self.x -= 1;
                    }
                });
                ui.vertical(|ui| {
                    ui.heading(format!("y is {}", self.y));

                    if ui.button("A").clicked() {
                        self.y = "A".into();
                    }
                    if ui.button("B").clicked() {
                        self.y = "C".into();
                        // self.y -= 1;
                    }
                });
            });
        });
    }
}
