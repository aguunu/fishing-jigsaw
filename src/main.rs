#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 724.0])
            .with_maximize_button(false)
            .with_resizable(false),
            // .with_icon(
            //     eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-32.png")[..])
            //         .expect("Failed to load icon"),
            // ),
            ..Default::default()
    };
    eframe::run_native(
        "Fishing Jigsaw",
        options,
        Box::new(|cc| Ok(Box::new(fishing_jigsaw::App::new(cc)))),
    )
}

