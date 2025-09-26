use std::path::PathBuf;

use crate::TemplateApp;
use crate::app::Page;
use crate::styles::colors;
use crate::types::wallet_import::WalletImport;
use serde_json;

use egui::{Align, Button, Color32, Layout, RichText, vec2};
use rfd;

pub fn render_home_page(app: &mut TemplateApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        let heading = ui.label(
            RichText::new("Live Wallet")
                .size(50.0)
                .color(colors::BLUE_400),
        );

        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |_ui| heading);

        ui.horizontal(|ui| {
            // Items will flow horizontally

            // Push item to the right (like flex-grow or space-between)
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let import_wallet_button = ui.add(create_wallet_option_button(
                    "Import wallet",
                    colors::BLUE_400,
                ));
                if import_wallet_button.clicked() {
                    println!("Import wallet button clicked");
                    // open import file dialog
                    let path_buf = rfd::FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .set_title("Import wallet")
                        .pick_file();

                    handle_import_wallet_file(path_buf);
                }

                // Button 2: Create Wallet
                let create_wallet_button = ui.add(create_wallet_option_button(
                    "Create wallet",
                    colors::GREEN_400,
                ));
                if create_wallet_button.clicked() {
                    println!("Create wallet button clicked");
                    app.page = Page::CreateWallet;
                }

                // Button 3: Settings
                let settings_button = ui.add(create_wallet_option_button(
                    "Hardware wallet",
                    colors::BLUE_700,
                ));
                if settings_button.clicked() {
                    println!("Create wallet button clicked");
                    // app.page = Page::Settings;
                }
            });
        });
    });
}

fn create_wallet_option_button(text: &'_ str, color: Color32) -> Button<'_> {
    let button_size = vec2(150.0, 50.0); // width, height
    let background_color = Color32::WHITE;
    return Button::new(
        RichText::new(text)
            .size(20.0) // text size
            .color(color),
    )
    .min_size(button_size)
    .fill(background_color);
}

fn handle_import_wallet_file(path: Option<PathBuf>, app: &mut TemplateApp) {
    // open import file dialog
    let path = match path {
        Some(p) => p,
        None => {
            println!("No file selected");
            return;
        }
    };
    match std::fs::read_to_string(path) {
        Ok(json_content) => {
            match serde_json::from_str::<WalletImport>(&json_content) {
                Ok(wallet) => {
                    // Do something with the imported wallet
                    println!("Wallet imported successfully {}", &json_content);
                    app.page = Page::CreateWallet;
                    // todo navigate to create_wallet_page but pass in all this data
                }
                Err(e) => {
                    println!("Failed to parse JSON: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to read file: {}", e);
        }
    }
}
