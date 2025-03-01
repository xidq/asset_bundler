#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release! YAY! NO MORE CMD!!!!!!!!!!!


// mod encrypt;
mod encrypt_bez_async_i_bez_chacha20;
mod decrypt_copy;
// mod xor;
mod ui;
mod ui_language;
mod moi_chacha20;
mod encrypt_filetype;
mod encrypt_asset_setting;
// mod ui_window;
mod ui_play_sound;
// use ui_window::*;
// use ui_play_sound::*;
use ui::*;
// use tokio::*;
// use tokio::sync::mpsc;
// use tokio::runtime::Runtime;
// mod data_checker;



use eframe::egui;

// use std::env;



// #[tokio::main]
fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 2 {
    //     print_usage();
    //     return;
    // }
    // let rt = Runtime::new().expect("unable to create Runtime");

    // let _enter = rt.enter();


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((700.0, 700.0)),
        // renderer: eframe::Renderer::default().egui_wgpu(),
        ..eframe::NativeOptions::default()
    };

    // Uruchamiamy GUI w egui
    eframe::run_native(
        Appencja::name(),
        native_options.clone(),
        Box::new(|_| Ok(Box::<Appencja>::default())),
    )
    .unwrap();




    //some tests here, if i was using 2x eframe::run_native than second window was avaliable only 
    // after first one was closed ;(((

    // eframe::run_native(
    //     MoiWindow::name(),
    //     native_options.clone(),
    //     Box::new(|_| Ok(Box::<MoiWindow>::default())),
    // )
    // .unwrap();

    // match args[1].as_str() {
    //     "encrypt" if args.len() == 6 => {
    //         let input_folder = &args[2];
    //         let output_file = &args[3];
    //         let index_file = &args[4];
    //         let template = &args[5];

    //         match encrypt::encrypt_folder(input_folder, output_file, index_file, template) {
    //             Ok(_) => println!("Pliki zaszyfrowane pomyślnie!"),
    //             Err(e) => eprintln!("Błąd szyfrowania: {}", e),
    //         }
    //     }
    //     "decrypt" if args.len() == 5 => {
    //         let dat_file = Path::new(&args[2]);
    //         let idx_file = Path::new(&args[3]);
    //         let output_folder = &args[4];

    //         match decrypt::decrypt_files(dat_file, idx_file, output_folder) {
    //             Ok(_) => println!("Pliki odszyfrowane pomyślnie!"),
    //             Err(e) => eprintln!("Błąd deszyfrowania: {}", e),
    //         }
    //     }
    //     "ui" => {
    //         // Tworzymy kontekst egui
    //         let native_options = eframe::NativeOptions {
    //             viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
    //             ..eframe::NativeOptions::default()
    //         };

    //         // Uruchamiamy GUI w egui
    //         eframe::run_native(
    //             Appencja::name(),
    //             native_options,
    //             Box::new(|_| Ok(Box::<Appencja>::default())),
    //         )
    //         .unwrap();
    //     }
    //     _ => print_usage(),
    // }
}


