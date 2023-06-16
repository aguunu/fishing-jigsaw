use crate::{
    manager::Manager,
    minigames::{Jigsaw, ALL_FIGURES},
    CustomWindow,
};

pub type JigsawManager = Manager<Jigsaw>;

impl CustomWindow for Manager<Jigsaw> {
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
