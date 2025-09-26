use crate::pages::create_wallet::render_create_wallet_page;
use crate::pages::home::render_home_page;
use crate::types::wallet_import::WalletImport;
use egui_toast::Toasts;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    pub label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub value: f32,
    pub page: Page,
    pub wallet: Option<WalletImport>,

    #[serde(skip)] // This how you opt-out of serialization of a field
    pub toasts: Toasts,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            page: Page::Home,
            wallet: None,
            toasts: Toasts::new()
                .anchor(egui::Align2::RIGHT_TOP, (20.0, 20.0))
                .direction(egui::Direction::BottomUp),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Page {
    Home,
    CreateWallet,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
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

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            match self.page {
                Page::Home => render_home_page(self, ctx),
                Page::CreateWallet => render_create_wallet_page(self, ctx),
            }
        });
    }
}
