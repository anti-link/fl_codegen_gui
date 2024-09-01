#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] pub mod codegen;

// hide console window on Windows in release
use std::time::{SystemTime, UNIX_EPOCH};
use eframe::egui::{self, FontFamily};

use egui::FontFamily::Proportional;
use codegen::{generate_fl_code, remaining_time, float_from_time};
use egui_modal::Modal;
fn main() -> eframe::Result {
    let icon = include_bytes!("../Trollface.png");
    let image = image::load_from_memory(icon).expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = image.dimensions();


    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([620.0, 280.0]).with_icon(egui::IconData { 
            rgba: image.into_raw(), 
            width: icon_width, 
            height: icon_height,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "Family Link Codegen",
        options,
        
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    shared_secret: String,
    custom_timestamp: bool,
    selected_timestamp: String,
    validated_selected_timestamp: u64
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            shared_secret: "".to_owned(),
            custom_timestamp: false,
            selected_timestamp: "222".to_string(),
            validated_selected_timestamp: 0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let modal = Modal::new(ctx, "my_modal");
        
        modal.show(|ui| {
            // these helper functions help set the ui based on the modal's
            // set style, but they are not required and you can put whatever
            // ui you want inside [`.show()`]
            modal.title(ui, "Error");
            modal.frame(ui, |ui| {
                modal.body(ui, "Invalid Unix timestamp submitted.");
            });
        });

        if !self.custom_timestamp {
            self.selected_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("crazy errr").as_secs().to_string();
            self.validated_selected_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("crazy errr").as_secs();
        }

        let font_id = egui::FontId::new(48.0, FontFamily::Monospace);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Family Link Codegen");
            ui.horizontal(|ui| {
                ui.label("Shared secret: ");
                ui.add(egui::TextEdit::singleline(&mut self.shared_secret).hint_text("Paste your shared secret here"));
            });
            ui.add(egui::Checkbox::new(&mut self.custom_timestamp, "Generate for future time"));
            ui.add(egui::Hyperlink::from_label_and_url("How do I get a Unix timestamp?", "https://www.unixtimestamp.com/"));
            ui.horizontal(|ui| {
                ui.label("Unix timestamp: ");
                ui.add_enabled(self.custom_timestamp, egui::TextEdit::singleline(&mut self.selected_timestamp).hint_text("Enter a unix timestamp here"));
            });
            if ui.add_enabled(self.custom_timestamp,egui::Button::new("Submit")).clicked() {
                match &self.selected_timestamp {
                    num=>match num.parse::<u64>() {
                        Ok(num)=> {
                            self.validated_selected_timestamp = num
                        }
                        Err(_)=> {
                            modal.open()
                        }
                    },
                };
            }

            ui.label(
                egui::RichText::new(format!("{}", generate_fl_code(self.shared_secret.clone(), self.validated_selected_timestamp)))
                    .font(font_id)
            );

            ui.add(egui::ProgressBar::new(float_from_time(remaining_time(self.validated_selected_timestamp))).text(
                format!(
                    "Expires in: {:2}:{:02}",
                    remaining_time(self.validated_selected_timestamp) / 60,
                    remaining_time(self.validated_selected_timestamp) % 60,
                )
            ));


            egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url("Made by the amazing team at AntiLink", "https://github.com/anti-link"));
                    ui.add(egui::Hyperlink::from_label_and_url("Source code", "https://github.com/anti-link/fl_codegen_gui"));
             });

             ui.ctx().request_repaint();
        });
    }
}