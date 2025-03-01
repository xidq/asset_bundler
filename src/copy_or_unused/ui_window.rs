use egui::*;


pub struct MoiWindow {
    pub is_that_window_visible: bool,
}

impl Default for MoiWindow {

    fn default() -> Self {

        MoiWindow {
            is_that_window_visible:false,
        }
    }
}
impl MoiWindow {

    pub fn name() -> &'static str {
        concat!("Asset Bundler v", env!("CARGO_PKG_VERSION"))
    }

}

impl eframe::App for MoiWindow {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::H) && i.modifiers.ctrl) {
            self.is_that_window_visible = !self.is_that_window_visible; 
        }

        ctx.request_repaint();
        if self.is_that_window_visible{
            CentralPanel::default()
                .show(ctx, |ui| {
                    ui.label("lelelelele")
            });
        }
    }
}
