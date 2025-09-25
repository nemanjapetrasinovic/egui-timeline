#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] use egui::Color32;
// hide console window on Windows in release
use egui_timeline::Timeline;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Timeline hello world",
        options,
        Box::new(|_cc| {
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    seek_position: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            seek_position: 0.0
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Timeline example");

            let timeline = Timeline::new(10.0, 100.0, &mut self.seek_position);
            let timeline_add = ui.add(timeline);
            if timeline_add.clicked() || timeline_add.drag_stopped() {
                println!("Seek updated");
            }

            ui.label(format!("Seek position {}", self.seek_position));

            let mut seek_position_other: f64 = 0.0;
            let timeline2 = Timeline::new(250.0, 1000.0, &mut seek_position_other)
                .accent_color(Color32::from_rgb(15, 50, 140))
                .font_size(14.0)
                .radius_factor(0.5);

            let timeline_add2 = ui.add(timeline2);
            if timeline_add2.clicked() || timeline_add2.drag_stopped() {
                println!("Seek updated");
            }

        });
    }
}
