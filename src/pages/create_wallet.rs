use crate::TemplateApp;
use egui::{Color32, RichText};

pub fn render_create_wallet_page(app: &mut TemplateApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("This is a test app for all things me.");

        let home_page_button = ui.button(
            RichText::new("Home Page")
                .size(30.0)
                .color(Color32::LIGHT_BLUE),
        );

        if home_page_button.clicked() {
            app.page = crate::app::Page::Home;
        };
    });
}
