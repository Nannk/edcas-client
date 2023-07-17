use std::ops::Add;
use eframe::egui;
use eframe::egui::TextureHandle;
use eframe::emath::Numeric;
use crate::app::explorer::structs::{BodyImplementation, Signal};
use crate::ICON_BODY_SIGNAL;

pub struct System {
    pub name: String,
    pub allegiance: String,
    pub economy_localised: String,
    pub second_economy_localised: String,
    pub government_localised: String,
    pub security_localised: String,
    pub population: String,
    pub body_count: String,
    pub non_body_count: String,
    pub signal_list: Vec<SystemSignal>,
    pub body_list: Vec<Box<dyn BodyImplementation>>,
    pub planet_signals: Vec<PlanetSignals>,
    pub index: usize,
}

pub struct PlanetSignals{
    pub body_name: String,
    pub body_id: i64,
    pub signals: Vec<Signal>
}

impl PartialEq for PlanetSignals {
    fn eq(&self, other: &Self) -> bool {
        self.body_id == other.body_id
    }
}

impl System{
    pub fn draw_body_signal_list(&self, ui: &mut egui::Ui) {
        egui::Grid::new("body_signal_grid")
            .num_columns(3)
            .striped(true)
            .min_col_width(130.0)
            .show(ui, |ui| {
                ui.label("Body");
                ui.label("Type");
                ui.label("Count");
                ui.end_row();

                for body_signal in &self.planet_signals{
                    for signal in &body_signal.signals{
                        ui.label(body_signal.body_name.trim_start_matches(&self.name));
                        if &signal.type_localised == "null"{
                            ui.label(&signal.r#type);
                        }else {
                            ui.label(&signal.type_localised);
                        }

                        let id = body_signal.body_name.clone().add(&signal.r#type.to_string().clone());

                        egui::Grid::new(id)
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label(signal.count.to_string());
                                let texture: TextureHandle = ui.ctx().load_texture(
                                    "body-signal-icon",
                                    ICON_BODY_SIGNAL.lock().unwrap().get_planet_signal_icon_from_string(signal.r#type.clone()).clone(),
                                    egui::TextureOptions::LINEAR,
                                );

                                let img_size = 32.0 * texture.size_vec2() / texture.size_vec2().y;
                                ui.image(&texture, img_size);
                            });
                        ui.end_row();
                    }
                }
            });
    }

    fn draw_system_details(&self, ui: &mut egui::Ui) {
        ui.label("Allegiance");
        ui.label(&self.allegiance);
        ui.end_row();

        ui.label("Economy");
        ui.label(&self.economy_localised);
        ui.end_row();

        ui.label("sec. Economy");
        ui.label(&self.second_economy_localised);
        ui.end_row();

        ui.label("Government");
        ui.label(&self.government_localised);
        ui.end_row();

        ui.label("Security");
        ui.label(&self.security_localised);
        ui.end_row();

        ui.label("Population");
        ui.label(&self.population);
        ui.end_row();
    }

    pub fn draw_system_info(&self, ui: &mut egui::Ui) {
        egui::Grid::new("system_data_grid")
            .num_columns(2)
            .striped(true)
            .min_col_width(200.0)
            .show(ui, |ui| {
                self.draw_system_details(ui);
            });

        ui.separator();
        egui::Grid::new("body_count_grid")
            .num_columns(2)
            .striped(true)
            .min_col_width(200.0)
            .show(ui, |ui| {
                ui.label("Bodies");
                ui.label(&self.body_count);
                ui.end_row();
                ui.label("Non-bodies");
                ui.label(&self.non_body_count);
                ui.end_row();
            });


        if !self.body_count.eq("N/A") {
            ui.add(egui::ProgressBar::new((&self.body_list.len().to_f64() / (&self.body_count.parse::<f64>().unwrap() + &self.non_body_count.parse::<f64>().unwrap())) as f32)
                .text(&self.body_list.len().to_string().add("/").add((&self.body_count.parse::<f64>().unwrap()+&self.non_body_count.parse::<f64>().unwrap()).to_string().as_str()))
            );
        }
        ui.end_row();
        ui.separator();
        ui.heading("System Signals");
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                self.draw_system_signal_list(ui);
            });
    }


    fn draw_system_signal_list(&self, ui: &mut egui::Ui) {

        egui::Grid::new("system_signal_grid")
            .num_columns(2)
            .striped(true)
            .min_col_width(130.0)
            .show(ui, |ui| {
                ui.label("Name");
                ui.label("Thread");
                ui.end_row();
                for system_signal in &self.signal_list{
                    ui.label(&system_signal.name);
                    ui.label(&system_signal.thread);

                    ui.end_row();
                }
            });
    }

    pub fn insert_body(&mut self, body: Box<dyn BodyImplementation>) -> usize{
        let id = body.get_id().clone();
        self.body_list.push(body);

        self.body_list.sort_by(|body_a, body_b| {
            body_a.get_id().cmp(&body_b.get_id())
        });

        for i in 0..self.body_list.len(){
            if id == self.body_list.get(i).unwrap().get_id(){
                return i;
            }
        }
        return 0;
    }
}


#[derive(Clone)]
pub struct SystemSignal{
    pub timestamp: String,
    pub event: String,
    pub name: String,
    pub thread: String
}