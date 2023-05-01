use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{mcts::Environment, mcts::Stats, CustomWindow};

pub struct StatsManager<T: Environment> {
    pub current_stats: Arc<Mutex<Option<Stats<T>>>>,
    pub actions_history: Arc<Mutex<HashMap<T::Action, Vec<[f64; 2]>>>>,
}

impl<T: Environment> Default for StatsManager<T> {
    fn default() -> Self {
        Self {
            current_stats: Arc::default(),
            actions_history: Arc::default(),
        }
    }
}
impl<T: Environment> StatsManager<T> {
    pub fn reset(&mut self) {
        self.current_stats = Arc::default();
        self.actions_history = Arc::default();
    }
}

impl<T: Environment> CustomWindow for StatsManager<T> {
    fn name(&self) -> &'static str {
        "📈 Actions Graph"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
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
                    plot_ui.line(egui::plot::Line::new(data.clone()).name(format!("{:?}", action)));
                    // is necessary to clone the points ?
                }
            });
    }
}
