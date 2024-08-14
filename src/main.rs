use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                    .with_inner_size([320.0, 200.0]),
        ..Default::default()
    };

    let mut url_text = "https://example.com".to_owned();
    let mut res_text = "".to_owned();

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.hyperlink("https://docs.rs/egui/latest/egui/");

            ui.separator();

            ui.label("URL");
            ui.text_edit_singleline(&mut url_text);

            ui.label("Response");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.text_edit_multiline(&mut res_text);
                
                if ui.button("GET").clicked() {
                    let res = reqwest::blocking::get(&url_text).unwrap();
                    res_text = res.text().unwrap();
                }
            });
        });
    })
}
