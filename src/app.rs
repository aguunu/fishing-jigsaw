use egui::Color32;
use rand::SeedableRng;

use crate::jigsaw::{Jigsaw, FIGURES};
use crate::jigsaw;

use crate::deterministic::Deterministic;
use crate::solver::Solver;

pub struct App {
    state: Jigsaw,
    strategy: Deterministic,
    distribution: Distribution,
}

impl Default for App {
    fn default() -> Self {
        let mut strategy = Deterministic::new();
        strategy.run();

        let mut dist = Distribution::default();
        dist.compute(&strategy);

        Self {
            state: Jigsaw::default(),
            strategy: strategy,
            distribution: dist,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

struct Distribution {
    data: Vec<usize>,
    test_size: usize,
    seed: u64,
    state: Jigsaw,
}

impl Default for Distribution {
    fn default() -> Self {
        Self {
            data: vec![],
            test_size: 4096,
            seed: 2024,
            state: Jigsaw::default(),
        }
    }
}

impl Distribution {
    fn set_state(&mut self, state: Jigsaw) {
        self.state = state;
    }

    fn compute(&mut self, s: &Deterministic) {
        let mut rng = SeedableRng::seed_from_u64(self.seed);

        self.data = vec![0; 30];

        for _ in 0..self.test_size {
            // let mut game = Jigsaw::default();
            let mut game = self.state.clone();
            while !game.has_finished() {
                let input = s.solve(&game);
                game.perform_action(input);
                game.set_random_figure(&mut rng);
            }
            let r = game.round as usize;

            while r >= self.data.len() {
                self.data.push(0);
            }

            self.data[r] += 1;
        }
    }

    fn ui(&self, ui: &mut egui::Ui) {
        let n_test = self.test_size;
        let dist = &self.data;

        ui.vertical_centered(|ui| {
            egui_plot::Plot::new("distribution")
                .allow_scroll(false)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_double_click_reset(false)
                .allow_boxed_zoom(false)
                .show_grid(false)
                .show_axes(false)
                .show(ui, |ui| {
                    let bars = dist
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| {
                            egui_plot::Bar::new(i as f64, v as f64 / n_test as f64).width(1.0)
                        })
                        .map(|b| {
                            let color = match b.argument as usize {
                                ..11 => Color32::GREEN,
                                11..25 => Color32::YELLOW,
                                25.. => Color32::RED,
                            };

                            b.fill(color)
                        })
                        .collect();
                    let plt = egui_plot::BarChart::new(bars);
                    ui.bar_chart(plt);
                    ui.vline(egui_plot::VLine::new(10.0).color(Color32::WHITE));
                    ui.vline(egui_plot::VLine::new(25.0).color(Color32::WHITE));
                });

            let rl = dist[0..=10].iter().sum::<usize>();
            let rm = dist[11..=24].iter().sum::<usize>();
            let rs = dist[25..].iter().sum::<usize>();
            let text = egui::RichText::new(format!(
                "P(X < 11) ≈ {:.2} | P(11 < X < 25) ≈ {:.2} | P(25 < X) ≈ {:.2})",
                rl as f64 / n_test as f64,
                rm as f64 / n_test as f64,
                rs as f64 / n_test as f64,
            ))
            .weak();
            ui.label(text);
        });
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let state = self.state.clone();

            if self.state.has_finished() {
                self.state = Jigsaw::default();
                let mut rng = rand::SeedableRng::from_rng(rand::thread_rng()).unwrap();
                self.state.set_random_figure(&mut rng);
            }

            ui.horizontal_wrapped(|ui| {
                let text = "Configure you current game state. \
                    You must input your board state, \
                    current piece and number of rounds.";
                ui.label(text);
            });
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                let best_action = self.strategy.solve(&self.state);

                render_board(ui, &mut self.state, best_action);
                ui.add_space(8.0);
                render_figure(ui, &mut self.state);
            });

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.label("Select the quantity of pieces you have used for your game state.");
            });
            ui.add(
                egui::Slider::new(&mut self.state.round, 0..=30 as u8)
                    .clamp_to_range(true)
                    .text("quantity"),
            );

            ui.horizontal_wrapped(|ui| {
                ui.label("Select the current figure of your game state.");
            });

            ui.add(
                egui::Slider::new(&mut self.state.figure, 0..=(FIGURES.len() - 1) as u8)
                    .clamp_to_range(true)
                    .text("figure"),
            );

            ui.horizontal(|ui| {
                if ui.button("Take").clicked {
                    let best_action = self.strategy.solve(&self.state);
                    self.state.perform_action(best_action);
                    let mut rng = rand::SeedableRng::from_rng(rand::thread_rng()).unwrap();
                    self.state.set_random_figure(&mut rng);
                };

                if ui.button("Reset").clicked {
                    self.state = Jigsaw::default();
                };
            });
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                let text = "This plot represents a distribution of \
                    scores computed by simulating your current \
                    game state until the game has finished following \
                    the strategy. You can see the probability of \
                    finishing the game in less than 10 rounds below.";

                ui.label(text);
            });

            if self.state.round != state.round
                || self.state.figure != state.figure
                || self.state.board != state.board
            {
                self.distribution.set_state(self.state);
                self.distribution.compute(&self.strategy);
            }
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::Vec2::new(ui.available_width(), 100.0), |ui| {
                    self.distribution.ui(ui)
                });
            });

            ui.separator();

            let text = "The plot below represents the average amount \
                of pieces you will need to use from the current \
                game state to the end of the game taking a specific action.\
                Therefore, the smaller the value, the better the outcome will be.";
            ui.label(text);

            ui.allocate_ui(egui::Vec2::new(ui.available_width(), 140.0), |ui| {
                egui_plot::Plot::new("bar-chart")
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .allow_double_click_reset(false)
                    .show_grid(false)
                    .include_x(0.0)
                    .show(ui, |ui| {
                        let mut bars = vec![];
                        for action in self.state.legal_actions() {
                            let mut s = self.state.clone();
                            s.perform_action(action);
                            let dst: Vec<(u8, f32)> = self.strategy.distances(s.board).collect();

                            let n = dst.len();
                            let sum = dst.iter().map(|(_, b)| *b).sum::<f32>();
                            let avg = sum as f64 / n as f64;

                            let bar = egui_plot::Bar::new(action as f64, avg).width(1.0);
                            bars.push(bar);
                        }
                        let plt = egui_plot::BarChart::new(bars);
                        ui.bar_chart(plt);
                    });
            });
            ui.add_space(16.0);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
            ui.add_space(8.0);
        });
    }
}

fn render_figure(ui: &mut egui::Ui, state: &mut Jigsaw) {
    let cell_size = (30.0, 30.0);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = (0.0, 0.0).into();

        let (max_x, max_y) = (3, 3);

        for i in 0..max_x {
            let mut mask = 1 << (jigsaw::N * jigsaw::M - (i + 1));

            ui.horizontal(|ui| {
                for _ in 0..max_y {
                    let (rect, _response) =
                        ui.allocate_exact_size(cell_size.into(), egui::Sense::hover());

                    let value = state.figure().value & mask != 0;

                    let color = if value {
                        egui::Color32::RED
                    } else {
                        egui::Color32::TRANSPARENT
                    };

                    ui.painter().rect_filled(rect, 0.0, color);
                    ui.painter()
                        .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                    mask >>= jigsaw::N;
                }
            });
        }
    });
}

fn render_board(ui: &mut egui::Ui, state: &mut Jigsaw, best_action: u8) {
    let cell_size = (30.0, 30.0);

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = (0.0, 0.0).into();

        for y_offset in 0..jigsaw::N {
            ui.horizontal(|ui| {
                for x_offset in 0..jigsaw::M {
                    let offsets = (x_offset, y_offset);

                    let (rect, response) =
                        ui.allocate_exact_size(cell_size.into(), egui::Sense::click());

                    ui.painter()
                        .rect_stroke(rect, 0.0, (1.0, egui::Color32::WHITE));

                    if state.get_value(offsets) {
                        ui.painter().rect_filled(rect, 0.0, egui::Color32::GOLD);
                    }

                    if state.fig_intesect(best_action, offsets) {
                        ui.painter().rect_filled(rect, 0.0, egui::Color32::GREEN);
                    }

                    if response.hovered() {
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            format!("{}", Jigsaw::offset_to_action(offsets)),
                            egui::FontId::default(),
                            egui::Color32::WHITE,
                        );
                    }

                    if response.clicked() {
                        state.toggle(offsets);
                    }
                }
            });
        }
    });
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Maintained by ");
        ui.hyperlink_to("@aguunu", "https://github.com/aguunu");
        ui.label(" you can find the source code ");
        ui.hyperlink_to("here", "https://github.com/aguunu/fishing-jigsaw");
        ui.label(".");
    });
}
