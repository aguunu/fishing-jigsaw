use crate::mcts;
use crate::minigames::{Jigsaw, ALL_FIGURES};

use mcts::{Config, Environment, Stats};

use std::{
    collections::HashMap,
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

pub struct App {
    current_stats: Arc<Mutex<Option<Stats<Jigsaw>>>>,
    actions_history: Arc<Mutex<HashMap<u8, Vec<[f64; 2]>>>>,
    algorithm_config: Config,
    algorithm_handler: Option<std::thread::JoinHandle<()>>,
    game: Jigsaw,
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
    fn reset_stats(&mut self) {
        self.current_stats = Arc::default();
        self.actions_history = Arc::default();
    }

    fn best_action(&self) -> Option<u8> {
        let local_stats = self.current_stats.clone();
        let mut current_stats = local_stats.lock().unwrap();
        current_stats.as_mut().and_then(|stats| stats.best_action())
    }

    fn algorithm_is_running(&self) -> bool {
        match &self.algorithm_handler {
            Some(handler) => !handler.is_finished(),
            None => false,
        }
    }

    pub fn game_controller(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.set_enabled(!self.algorithm_is_running());

            if ui.button("Reset").clicked() {
                /* Reset current game */
                self.reset_stats();
                self.game = Jigsaw::new();
            }

            if ui.button("Perform Action").clicked() {
                /* Perform selected optimal action */
                let action = self.best_action();
                if let Some(action) = action {
                    self.game.perform_action(action);
                    self.reset_stats();
                }
            }

            if ui.button("Compute").clicked() {
                /* Perform tree search in a new thread */
                self.reset_stats();

                let state = self.game.clone();

                let local_stats = self.current_stats.clone();
                let local_history = self.actions_history.clone();

                let algorithm_config = self.algorithm_config.clone();

                let handler = std::thread::spawn(move || {
                    mcts::compute(&state, algorithm_config, |stats| {
                        let mut history = local_history.lock().unwrap();

                        stats.actions.iter().for_each(|&(action, visits)| {
                            history
                                .entry(action)
                                .or_default()
                                .push([stats.iters.into(), visits.into()])
                        });

                        let mut current_stats = local_stats.lock().unwrap();
                        *current_stats = Some(stats);
                    });
                });

                self.algorithm_handler = Some(handler);
            }
        });

        let local_stats = &self.current_stats.clone();
        let mut stats = local_stats.lock().unwrap();

        let progress = stats.as_mut().map_or_else(
            || 0.0,
            |stats| stats.iters as f32 / self.algorithm_config.max_iters as f32,
        );

        ui.add(egui::ProgressBar::new(progress));
    }

    pub fn jigsaw_window(&mut self, ui: &mut egui::Ui) {
        let cell_size = (50.0, 50.0);

        ui.horizontal_top(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = (0.0, 0.0).into();

                let rows = 4;
                let cols = 6;

                for row in 0..rows {
                    for col in 0..cols {
                        let (rect, response) =
                            ui.allocate_exact_size(cell_size.into(), egui::Sense::click());

                        ui.painter()
                            .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                        if self.game.coord((row, col)) {
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::RED);
                        }

                        let coord = 6 * row + col;

                        if let Some(action) = self.best_action() {
                            if (self.game.figure() >> action) & ((1 << 23) >> coord) != 0 {
                                ui.painter().rect_filled(rect, 0.0, egui::Color32::GREEN);
                            }
                        }

                        if response.clicked() {
                            self.reset_stats();
                            self.game.toggle_coord((row, col));
                            println!("{:#?}", self.game);
                        }
                    }
                    ui.end_row();
                }
            });

            /* Render current figure */
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = (0.0, 0.0).into();

                let rows = 3;
                let cols = 3;

                for row in 0..rows {
                    for col in 0..cols {
                        let (rect, _response) =
                            ui.allocate_exact_size(cell_size.into(), egui::Sense::hover());

                        ui.painter()
                            .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                        let figure = self.game.figure();
                        if figure & (1 << (6 * 4 - 1)) >> row * 6 + col != 0 {
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::RED);
                        }
                    }
                    ui.end_row();
                }
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let round_label = ui.label("Round:");
                    ui.add(egui::DragValue::new(&mut self.game.round).clamp_range(0..=16))
                        .labelled_by(round_label.id);
                });

                for index in 0..ALL_FIGURES.len() {
                    if ui
                        .radio_value(&mut self.game.figure_index, index as u8, "")
                        .clicked()
                    {
                        self.reset_stats();
                    }
                }
            });
        });
    }

    fn adjust_parameters(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Grid::new("algorithm-config-grid")
            .striped(true)
            .show(ui, |ui| {
                ui.set_enabled(!self.algorithm_is_running());

                // exploration rate
                ui.label("Exploration Rate")
                    .on_hover_text("Algorithm exploration rate.");
                ui.add(egui::Slider::new(
                    &mut self.algorithm_config.c,
                    RangeInclusive::new(0.0, 16.0),
                ));
                ui.end_row();

                // iterations
                ui.label("Iterations")
                    .on_hover_text("Algorithm iterations.");
                ui.add(egui::Slider::new(
                    &mut self.algorithm_config.max_iters,
                    1..=800_000,
                ));
                ui.end_row();

                // callback update
                ui.label("Update Interval").on_hover_text(format!(
                    "Update stats every {} iterations.",
                    self.algorithm_config.callback_interval
                ));
                ui.add(egui::Slider::new(
                    &mut self.algorithm_config.callback_interval,
                    1_000..=self.algorithm_config.max_iters,
                ));
                ui.end_row();

                // max depth
                ui.label("Max Depth")
                    .on_hover_text("Steps ahead computed by the algorithm.");
                ui.add(egui::Slider::new(
                    &mut self.algorithm_config.max_depth,
                    1..=16,
                ));
                ui.end_row();

                if ui.button("Reset").clicked() {
                    self.algorithm_config = Config::default();
                }
                ui.end_row();
            });
    }

    fn plot_stats(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label(egui::widget_text::RichText::new(
            "The more visits an action has received, the better it is considered to be because it is an indicator of the action's reliability and importance in the search process."
        ));

        egui::plot::Plot::new("actions-plot")
            .legend(egui::plot::Legend::default())
            .reset()
            .allow_boxed_zoom(false)
            .allow_double_click_reset(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false)
            .clamp_grid(true)
            .show(ui, |plot_ui| {
                let history = self.actions_history.lock().unwrap();

                for (action, data) in history.iter() {
                    plot_ui.line(egui::plot::Line::new(data.clone()).name(format!("#{}", action)));
                    // is neccesary to clone the points ?
                }
            });
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_stats: Arc::default(),
            actions_history: Arc::default(),
            game: Jigsaw::new(),
            algorithm_config: Config::default(),
            algorithm_handler: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.algorithm_is_running() {
            // request repaint if algorithm is running
            ctx.request_repaint();
        }

        if self.game.has_finished() {
            self.reset_stats();
            self.game = Jigsaw::new();
        }

        /*
            Left Side Panel
        */
        egui::SidePanel::left("left-panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(egui::RichText::from("🔧 Adjust Parameters").heading());
                self.adjust_parameters(ui, _frame);

                ui.separator();
                ui.label(egui::RichText::from("🎮 Game Controller").heading());
                self.game_controller(ui, _frame);

                ui.separator();
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        use egui::special_emojis::GITHUB;
                        ui.hyperlink_to(
                            format!("{} GitHub", GITHUB),
                            "https://github.com/agustinemk/fishing-jigsaw",
                        );

                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
                });
            });

        egui::Window::new("📈 Actions Graph").show(ctx, |ui| {
            self.plot_stats(ui, _frame);
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Window::new("🎣 Jigsaw")
                .resizable(false)
                .show(ctx, |ui| self.jigsaw_window(ui));
        });
    }
}
