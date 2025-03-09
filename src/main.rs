#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release! YAY! NO MORE CMD!!!!!!!!!!!


mod encrypt_bez_async_i_bez_chacha20;
mod decrypt_copy;
mod ui;
mod utils;
mod moi_chacha20;
mod encrypt_filetype;
mod encrypt_asset_setting;
mod image_actions;
mod image_channel_bundler;
mod create_static_id_per_file;

use eframe::egui;


fn main() {



    let native_options = eframe::NativeOptions {
        
        viewport: egui::ViewportBuilder::default()
        .with_resizable(false)
        .with_inner_size((740.0, 840.0)),
        // renderer: eframe::Renderer::default().egui_wgpu(),
        ..eframe::NativeOptions::default()
    };


    eframe::run_native(
        crate::ui::ui_defaults::Appencja::name(),
        native_options.clone(),
        Box::new(|_| Ok(Box::<crate::ui::ui_defaults::Appencja>::default())),
    )
    .unwrap();





}


