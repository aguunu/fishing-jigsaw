use std::thread::JoinHandle;

use crate::{
    mcts::{Config, Environment, Tree},
    minigames::{Jigsaw, ALL_FIGURES},
    stats_manager::StatsManager,
    CustomWindow,
};

pub struct JigsawManager {
    pub state: Jigsaw,
    pub stats_manager: StatsManager<Jigsaw>,
    pub handler: Option<JoinHandle<()>>,
    pub config: Option<Config>,
}

impl Default for JigsawManager {
    fn default() -> Self {
        Self {
            state: Jigsaw::new(),
            stats_manager: StatsManager::default(),
            handler: None,
            config: None,
        }
    }
}

impl JigsawManager {
    pub fn optimal_action(&self) -> Option<u8> {
        let local_stats = self.stats_manager.current_stats.clone();
        let mut current_stats = local_stats.lock().unwrap();
        current_stats.as_mut().and_then(|stats| stats.best_action())
    }

    pub fn reset(&mut self) {
        self.stats_manager.reset();
        self.state = Jigsaw::new();
    }

    pub fn is_computing(&self) -> bool {
        self.handler
            .as_ref()
            .map_or(false, |handler| !handler.is_finished())
    }

    pub fn progress(&self) -> f32 {
        let local_stats = &self.stats_manager.current_stats.clone();
        let stats = local_stats.lock().unwrap();

        if let (Some(config), Some(stats)) = (self.config.as_ref(), stats.as_ref()) {
            stats.iters as f32 / config.max_iters as f32
        } else {
            0.0
        }
    }

    pub fn compute(&mut self, config: &Config) {
        self.config = Some(config.clone());

        self.stats_manager.reset();

        let state = self.state.clone();
        let config = config.clone();

        let local_stats = self.stats_manager.current_stats.clone();
        let local_history = self.stats_manager.actions_history.clone();

        let handler = std::thread::spawn(move || {
            Tree::new(state, config).compute(|stats| {
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

        self.handler = Some(handler);
    }

    pub fn perform(&mut self, action: u8) {
        self.state.perform_action(action);
        self.stats_manager.reset();
    }
}

impl CustomWindow for JigsawManager {
    fn name(&self) -> &'static str {
        "🎣 Jigsaw"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        let cell_size = (50.0, 50.0);

        let optimal_action = self.optimal_action();

        ui.horizontal_top(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = (0.0, 0.0).into();

                let board_shape = (4, 6);

                for row in 0..board_shape.0 {
                    for col in 0..board_shape.1 {
                        let (rect, response) =
                            ui.allocate_exact_size(cell_size.into(), egui::Sense::click());

                        ui.painter()
                            .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                        if self.state.coord(row, col) {
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::GOLD);
                        }

                        if let Some(action) = optimal_action {
                            if self.state.in_figure(action, Jigsaw::index(row, col)) {
                                ui.painter().rect_filled(rect, 0.0, egui::Color32::GREEN);
                            }
                        }

                        if response.hovered() {
                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                Jigsaw::index(row, col).to_string(),
                                egui::FontId::default(),
                                egui::Color32::WHITE,
                            );
                        }

                        if response.clicked() {
                            self.stats_manager.reset();
                            self.state.toggle_coord(row, col);
                            dbg!(&self.state);
                        }
                    }
                    ui.end_row();
                }
            });

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = (0.0, 0.0).into();

                let figure_shape = (3, 3);

                for row in 0..figure_shape.0 {
                    for col in 0..figure_shape.1 {
                        let (rect, _response) =
                            ui.allocate_exact_size(cell_size.into(), egui::Sense::hover());

                        ui.painter()
                            .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                        if self.state.in_figure(0, Jigsaw::index(row, col)) {
                            ui.painter().rect_filled(rect, 0.0, egui::Color32::RED);
                        }
                    }
                    ui.end_row();
                }
            });

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let quantity_label = ui.label("Quantity");
                    ui.add(egui::DragValue::new(&mut self.state.quantity).clamp_range(0..=16))
                        .labelled_by(quantity_label.id);
                });

                for index in 0..ALL_FIGURES.len() {
                    if ui
                        .radio_value(&mut self.state.figure_index, index as u8, String::default())
                        .clicked()
                    {
                        self.stats_manager.reset();
                    }
                }
            });
        });
    }
}
