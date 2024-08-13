// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
// #![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

use std::fs::File;
use std::io::{Write};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                    .with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    // Our application state:
    let mut output_path = "./output";
    let mut input_path = "https://example.com".to_owned();
    let mut res_text = "".to_owned();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.hyperlink("https://docs.rs/egui/latest/egui/");

            ui.separator();

            ui.label("URL");
            ui.text_edit_singleline(&mut input_path);

            ui.label("Response");

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.text_edit_multiline(&mut res_text);
                
                if ui.button("GET").clicked() {
                    let url = "https://example.com";
                    let res = reqwest::blocking::get(url).unwrap();
                    res_text = res.text().unwrap();
                }
            });
        });
    })
}

// fn input_file() {
//     let mut f = File::open("test.txt").expect("file not found");
// }

// fn output_file() -> std::io::Result<()> {
//     let mut file = File::create("test.txt")?;
//     file.write_all("test".as_bytes())?;
//     println!("completed.");
//     Ok(())
// }
