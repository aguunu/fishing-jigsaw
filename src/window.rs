pub trait CustomWindow {
    fn name(&self) -> &'static str;

    fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name())
            .resizable(false)
            .show(ctx, |ui| self.ui(ui));
    }

    fn ui(&mut self, ui: &mut egui::Ui);
}
