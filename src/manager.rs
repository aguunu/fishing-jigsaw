use std::thread::JoinHandle;

use crate::{
    mcts::{Config, Environment, Tree},
    stats_manager::StatsManager,
};

pub struct Manager<T: Environment + Default> {
    pub state: T,
    pub stats_manager: StatsManager<T>,
    pub handler: Option<JoinHandle<()>>,
    pub config: Option<Config>,
}

impl<T: Environment + Default> Default for Manager<T> {
    fn default() -> Self {
        Self {
            state: T::default(),
            stats_manager: StatsManager::default(),
            handler: None,
            config: None,
        }
    }
}

impl<T: Environment + Default + Clone + Sync + Send + 'static> Manager<T> {
    pub fn optimal_action(&self) -> Option<T::Action> {
        let local_stats = self.stats_manager.current_stats.clone();
        let mut current_stats = local_stats.lock().unwrap();
        current_stats.as_mut().and_then(|stats| stats.best_action())
    }

    pub fn reset(&mut self) {
        self.stats_manager.reset();
        self.state = T::default();
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

    pub fn perform(&mut self, action: T::Action) {
        self.state.perform_action(action);
        self.stats_manager.reset();
    }
}
