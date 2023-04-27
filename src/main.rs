#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    let options = {
        let mut options = eframe::NativeOptions::default();
        options.initial_window_size = Some((1024.0, 768.0).into());
        options
    };

    eframe::run_native(
        "Fishing Jigsaw",
        options,
        Box::new(|cc| Box::new(fishing_jigsaw::App::new(cc))),
    )
}
