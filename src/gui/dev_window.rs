use eframe::egui;

pub struct DevWindow {
    pub logs: Vec<String>, // Owned logs, synced from SimulatorLauncher
}

impl DevWindow {
    pub fn new(logs: Vec<String>) -> Self {
        Self { logs }
    }
}

impl eframe::App for DevWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Development Logs")
            .id(egui::Id::new("dev_window"))
            .default_pos([1200.0, 0.0])
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Tropism Logs");
                egui::ScrollArea::vertical()
                    .max_height(350.0)
                    .show(ui, |ui| {
                        for log in &self.logs {
                            ui.label(log);
                        }
                    });
            });

        ctx.request_repaint();
    }
}