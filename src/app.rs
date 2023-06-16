use crate::{
    mcts::{Config, Environment},
    CustomWindow, JigsawManager,
};

use std::ops::RangeInclusive;

pub struct App {
    search_config: Config,
    jigsaw_manager: JigsawManager,
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl App {
    pub fn game_controller(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_enabled(!self.jigsaw_manager.is_computing());

            if ui.button("New Game").clicked() {
                self.jigsaw_manager.reset();
            }

            if ui.button("Perform Action").clicked() {
                if let Some(action) = self.jigsaw_manager.optimal_action() {
                    self.jigsaw_manager.perform(action);
                }
            }

            if ui.button("Compute").clicked() {
                // Perform tree search in a new thread.
                self.jigsaw_manager.compute(&self.search_config);
            }
        });

        ui.add(egui::ProgressBar::new(self.jigsaw_manager.progress()));
    }

    fn adjust_parameters(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("algorithm-config-grid")
            .striped(true)
            .show(ui, |ui| {
                ui.set_enabled(!self.jigsaw_manager.is_computing());

                ui.label("Exploration Rate")
                    .on_hover_text("Algorithm exploration rate.");
                ui.add(egui::Slider::new(
                    &mut self.search_config.c,
                    RangeInclusive::new(0.0, 16.0),
                ));
                ui.end_row();

                ui.label("Iterations")
                    .on_hover_text("Algorithm iterations.");
                ui.add(egui::Slider::new(
                    &mut self.search_config.max_iters,
                    10_000..=800_000,
                ));
                ui.end_row();

                ui.label("Refresh Interval").on_hover_text(format!(
                    "Update stats every {} iterations.",
                    self.search_config.callback_interval
                ));
                ui.add(egui::Slider::new(
                    &mut self.search_config.callback_interval,
                    1_000..=self.search_config.max_iters,
                ));
                ui.end_row();

                ui.label("Max Depth")
                    .on_hover_text("Steps ahead computed by the algorithm.");
                ui.add(egui::Slider::new(&mut self.search_config.max_depth, 1..=16));
                ui.end_row();

                if ui.button("Reset").clicked() {
                    self.search_config = Config::default();
                }
                ui.end_row();
            });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            search_config: Config::default(),
            jigsaw_manager: JigsawManager::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.jigsaw_manager.is_computing() {
            // Request repaint if algorithm is running.
            ctx.request_repaint();
        }

        if self.jigsaw_manager.state.has_finished() {
            self.jigsaw_manager.reset();
        }

        egui::SidePanel::left("left-panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(egui::RichText::from("🔧 Adjust Parameters").heading());
                self.adjust_parameters(ui);

                ui.separator();
                ui.label(egui::RichText::from("🎮 Game Controller").heading());
                self.game_controller(ui);

                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        use egui::special_emojis::GITHUB;
                        ui.hyperlink_to(
                            format!("{} GitHub", GITHUB),
                            "https://github.com/aguunu/fishing-jigsaw",
                        );

                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.jigsaw_manager.show(ctx);
            self.jigsaw_manager.stats_manager.show(ctx);
        });
    }
}
