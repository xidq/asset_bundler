use egui::{Color32, Pos2, Response, RichText, Stroke/*, Window,CornerRadius*/};
use egui::{CentralPanel, /*FontFamily,*/FontDefinitions,FontData, TextEdit, ComboBox};
// use rand::random_range;
use ecolor::Hsva;
use rfd::*;

use std::sync::{Arc,mpsc, Mutex};
use crate::ui_play_sound::*;
// use crate::encrypt;
use crate::encrypt_bez_async_i_bez_chacha20;
use crate::decrypt_copy;
use crate::ui_language;
// use core::random;
use std::f32::consts::PI;
// use std::thread;
use std::path::Path;
// use std::time::Duration;
// use rodio::*;
// use std::sync::Arc;
use chrono::offset::Local;
// use rand::seq::IteratorRandom;
// use toml::Value;
// use tokio::*;
// use crate::data_checker;
// use std::vec::*;
// use encrypt::StatusPakExport;










// -------------------------------------------------------------------------------------------------------------------------------------------
// ██████   █████  ████████  █████  
// ██   ██ ██   ██    ██    ██   ██ 
// ██   ██ ███████    ██    ███████ 
// ██   ██ ██   ██    ██    ██   ██ 
// ██████  ██   ██    ██    ██   ██


pub struct Appencja {

    imput_folder_path: String,
    output_folder_path: String,
    imput_de_dat_folder_path: String,
    imput_de_idx_folder_path: String,
    output_de_folder_path: String,
    output_name: String,

    password: String,
    de_password: String,
    template: String,
    show_debug_labels: bool,
    debug_imput_idx_file:String,
    debug_output_idx_path:String,
    debug_idx_password:String,
    debug_create_lua_file: bool,
    toggle_encryption: u8,
    toggle_de_encryption: u8,
    toggle_language:bool,
    current_language: ui_language::Language,
    brainroot: bool,
    toggle_compression: u8,
    poziom_kompresji:u8,
    // rozszerzenie_pliku:&'static str,
    kllikker:i32,
    rotation_image:f32,

    zapis_pracuje:bool,
    czy_to_juz_koniec:u8,
    de_czy_to_juz_koniec:u8,

    settings_toggle:bool,


    szyfrowanie_err_3:String,
    szyfrowanie_err_4:String,


    // encryption_result: Option<Result<Arc<Mutex<Vec<usize>>>, String>>,
    // encryption_rx: Option<mpsc::Receiver<Result<Arc<Mutex<Vec<usize>>>, String>>>,
    tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
    rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

    de_tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
    de_rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

    sz_loading: u8,
    sz_loading_anim: &'static str,
    de_loading_anim: &'static str,
    de_loading: u8,

    timestamp_sekundy:u64,
    timestamp_milisekundy:u32,
    przetworzone_pliki:usize,
    de_timestamp_sekundy:u64,
    de_timestamp_milisekundy:u32,
    de_przetworzone_pliki:usize,




    ustawienia_menu:u8,
    toggle_halp:u8,
    toggle_halp_komunikat:bool,
    nie_krzycz_zmniejszacz:f32,
    nie_krzycz_iter:i32,
    ustawienia_glosnosc:f32,
    

}


// -------------------------------------------------------------------------------------------------------------------------------------------
// ██████  ███████ ███████  █████  ██    ██ ██      ████████ ███████ 
// ██   ██ ██      ██      ██   ██ ██    ██ ██         ██    ██      
// ██   ██ █████   █████   ███████ ██    ██ ██         ██    ███████ 
// ██   ██ ██      ██      ██   ██ ██    ██ ██         ██         ██ 
// ██████  ███████ ██      ██   ██  ██████  ███████    ██    ███████

impl Default for Appencja {

    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        let (de_tx, de_rx) = mpsc::channel();
        Appencja {

            imput_folder_path: String::from(""),
            output_folder_path: String::from(""),
            imput_de_dat_folder_path: String::from(""),
            imput_de_idx_folder_path: String::from(""),
            output_de_folder_path: String::from(""),
            output_name: String::from("output_file"),
            password: String::from("lubieplacki6969"),
            de_password: String::from("lubieplacki6969"),
            template: String::from("None"),
            show_debug_labels: false, 
            debug_imput_idx_file: String::from(""),
            debug_output_idx_path: String::from(""),
            debug_idx_password: String::from("lubieplacki6969"),
            debug_create_lua_file: false, 
            toggle_encryption:0,
            toggle_de_encryption:0,
            toggle_language:false,
            current_language: ui_language::Language::polish(),
            brainroot:false,
            toggle_compression:0,
            poziom_kompresji:3,
            kllikker:0,
            rotation_image:0.0,
            // rozszerzenie_pliku: "dat",
            tx,
            rx,
            de_tx,
            de_rx,
            sz_loading: 0,
            de_loading: 0,

            sz_loading_anim: "",
            de_loading_anim: "",

            zapis_pracuje: false,
            czy_to_juz_koniec:0,
            de_czy_to_juz_koniec:0,

            settings_toggle:false,


            szyfrowanie_err_3:"".to_string(),
            szyfrowanie_err_4:"".to_string(),

            // encryption_result: None,
            // encryption_rx: None,

            przetworzone_pliki:0,
            timestamp_sekundy:0,
            timestamp_milisekundy:0,
            de_timestamp_sekundy:0,
            de_timestamp_milisekundy:0,
            de_przetworzone_pliki:0,



            //kliki
            ustawienia_menu:0,
            toggle_halp:0,
            toggle_halp_komunikat:false,
            nie_krzycz_zmniejszacz:1.,
            nie_krzycz_iter:0,
            ustawienia_glosnosc:1.,
        }

    }
    
}


// -------------------------------------------------------------------------------------------------------------------------------------------
// ████████ ██ ████████ ██      ███████ 
//    ██    ██    ██    ██      ██      
//    ██    ██    ██    ██      █████   
//    ██    ██    ██    ██      ██      
//    ██    ██    ██    ███████ ███████

impl Appencja {

    pub fn name() -> &'static str {
        concat!("Asset Bundler v", env!("CARGO_PKG_VERSION"))
    }

}

// -------------------------------------------------------------------------------------------------------------------------------------------

impl eframe::App for Appencja {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        

        if self.toggle_language {

            self.current_language = ui_language::Language::english();

        } else {

            self.current_language = ui_language::Language::polish();

        }

        // -------------------------------------------------------------------------------------------------------------------------------------------
        //  ██████  ██    ██ ███████ ██████  ██████  ██ ██████  ███████ 
        // ██    ██ ██    ██ ██      ██   ██ ██   ██ ██ ██   ██ ██      
        // ██    ██ ██    ██ █████   ██████  ██████  ██ ██   ██ █████   
        // ██    ██  ██  ██  ██      ██   ██ ██   ██ ██ ██   ██ ██      
        //  ██████    ████   ███████ ██   ██ ██   ██ ██ ██████  ███████ 
                                                                     
                                                                     
        // ███████  ██████  ███    ██ ████████                          
        // ██      ██    ██ ████   ██    ██                             
        // █████   ██    ██ ██ ██  ██    ██                             
        // ██      ██    ██ ██  ██ ██    ██                             
        // ██       ██████  ██   ████    ██   
                // Konfiguracja czcionki japońskiej
        let mut font_definitions = FontDefinitions::default();

        // // Wczytanie czcionki bezpośrednio do pliku wykonywalnego
        let font_data = FontData::from_static(include_bytes!("../assets/Forum-Regular.ttf"));

        font_definitions.font_data.insert("XanhMono".to_owned(), Arc::new(font_data));
        font_definitions.families.insert(

            egui::FontFamily::Proportional, // Use default proportional to override, u can add new like in next example
            vec!["XanhMono".to_owned()],

        );

        //confirm
        ctx.set_fonts(font_definitions);



                // // Konfiguracja czcionki japońskiej
                // let mut font_definitions = FontDefinitions::default();
        
                // // Wczytanie czcionki bezpośrednio do pliku wykonywalnego
                // let font_data = FontData::from_static(include_bytes!("../assets/NotoSerifJP-VariableFont_wght.ttf"));

        
                // font_definitions.font_data.insert("japanese_font".to_owned(), Arc::new(font_data));
                // font_definitions.families.insert(
                //     egui::FontFamily::Name("japanese_font".to_owned().into()), // Używamy dedykowanej rodziny
                //     vec!["japanese_font".to_owned()],
                // );

        
                // // Tutaj konfigurujesz czcionki w kontekście, ale bez globalnej zmiany
                // ctx.set_fonts(font_definitions);
        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██   ██ ███████ ██    ██                                   
        // ██  ██  ██       ██  ██                                    
        // █████   █████     ████                                     
        // ██  ██  ██         ██                                      
        // ██   ██ ███████    ██                                      
                                                                
                                                                
        // ██████  ██ ███    ██ ██████  ██ ███    ██  ██████  ███████ 
        // ██   ██ ██ ████   ██ ██   ██ ██ ████   ██ ██       ██      
        // ██████  ██ ██ ██  ██ ██   ██ ██ ██ ██  ██ ██   ███ ███████ 
        // ██   ██ ██ ██  ██ ██ ██   ██ ██ ██  ██ ██ ██    ██      ██ 
        // ██████  ██ ██   ████ ██████  ██ ██   ████  ██████  ███████ 
        // naciskanie klawiszy i tego konsekwencje
        if ctx.input(|i| i.key_pressed(egui::Key::E) && i.modifiers.ctrl) {
            self.show_debug_labels = !self.show_debug_labels; 
        }
        if ctx.input(|i| i.key_pressed(egui::Key::B) && i.modifiers.ctrl) {
            self.brainroot = !self.brainroot; 
        }
        // if ctx.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl) {
        //     self.toggle_encryption = !self.toggle_encryption; 
        // }
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.settings_toggle = !self.settings_toggle; 
        }

        // checker for filled necessary data
        let sprawdzacz_plikow_kompresji= if !self.imput_folder_path.is_empty() && !self.output_folder_path.is_empty() && !self.output_name.is_empty(){
            true
        }else{
            false
        };


        // -------------------------------------------------------------------------------------------------------------------------------------------
        //  ██████  ██████  ██       ██████  ██████  ███████ 
        // ██      ██    ██ ██      ██    ██ ██   ██ ██      
        // ██      ██    ██ ██      ██    ██ ██████  ███████ 
        // ██      ██    ██ ██      ██    ██ ██   ██      ██ 
        //  ██████  ██████  ███████  ██████  ██   ██ ███████

        //kolory niestandardowe:
        let szarawy_ciemny_hsva = Hsva {
            h: 0.5,    // 0-1
            s: 0.2,    // 0-1
            v: 0.05,    // 0-1
            a: 1.0,    // 0-1
        };

        let szarawy_ciemny: Color32 = szarawy_ciemny_hsva.into();

        let fiolet_ciemny_hsva = Hsva {
            h: 0.9,    // Hue: 0 degrees (czerwony kolor)
            s: 0.7,    // Saturation: pełna saturacja (intensywny kolor)
            v: 0.2,    // Value: pełna jasność
            a: 1.0,    // Alpha: pełna przezroczystość
        };

        let fiolet_ciemny: Color32 = fiolet_ciemny_hsva.into();

        let zolty_ciemny_hsva = Hsva {
            h: 0.091,    // Hue: 0 degrees (czerwony kolor)
            s: 0.57,    // Saturation: pełna saturacja (intensywny kolor)
            v: 0.4,    // Value: pełna jasność
            a: 1.0,    // Alpha: pełna przezroczystość
        };

        let zolty_ciemny: Color32 = zolty_ciemny_hsva.into();



        //another checker
        let sprawdzacz_plikow_dekompresja: bool = 
            Path::new(&self.imput_de_dat_folder_path)
                .extension()
                .map_or(false,
                    |ext| ext == "jrz" || ext == "jrzs") &&
            Path::new(&self.imput_de_idx_folder_path)
                .extension()
                .map_or(false, 
                    |ext| ext == "idx") &&
            !self.output_de_folder_path.is_empty();


        //spacje
        // let sss:f32=20.;
        let ss:f32=10.;
        let bs:f32 =5.;

        //rozm czcionek
        let cm:f32 = 8.;
        let _cs:f32 = 12.;
        let cd:f32 = 16.;

        //kółka
        let full_cirkul_sajz:f32 = 7.;
        let full_cirkul_kolor = Color32::GREEN;
        let pusty_cirkul_sajz:f32 = 5.;
        let pusty_cirkul_kolor = Color32::GRAY;
        let pusty_cirkul_ma_stroke: f32 = 1.5;
        let offset_cirkul: f32 = 14.;

        //override some things bcoz i want to
        let visuals = ctx.style().visuals.clone();
        let mut visuals = visuals;
        visuals.selection.bg_fill = fiolet_ciemny; // Zmiana tła na zielony przy selekcji
        visuals.selection.stroke = Stroke{width:2.,color:Color32::WHITE};
        ctx.set_visuals(visuals);

        


        egui_extras::install_image_loaders(ctx);

        ctx.request_repaint();
        // let centralny_panel = egui::containers::Frame {

        //     // fill: Color32::DARK_BLUE,
        //     corner_radius:CornerRadius{nw:200,ne:200,sw:2,se:2},
        //     ..Default::default()
        // };

        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██    ██ ██ 
        // ██    ██ ██ 
        // ██    ██ ██ 
        // ██    ██ ██ 
        //  ██████  ██ 

        CentralPanel::default()
        .show(ctx, |_ui| { //.frame(centralny_panel)


        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██       ███████ ███████ ████████          
        // ██       ██      ██         ██             
        // ██       █████   █████      ██             
        // ██       ██      ██         ██             
        // ████████ ███████ ██         ██             
                                                
                                                
        // ██████   █████  ███    ██ ███████ ██      
        // ██   ██ ██   ██ ████   ██ ██      ██      
        // ██████  ███████ ██ ██  ██ █████   ██      
        // ██      ██   ██ ██  ██ ██ ██      ██      
        // ██      ██   ██ ██   ████ ███████ ██████ 

            egui::SidePanel::left("lewy_panel")
            .exact_width(350.)
            .resizable(false)
            .show(ctx, |ui|{
                egui::ScrollArea::vertical()
                .show(ui, |ui| {




                        ui.horizontal(|ui|{
                            // ui.add(egui::Label::new(encrypt::encrypt_folder::Result);
                            ui.add_space(250.);
                            ui.selectable_value(&mut self.toggle_language, false, "PL");
                            ui.add(egui::Label::new("||").selectable(false));
                            ui.selectable_value(&mut self.toggle_language, true, "EN");

                        });
                                                                                        
                        ui.add_space(bs);

                        //  _             _         
                        // | |_ ___ ___ _| |___ ___ 
                        // |   | -_| .'| . | -_|  _|     font:triangles  http://patorjk.com/software/taag/#p=display&f=Rectangles&t=header
                        // |_|_|___|__,|___|___|_| 

                        ui.vertical_centered(|ui| {

                            ui.add(egui::Label::new(RichText::new(self.current_language.szyfrowanie_naglowek).size(cd)).selectable(false));

                        });

                        ui.add_space(ss);

                        //  _                 _              _   _   
                        // |_|___ ___ ___ ___| |_    ___ ___| |_| |_ 
                        // | |   | . | . |  _|  _|  | . | .'|  _|   |
                        // |_|_|_|  _|___|_| |_|    |  _|__,|_| |_|_|
                        //       |_|                |_|             

                        ui.add(egui::Label::new(self.current_language.wybierz_folder_wejsciowy).selectable(false));

                        ui.add_space(bs);

                        ui.horizontal(|ui|{

                            ui.add_space(20.);
                            
                            let btn_szyfrowanie_folder_wejsciowy : Response = ui.button(self.current_language.wybierz_folder);

                            if btn_szyfrowanie_folder_wejsciowy.clicked() {
                                self.czy_to_juz_koniec = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {

                                    self.imput_folder_path = path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wejsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wejsciowy.rect.min.x - offset_cirkul,
                                btn_szyfrowanie_folder_wejsciowy.rect.min.y+(btn_szyfrowanie_folder_wejsciowy.rect.size().y / 2.));
                            
                            if !self.imput_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, full_cirkul_sajz, full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));

                            }

                            if self.imput_folder_path.len() < 28{

                                ui.add(egui::Label::new(&self.imput_folder_path).selectable(false));

                            }

                        });

                        if self.imput_folder_path.len() >= 28{

                            ui.add(egui::Label::new(&self.imput_folder_path).selectable(false));

                        }

                        ui.add_space(ss);
                                                                    
                        //          _           _              _   _   
                        //  ___ _ _| |_ ___ _ _| |_    ___ ___| |_| |_ 
                        // | . | | |  _| . | | |  _|  | . | .'|  _|   |
                        // |___|___|_| |  _|___|_|    |  _|__,|_| |_|_|
                        //             |_|            |_|             

                        ui.add(egui::Label::new(self.current_language.wybierz_folder_wyjsciowy).selectable(false));

                        ui.add_space(bs);

                        ui.horizontal(|ui|{



                            ui.add_space(20.);

                            let btn_szyfrowanie_folder_wyjsciowy = ui.button(self.current_language.wybierz_folder);

                            if btn_szyfrowanie_folder_wyjsciowy.clicked() {
                                self.czy_to_juz_koniec = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    // Zapisujemy wybrany folder
                                    self.output_folder_path = path.to_string_lossy().to_string();
                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.x - offset_cirkul,
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.y+(btn_szyfrowanie_folder_wyjsciowy.rect.size().y / 2.));
                            
                            if !self.output_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, full_cirkul_sajz, full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));

                            }

                            if self.output_folder_path.len() < 28 {

                                ui.add(egui::Label::new(&self.output_folder_path).selectable(false));

                            }

                        });

                        if self.output_folder_path.len() >= 28{

                            ui.add(egui::Label::new(&self.output_folder_path).selectable(false));

                        }

                        ui.add_space(ss);

                        //  ___ _ _                          
                        // |  _|_| |___    ___ ___ _____ ___ 
                        // |  _| | | -_|  |   | .'|     | -_|
                        // |_| |_|_|___|  |_|_|__,|_|_|_|___|
                                                        
                        ui.add(egui::Label::new(self.current_language.szyfrowanie_nazwa_tytul).selectable(false));

                        ui.add_space(ss);

                        let output_name_input = TextEdit::singleline(&mut self.output_name)
                            .hint_text(&self.current_language.szyfrowanie_nazwa.to_string());

                        ui.add(output_name_input);

                        ui.add_space(ss);







                        //                                    _         
                        //  ___ ___ _____ ___ ___ ___ ___ ___|_|___ ___ 
                        // |  _| . |     | . |  _| -_|_ -|_ -| | . |   |
                        // |___|___|_|_|_|  _|_| |___|___|___|_|___|_|_|
                        //               |_|        
                                    
                        ui.add(egui::Label::new(&self.current_language.szyfrowanie_kompresja_wybor.to_string()).selectable(false));

                        ui.add_space(bs);

                        ui.horizontal(|ui|{

                            let c_c_b = ui.selectable_value(&mut self.toggle_compression, 0, "Brak");
                            if c_c_b.clicked(){self.czy_to_juz_koniec=0};

                            ui.add(egui::Label::new("||").selectable(false));
                            // let c_c_z = ui.add(egui::Label::new(RichText::new("Zstd").strikethrough());
                            // if c_c_z.hovered(){
                            //     ui.add(egui::Label::new("Ma problemy egzystencjonalne");
                            // };
                            let c_c_z = ui.selectable_value(&mut self.toggle_compression, 1, "Zstd");
                            if c_c_z.clicked(){self.czy_to_juz_koniec=0};

                        });

                        ui.add_space(bs);
                        
                        ui.horizontal(|ui|{
                        
                            ui.add(egui::Label::new(&self.current_language.szyfrowanie_kompresja.to_string()).selectable(false));
                            let compression_empty_let:bool;
                            if self.toggle_compression>=1{compression_empty_let=true}else{compression_empty_let=false};
                            ui.add_enabled(compression_empty_let,|ui: &mut egui::Ui|{
                                ui.add(egui::Slider::new(&mut self.poziom_kompresji, 1..=22).text(""))

                            });
                        
                        });
                        
                        ui.add_space(ss);

                        //                          _   _         
                        //  ___ ___ ___ ___ _ _ ___| |_|_|___ ___
                        // | -_|   |  _|  _| | | . |  _| | . |   |
                        // |___|_|_|___|_| |_  |  _|_| |_|___|_|_|
                        //                 |___|_|                


                        ui.horizontal(|ui|{

                            let c_e_b =ui.selectable_value(&mut self.toggle_encryption, 0,  "Brak");
                            if c_e_b.clicked(){self.czy_to_juz_koniec=0}
                            ui.add(egui::Label::new("||").selectable(false));
                            let c_e_x = ui.selectable_value(&mut self.toggle_encryption, 1,  RichText::new("chacha, do not use!").strikethrough());
                            // if c_e_x.clicked() {self.czy_to_juz_koniec=0}
                            // let c_e_x = ui.add(egui::Label::new(RichText::new("chacha").strikethrough());
                            if c_e_x.hovered(){
                                ui.add(egui::Label::new(&self.current_language.problem_egzystencjonalny.to_string()).selectable(false));
                            };


                        });
                        if self.toggle_encryption == 0{

                            ui.add_space(ss);
                            ui.vertical_centered(|ui|{

                                ui.add(egui::Label::new(RichText::new(self.current_language.szyfrowanie_haslo_wylaczone).color(fiolet_ciemny)).selectable(false));
                            
                            });

                            ui.add_space(ss);

                        }else{

                            ui.add(egui::Label::new(self.current_language.szyfrowanie_haslo_tytul.to_string()).selectable(false));
                            ui.add_space(bs);
                            let password_input = TextEdit::singleline(&mut self.password)
                            .password(true)
                            .hint_text(&self.current_language.szyfrowanie_haslo.to_string());
                            ui.add(password_input);

                        }

                        //  _                 _     _       
                        // | |_ ___ _____ ___| |___| |_ ___ 
                        // |  _| -_|     | . | | .'|  _| -_|
                        // |_| |___|_|_|_|  _|_|__,|_| |___|
                        //               |_|                

                        ui.add_space(ss);

                        ComboBox::from_label(&self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string())
                            .selected_text(self.template.clone())
                            .show_ui(ui, |ui| {

                                ui.selectable_value(&mut self.template, "none".to_string(), "none");
                                ui.selectable_value(&mut self.template, "assets".to_string(), "assets");
                                ui.selectable_value(&mut self.template, "images".to_string(), "images");
                                ui.selectable_value(&mut self.template, "audio".to_string(), "audio");
                                ui.selectable_value(&mut self.template, "3d_model".to_string(), "3d models");
                                ui.selectable_value(&mut self.template, "documents".to_string(), "documents");
                                ui.selectable_value(&mut self.template, "raw_photos".to_string(), "raw photos");
                            
                            });

                        //  _       _   _           
                        // | |_ _ _| |_| |_ ___ ___ 
                        // | . | | |  _|  _| . |   |
                        // |___|___|_| |_| |___|_|_|
                        

                               

                        let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

                            &self.current_language.szyfrowanie_przycisk_ok}
                            else
                            {&self.current_language.szyfrowanie_przycisk_nie_ok};
                        
                        // ui.add_space(ss);
                        // let t_p_d= match self.czy_to_juz_koniec{
                        //     0 => tekst_przycisku_kompresji,
                        //     1 => &&sz_loading_anim as &str,
                        //     2 => &self.current_language.szyfrowanie_przycisk_koniec,
                        //     3 => &self.current_language.szyfrowanie_przycisk_3,
                        //     4 => &self.current_language.szyfrowanie_przycisk_4,
                        //     _ => ""
                        // };


                        let blblblblblblbl = if !self.imput_folder_path.is_empty() && self.czy_to_juz_koniec !=1 && !self.output_folder_path.is_empty() && !self.output_name.is_empty() && !self.zapis_pracuje{true}else{false};
                        let szyfr_butt_col = if sprawdzacz_plikow_kompresji == true && self.czy_to_juz_koniec ==0{Color32::DARK_GREEN}else if sprawdzacz_plikow_kompresji == true && self.czy_to_juz_koniec ==1{zolty_ciemny}else{szarawy_ciemny};

                        match self.rx.try_recv() {
                            Ok(Ok(afasdaf)) => {
                                let danene = afasdaf.lock().unwrap();
                                self.czy_to_juz_koniec = 2;
                                self.przetworzone_pliki = danene[0];
                                self.timestamp_sekundy = danene[1] as u64;
                                self.timestamp_milisekundy = danene[2] as u32;
                                play_finish_sound(self.ustawienia_glosnosc);
                            }
                            Ok(Err(e)) => {
                                self.czy_to_juz_koniec = 3;
                                self.szyfrowanie_err_3 = e.to_string();
                                // eprintln!("Błąd: {}", e);
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => {
                                ctx.request_repaint();
                                match self.sz_loading{
                                    181 => {self.sz_loading = 0; self.sz_loading_anim=""},
                                    151..=180 => {self.sz_loading += 1; self.sz_loading_anim="[=====>]"},
                                    121..=150 => {self.sz_loading += 1; self.sz_loading_anim="[====>-]"},
                                    91..=120 => {self.sz_loading += 1; self.sz_loading_anim="[===>--]"},
                                    61..=90 => {self.sz_loading += 1; self.sz_loading_anim="[==>---]"},
                                    30..=60 => {self.sz_loading += 1; self.sz_loading_anim="[=>----]"},
                                    _ => {self.sz_loading += 1; self.sz_loading_anim="[>-----]"},
                                }  

                            }
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                self.czy_to_juz_koniec = 4;
                                self.szyfrowanie_err_3 = "Disconected".to_string();
                            }
                        }
                        ui.add_space(ss);
                        let t_p_d= match self.czy_to_juz_koniec{
                            0 => RichText::new(tekst_przycisku_kompresji.to_string()),
                            1 => RichText::new(self.sz_loading_anim.to_string()).monospace().color(Color32::BLACK),
                            2 => RichText::new(&self.current_language.szyfrowanie_przycisk_koniec.to_string()),
                            3 => RichText::new(&self.current_language.szyfrowanie_przycisk_3.to_string()),
                            4 => RichText::new(&self.current_language.szyfrowanie_przycisk_4.to_string()),
                            _ => RichText::new("".to_string())
                        };

                        ui.vertical_centered(|ui|{
                            if ui.add(egui::Button::new(t_p_d
                            .size(cd))
                            .min_size(egui::vec2(250.0, 40.0))
                            .corner_radius(10.)
                            .fill(szyfr_butt_col))
                            .clicked() {



                                if blblblblblblbl{
                                    self.czy_to_juz_koniec = 0;
                                    let rozszerzenie_plikku = if self.toggle_encryption == 0 {"jrz"} else if self.toggle_encryption == 1 {"jrzs"} else{"bin"};
                                    let output_file = format!("{}/{}.{}" , self.output_folder_path, self.output_name,rozszerzenie_plikku);
                                    let index_file = format!("{}/{}.idx", self.output_folder_path, self.output_name);

                                    let arc_z_u8 = Arc::new(Mutex::new(vec![self.toggle_encryption,self.toggle_compression,self.poziom_kompresji]));
                                    let arc_z_bool = Arc::new(Mutex::new(vec![self.debug_create_lua_file]));
                                    let arc_z_str = Arc::new(Mutex::new(vec![self.imput_folder_path.clone().to_string(),output_file.clone().to_string(),index_file.clone().to_string(),self.template.clone().to_string(),self.password.clone().to_string()]));
                                        
                                        

                                    let tx_clone = self.tx.clone();
                                    std::thread::spawn(move || {
                                        let result = encrypt_bez_async_i_bez_chacha20::encrypt_folder(
                                            arc_z_str,
                                            arc_z_u8,
                                            arc_z_bool,
                                        );
                                        
                                        match tx_clone.send(result) {
                                            Ok(_) => println!("Wysłano wynik"),
                                            Err(e) => eprintln!("Błąd wysyłania: {}", e),
                                        }
                                    });
                                    
                                    self.czy_to_juz_koniec = 1;

                                }}

                                            

                            });


                                ui.add_space(ss);

                    ui.add_space(ss);

                    ui.separator();

                    ui.add_space(ss);
    

                                        // ██╗   ██╗███╗   ██╗██████╗  █████╗  ██████╗██╗  ██╗
                                        // ██║   ██║████╗  ██║██╔══██╗██╔══██╗██╔════╝██║ ██╔╝
                                        // ██║   ██║██╔██╗ ██║██████╔╝███████║██║     █████╔╝ 
                                        // ██║   ██║██║╚██╗██║██╔═══╝ ██╔══██║██║     ██╔═██╗ 
                                        // ╚██████╔╝██║ ╚████║██║     ██║  ██║╚██████╗██║  ██╗
                                        //  ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝                


                    // ●○⊙◯

                    //  _   _ _   _     
                    // | |_|_| |_| |___ 
                    // |  _| |  _| | -_|
                    // |_| |_|_| |_|___|

                    ui.vertical_centered(|ui| {

                        ui.add(egui::Label::new(RichText::new(self.current_language.deszyfrowanie_naglowek.to_string()).size(cd)).selectable(false));

                    });

                    ui.add_space(ss);

                    //                       _     _      ___ _ _     
                    //  ___ ___ ___ ___    _| |___| |_   |  _|_| |___ 
                    // | . | . | -_|   |  | . | .'|  _|  |  _| | | -_|
                    // |___|  _|___|_|_|  |___|__,|_|    |_| |_|_|___|
                    //     |_|                                        

                    ui.horizontal(|ui|{

                        ui.add_space(20.);

                        let btn_deszyfrowanie_folder_plik_dat = ui.button(&self.current_language.deszyfrowanie_plik_dat.to_string());
                        if btn_deszyfrowanie_folder_plik_dat.clicked() {
                            if let Some(path) = FileDialog::new().add_filter(&self.current_language.deszyfrowanie_plik_dat.to_string(), &["jrz","jrzs"]).pick_file() {

                                self.imput_de_dat_folder_path = path.to_string_lossy().to_string();
                    
                                // Sprawdzanie, czy wybrany plik ma rozszerzenie .jrzs
                                if path.extension().map(|ext| ext == "jrzs").unwrap_or(false) {
                                    self.toggle_de_encryption = 1; // Zmienna is_jrzs ustawiona na true
                                } else {
                                    self.toggle_de_encryption = 0; // Jeśli nie jest .jrzs, ustaw false
                                }
                            }

                        }

                        let btn_pozycjonowanie_deszyfrowanie_plik_dat = Pos2::new(
                            btn_deszyfrowanie_folder_plik_dat.rect.min.x - offset_cirkul,
                            btn_deszyfrowanie_folder_plik_dat.rect.min.y+(btn_deszyfrowanie_folder_plik_dat.rect.size().y / 2.));
                        
                        if !self.imput_de_dat_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_deszyfrowanie_plik_dat, full_cirkul_sajz, full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_deszyfrowanie_plik_dat, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));

                        }

                        if self.imput_de_dat_folder_path.len() < 28{

                            ui.add(egui::Label::new(&self.imput_de_dat_folder_path).selectable(false));
                        
                        }

                    });

                    if self.imput_de_dat_folder_path.len() >=28{

                        ui.add(egui::Label::new(&self.imput_de_dat_folder_path).selectable(false));

                    }

                    //                     _   _        ___ _ _     
                    //  ___ ___ ___ ___   |_|_| |_ _   |  _|_| |___ 
                    // | . | . | -_|   |  | | . |_'_|  |  _| | | -_|
                    // |___|  _|___|_|_|  |_|___|_,_|  |_| |_|_|___|
                    //     |_|                                      

                    ui.add_space(ss);

                    ui.horizontal(|ui|{

                        ui.add_space(20.);

                        let btn_deszyfrowanie_folder_plik_idx = ui.button(&self.current_language.deszyfrowanie_plik_idx.to_string());

                        if btn_deszyfrowanie_folder_plik_idx.clicked() {

                            if let Some(path) = FileDialog::new().add_filter(&self.current_language.deszyfrowanie_plik_idx.to_string(), &["idx"]).pick_file() {

                                self.imput_de_idx_folder_path = path.to_string_lossy().to_string();

                            }
                        }

                        let btn_pozycjonowanie_deszyfrowanie_plik_idx = Pos2::new(
                            btn_deszyfrowanie_folder_plik_idx.rect.min.x - offset_cirkul,
                            btn_deszyfrowanie_folder_plik_idx.rect.min.y+(btn_deszyfrowanie_folder_plik_idx.rect.size().y / 2.));
                        
                        if !self.imput_de_idx_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_deszyfrowanie_plik_idx, full_cirkul_sajz, full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_deszyfrowanie_plik_idx, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));

                        }

                        if self.imput_de_idx_folder_path.len() < 28 {

                            ui.add(egui::Label::new(&self.imput_de_idx_folder_path).selectable(false));

                        }

                    });

                    if self.imput_de_idx_folder_path.len() >= 28 {

                        ui.add(egui::Label::new(&self.imput_de_idx_folder_path).selectable(false));

                    }

                    ui.add_space(ss);

                    //          _           _      ___     _   _         
                    //  ___ _ _| |_ ___ _ _| |_   |  _|___| |_| |___ ___ 
                    // | . | | |  _| . | | |  _|  |  _| . | | . | -_|  _|
                    // |___|___|_| |  _|___|_|    |_| |___|_|___|___|_|  
                    //             |_|                                   

                    ui.add(egui::Label::new(self.current_language.wybierz_folder_wyjsciowy.to_string()).selectable(false));

                    ui.add_space(bs);

                    ui.horizontal(|ui|{

                        ui.add_space(20.);

                        let btn_deszyfrowanie_folder_folder_wyjsciowy = ui.button(self.current_language.h_wybierz_folder);
                        if btn_deszyfrowanie_folder_folder_wyjsciowy.clicked() {
                            
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.output_de_folder_path = path.to_string_lossy().to_string()+"/";
                            }

                        }

                        if self.output_de_folder_path.len() < 28 {

                            ui.add(egui::Label::new(&self.output_de_folder_path).selectable(false));

                        }

                        let btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy = Pos2::new(
                            btn_deszyfrowanie_folder_folder_wyjsciowy.rect.min.x - offset_cirkul,
                            btn_deszyfrowanie_folder_folder_wyjsciowy.rect.min.y+(btn_deszyfrowanie_folder_folder_wyjsciowy.rect.size().y / 2.));
                        
                        if !self.output_de_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy, full_cirkul_sajz, full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));

                        }

                    });

                    if self.output_de_folder_path.len() >= 28 {

                        ui.add(egui::Label::new(&self.output_de_folder_path).selectable(false));

                    }

                    //                          _   _         
                    //  ___ ___ ___ ___ _ _ ___| |_|_|___ ___ 
                    // | -_|   |  _|  _| | | . |  _| | . |   |
                    // |___|_|_|___|_| |_  |  _|_| |_|___|_|_|
                    //                 |___|_|               

                    ui.add_space(ss);

                    if self.toggle_de_encryption==0{

                        ui.vertical_centered(|ui|{


                            ui.add(egui::Label::new(RichText::new(&self.current_language.szyfrowanie_haslo_wylaczone.to_string()).color(fiolet_ciemny)).selectable(false));

                        });

                    }else{

                        ui.add(egui::Label::new(&self.current_language.szyfrowanie_haslo_tytul.to_string()).selectable(false));
                        ui.add_space(bs);
                        let password_input = TextEdit::singleline(&mut self.de_password)
                            .password(true)
                            .hint_text(&self.current_language.szyfrowanie_haslo.to_string());
                        ui.add(password_input);

                    };

                    ui.add_space(ss);

                    //  _       _   _           
                    // | |_ _ _| |_| |_ ___ ___ 
                    // | . | | |  _|  _| . |   |
                    // |___|___|_| |_| |___|_|_|
                                            
                    let deszyfr_tekst_check= if sprawdzacz_plikow_dekompresja {

                        &self.current_language.deszyfrowanie_przycisk_ok.to_string()
                        
                    } else {

                        &self.current_language.szyfrowanie_przycisk_nie_ok.to_string()

                    };
                    let de_t_p_d= match self.de_czy_to_juz_koniec{
                        0 => RichText::new(tekst_przycisku_kompresji.to_string()),
                        1 => RichText::new(self.de_loading_anim).monospace().color(Color32::BLACK),
                        2 => RichText::new(&self.current_language.szyfrowanie_przycisk_koniec.to_string()),
                        3 => RichText::new(&self.current_language.szyfrowanie_przycisk_3.to_string()),
                        4 => RichText::new(&self.current_language.szyfrowanie_przycisk_4.to_string()),
                        _ => RichText::new("")
                    };

                    match self.de_rx.try_recv() {
                        Ok(Ok(kgfghkh)) => {
                            let danene = kgfghkh.lock().unwrap();
                            self.de_czy_to_juz_koniec = 2;
                            self.de_przetworzone_pliki = danene[0];
                            self.de_timestamp_sekundy = danene[1] as u64;
                            self.de_timestamp_milisekundy = danene[2] as u32;
                            play_finish_sound(self.ustawienia_glosnosc);
                        }
                        Ok(Err(e)) => {
                            self.de_czy_to_juz_koniec = 3;
                            self.szyfrowanie_err_3 = e.to_string();
                            // eprintln!("Błąd: {}", e);
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {
                            ctx.request_repaint();
                            match self.de_loading{
                                181 => {self.de_loading = 0; self.de_loading_anim=""},
                                151..=180 => {self.de_loading += 1; self.de_loading_anim="[=====>]"},
                                121..=150 => {self.de_loading += 1; self.de_loading_anim="[====>-]"},
                                91..=120 => {self.de_loading += 1; self.de_loading_anim="[===>--]"},
                                61..=90 => {self.de_loading += 1; self.de_loading_anim="[==>---]"},
                                30..=60 => {self.de_loading += 1; self.de_loading_anim="[=>----]"},
                                _ => {self.de_loading += 1; self.de_loading_anim="[>-----]"},
                            }  
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            self.de_czy_to_juz_koniec = 4;
                            self.szyfrowanie_err_3 = "Disconected".to_string();
                        }
                    }

                    // let deszyfr_but_col = if sprawdzacz_plikow_dekompresja{Color32::DARK_GREEN}else{szarawy_ciemny};
                    let deszyfr_butt_col = if sprawdzacz_plikow_dekompresja == true && self.de_czy_to_juz_koniec ==0{Color32::DARK_GREEN}else if sprawdzacz_plikow_dekompresja == true && self.de_czy_to_juz_koniec ==1 {zolty_ciemny} else {szarawy_ciemny};
                    ui.vertical_centered(|ui|{

                        if ui.add(egui::Button::new(de_t_p_d
                        .size(cd)
                        .color(Color32::GRAY))
                        .min_size(egui::vec2(250.0, 40.0))
                        .corner_radius(10.)
                        .fill(deszyfr_butt_col))
                        .clicked() {

                            if sprawdzacz_plikow_dekompresja 
                            {
                                self.de_czy_to_juz_koniec = 0;

                                let dat_file = Path::new(&self.imput_de_dat_folder_path).to_path_buf(); 
                                let idx_file = Path::new(&self.imput_de_idx_folder_path).to_path_buf(); 
                                let output_folder = Path::new(&self.output_de_folder_path).to_path_buf(); 
                                let arc_z_str = Arc::new(Mutex::new(vec![self.de_password.clone().to_string()]));
                                let arc_z_path = Arc::new(Mutex::new(vec![dat_file.clone(),idx_file.clone(),output_folder.clone()]));
                                let arc_z_u8 = Arc::new(Mutex::new(vec![self.toggle_encryption.clone()]));

                                let de_tx_clone = self.de_tx.clone();
                                std::thread::spawn(move || {
                                    let de_result = decrypt_copy::decrypt_files(
                                                arc_z_path,
                                                arc_z_str,
                                                arc_z_u8,
                                            );
                                        
                                            match de_tx_clone.send(de_result) {
                                                Ok(_) => println!("Wysłano wynik"),
                                                Err(e) => eprintln!("Błąd wysyłania: {}", e),
                                            }
                                        });
                                        
                                        self.de_czy_to_juz_koniec = 1;
    
                                    }}
    
                                                
    
                                });

                    ui.add_space(ss);

                    ui.separator();

                    ui.add_space(ss);

                    //        _         _       _       _             
                    //  ___ _| |___ ___| |_ ___| |_ ___|_|___ ___ ___ 
                    // | .'| . |   | . |  _| .'|  _|   | | . |   |_ -|
                    // |__,|___|_|_|___|_| |__,|_| |_|_|_|___|_|_|___|

                    ui.add(egui::Label::new(RichText::new("CTRL + E for aditional menu").size(cm)).selectable(false));





            
                }); // zamknięcie scroll area
                
            }); //zamkniecie left panel


            // ----------------------------------------------------------------------------------------------------------------------------------------
            // ██████  ██  ██████  ██   ██ ████████      
            // ██   ██ ██ ██       ██   ██    ██         
            // ██████  ██ ██   ███ ███████    ██         
            // ██   ██ ██ ██    ██ ██   ██    ██         
            // ██   ██ ██  ██████  ██   ██    ██         
                                                      
                                                      
            // ██████   █████  ███    ██ ███████ ██      
            // ██   ██ ██   ██ ████   ██ ██      ██      
            // ██████  ███████ ██ ██  ██ █████   ██      
            // ██      ██   ██ ██  ██ ██ ██      ██      
            // ██      ██   ██ ██   ████ ███████ ███████

            egui::SidePanel::right("halp_menu")
            .exact_width(350.)
            .resizable(false)
            .show(ctx,|ui|{
                ui.add_space(ss);
                ui.columns(if self.show_debug_labels{3}else{2},|columns|{


                    columns[0].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienia_menu, 0, "info");
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienia_menu, 1, "ustawienia");
                    });
                    if self.show_debug_labels{
                        columns[2].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.ustawienia_menu, 2, "debug");
                        });
                    }

                });

                ui.add_space(5.);

                ui.separator();
                if self.ustawienia_menu == 0{
                    ui.columns(5,|columns|{
                        columns[0].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 0, "Info");
                        });
                        columns[1].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 1, "Stats");
                        });
                        columns[2].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 2, "pakowanie");
                        });
                        columns[3].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 3, "rozpakowywanie");
                        });
                        columns[4].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 4, "filtrowanie");
                        });
                    });
                }

                egui::ScrollArea::vertical()
                .show(ui, |ui| {

                    if self.ustawienia_menu==0{

                        if self.toggle_halp ==0 {
                            // let kolor_komunikatu = if !self.toggle_halp_komunikat{ Color32::RED } else { Color32::GRAY }; // Domyślny kolor

                            ui.separator();

                            ui.add(egui::Label::new(RichText::new("wiadomość TODO!".to_string())));
                            
                            // // Wyświetlanie tekstu z aktualnym kolorem
                            // ui.add(egui::Label::new(if self.nie_krzycz_iter<5{
                            //     RichText::new("(już nie) ZACINA PROGRAM(u) PODCZAS WYKONYWANIA FUNKCJI, \nTAK MA BYĆ! \nBĘDZIE ZMIANA JAK BĘDZIE POTRZEBA, \nNA RAZIE DZIAŁA, \nA JAK DZIAŁA TO NIE TYKAĆ!!!!!")
                            // }else{
                            //     RichText::new("Программа зависает во время выполнения функции, \nТак и должно быть! \nБудет изменение, если потребуется, \nПока работает, \nА если работает, не трогать!!!!!")
                            //     // RichText::new("プログラムは関数の実行中にフリーズします、\nそれで問題ありません！\n必要な場合に変更がありますが、\n現時点では動作しています、\n動作しているなら触らないでください！！！").font(egui::FontId::new(16.0, egui::FontFamily::Name("japanese_font".to_owned().into())))
                            // }
                            //     .size(cd * self.nie_krzycz_zmniejszacz)
                            //     .color(kolor_komunikatu))
                            //     .selectable(false));
                            
                            // // Przycisk, który zmienia kolor po kliknięciu
                            // if ui.button("Rozumim, nie krzyczaj").clicked() {
                            //     // Zmiana koloru po kliknięciu przycisku
                            //     self.toggle_halp_komunikat = true;
                            //     self.nie_krzycz_zmniejszacz = self.nie_krzycz_zmniejszacz * 0.9;
                            //     self.nie_krzycz_iter -= 1
                            // }
                            // if ui.button("Nie rozumim, krzyczaj!").clicked() {
                            //     // Zmiana koloru po kliknięciu przycisku
                            //     self.toggle_halp_komunikat = false;
                            //     self.nie_krzycz_zmniejszacz = self.nie_krzycz_zmniejszacz * 1.1;
                            //     self.nie_krzycz_iter += 1
                            // }
                            
                            ui.add(egui::Image::new(egui::include_image!("br/ezgif-762e334d60199c.gif")).sense(egui::Sense::click())
                            .max_height(330.0)
                            .max_width(300.0)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(1.)
                            // .rounding(10.0));
                            
                            );
                        } else if  self.toggle_halp == 1 {
                            let tekst_plików=
                                match &self.przetworzone_pliki % 10 {
                                    1 if &self.przetworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&self.przetworzone_pliki % 100 < 10) || (&self.przetworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                                let de_tekst_plików=
                                match &self.de_przetworzone_pliki % 10 {
                                    1 if &self.de_przetworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&self.de_przetworzone_pliki % 100 < 10) || (&self.przetworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                            ui.separator();
                            ui.add_space(ss);
                            ui.add(egui::Label::new(format!("Spakowano {} {} w czasie: {}.{}s",self.przetworzone_pliki,tekst_plików, self.timestamp_sekundy, &self.timestamp_milisekundy)).selectable(false));

                            ui.add_space(ss);
                            ui.add(egui::Label::new(format!("Rozpakowano {} {} w czasie: {}.{}s",self.de_przetworzone_pliki,de_tekst_plików, self.de_timestamp_sekundy, &self.de_timestamp_milisekundy)).selectable(false));

                            
                        
                        } else if self.toggle_halp == 2 {
                            ui.add(egui::Label::new("Lorem Ipsum, myślisz, że robisz wrażenie,
        Wszystko to bełkot, zero treści, to tylko pozorne istnienie.
        Kopiujesz, wklejasz, ale gdzie jest sens?
        Puste słowa jak piosenka, której nie chcesz słyszeć więcej.

        Nie ma tu nic oryginalnego, tylko ślepa repetetycja,
        Cały twój przekaz to sztuczna projekcja.
        Więc nie bądź dumny, bo w świecie tekstów masz już dość,
        Jak pusty filler – tylko niepotrzebny hałas wśród treści, która ma moc!”").selectable(false));
                        }
                        else if self.toggle_halp == 3 {
                            ui.add(egui::Label::new("Lorem Ipsum, myślisz, że robisz wrażenie,
        Wszystko to bełkot, zero treści, to tylko pozorne istnienie.
        Kopiujesz, wklejasz, ale gdzie jest sens?
        Puste słowa jak piosenka, której nie chcesz słyszeć więcej.

        Nie ma tu nic oryginalnego, tylko ślepa repetetycja,
        Cały twój przekaz to sztuczna projekcja.
        Więc nie bądź dumny, bo w świecie tekstów masz już dość,
        Jak pusty filler – tylko niepotrzebny hałas wśród treści, która ma moc!”").selectable(false));
                        }
                        else if self.toggle_halp == 4 {
                            ui.add(egui::Label::new("Brak:").selectable(false));
                            ui.add(egui::Label::new("wszystkie pliki są pakowane").selectable(false));
                            ui.add_space(ss);
                            ui.add(egui::Label::new("Assets:").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("rodzaj pliku, nazwa, wariant, podwariant, ID, ścieżka, file_size, offset").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("rodzaj pliku - generowane na podstawie rozszerzenia").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("nazwa - prefiks w nazwie pliku, określa ogólną nazwę assetu").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("wariant - wartość pomiędzy _X_ po nazwie, określa wariant").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("podwariant - wartość pomiędzy _X_ po wariancie, określa podwariant").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("ID - unikalne id generowane dla każdego pliku").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("ścieżka - oryginalna lokalna ścieżka do pliku wraz z nazwą").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("file_size - rozmiar danego pliku").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("offset - przesunięcie początku pliku").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("aktualne rodzaje plików:").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("Model_3D").selectable(false));
                            ui.add(egui::Label::new("glb | obj | 3mf | fbx | stl | dae | ply | x3d | 3ds | max | usd | usdz | c4d | ac | vmd | lwo | abc").selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("Image").selectable(false));
                            ui.add(egui::Label::new(        "jpg | png  | exr  | tga  | dds | ico | tiff | bmp  | jpeg | gif | tif | webp | heif | heic | raw" ).selectable(false));
                            ui.add_space(bs);
                            ui.add(egui::Label::new("DOKOŃCZYĆ!").selectable(false));
                        
                        }
                    //zakladka info 
                    } else if self.ustawienia_menu ==1 { 
                        ui.add_space(10.);
                        ui.vertical_centered_justified(|ui|{
                            ui.checkbox(&mut self.debug_create_lua_file, &self.current_language.create_lua_file.to_string());
                        });

                        ui.add_space(ss);

                        ui.columns(2,|columns|{
                            columns[0].vertical_centered_justified(|ui|{
                                ui.add(egui::Label::new("Głośność").selectable(false));
                            });
                            columns[1].vertical_centered_justified(|ui|{
                                ui.add(egui::Slider::new(&mut self.ustawienia_glosnosc, 0f32..=1.).text(""));
                            });
                        });


                    //zakladka ustawienia
                    } else if self.ustawienia_menu==2 {
                        if self.kllikker < 50{
                            let to_je_foto = ui.add(egui::Image::new(egui::include_image!("br/ok.png")).sense(egui::Sense::click())
                            .max_height(330.0)
                            .max_width(300.0)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(1.)
                            .rotate( self.rotation_image, egui::Vec2::splat(0.5))
                            // .rounding(10.0));
                            
                            );
                            if to_je_foto.clicked() {
                                crate::ui_play_sound::play_ahh_sound(self.ustawienia_glosnosc);
                                self.rotation_image = self.rotation_image + rand::random_range(0.0..=PI);
                                self.kllikker = self.kllikker+1;
                                
                            }
                        }else{
                            let to_je_foto = ui.add(egui::Image::new(egui::include_image!("br/chunky-cat-out.gif")).sense(egui::Sense::click())
                            .max_height(330.0)
                            .max_width(300.0)
                            .maintain_aspect_ratio(true)
                            .fit_to_original_size(1.)
                            .rotate( self.rotation_image, egui::Vec2::splat(0.5))
                            // .rounding(10.0));
                            
                            );

                            if to_je_foto.clicked() {
                                crate::ui_play_sound::play_ahh_sound(self.ustawienia_glosnosc);
                                self.rotation_image = self.rotation_image + rand::random_range(0.0..=PI);
                                self.rotation_image = self.rotation_image % (2.0 * PI); 
                                self.kllikker = self.kllikker+1;
                                
                            }
                        }

                        ui.add(egui::Label::new(self.kllikker.to_string()).selectable(false));


                            //  _                   
                            // |_|_____ ___ ___ ___ 
                            // | |     | .'| . | -_|
                            // |_|_|_|_|__,|_  |___|
                            //             |___|   
    
    
    

    
                            ui.add(egui::Label::new(self.current_language.ukryte_sktory).selectable(false));
    
                            ui.add_space(10.);
    
                            ui.separator();
    
                            ui.add(egui::Label::new(RichText::new(&self.current_language.debug_deszyfracja_idx.to_string()).strikethrough()).selectable(false));
    
                            ui.add(egui::Label::new(RichText::new(&self.current_language.deszyfrowanie_plik_idx.to_string()).strikethrough()).selectable(false));
    
                            ui.horizontal(|ui|{
    
                                ui.add_space(20.);
    
                                let btn_debug_plik_idx = ui.button(&self.current_language.deszyfrowanie_plik_idx.to_string());
                                if btn_debug_plik_idx.clicked() {
    
                                    if let Some(path) = FileDialog::new().pick_file() {
                                        self.debug_imput_idx_file = path.to_string_lossy().to_string();
                                    }
                                }
    
                                let btn_pozycjonowanie_debug_plik_idx = Pos2::new(
                                    btn_debug_plik_idx.rect.min.x - offset_cirkul,
                                    btn_debug_plik_idx.rect.min.y+(btn_debug_plik_idx.rect.size().y / 2.));
                                
                                if !self.debug_imput_idx_file.is_empty(){
            
                                    ui.painter().circle_filled(btn_pozycjonowanie_debug_plik_idx, full_cirkul_sajz, full_cirkul_kolor);
            
                                } else {
            
                                    ui.painter().circle_stroke(btn_pozycjonowanie_debug_plik_idx, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));
            
                                }
    
                                ui.add(egui::Label::new(&self.debug_imput_idx_file).selectable(false));
    
                            });
    
                            ui.add(egui::Label::new(&self.current_language.wybierz_folder_wyjsciowy.to_string()).selectable(false));
    
                            ui.horizontal(|ui|{
    
                                ui.add_space(20.);
    
                                let btn_debug_folder_wyjsciowy = ui.button(&self.current_language.wybierz_folder.to_string());
                                if btn_debug_folder_wyjsciowy.clicked() {
    
                                    if let Some(path) = FileDialog::new().pick_folder() {
                                        self.debug_output_idx_path = path.to_string_lossy().to_string()+"/";
                                    }
    
                                }
                                if self.debug_output_idx_path.len() < 28{
    
                                    ui.add(egui::Label::new(&self.debug_output_idx_path).selectable(false));
    
                                }
    
                                let btn_pozycjonowanie_debug_folder_wyjsciowy = Pos2::new(
                                    btn_debug_folder_wyjsciowy.rect.min.x - offset_cirkul,
                                    btn_debug_folder_wyjsciowy.rect.min.y+(btn_debug_folder_wyjsciowy.rect.size().y / 2.));
                                
                                if !self.debug_output_idx_path.is_empty(){
            
                                    ui.painter().circle_filled(btn_pozycjonowanie_debug_folder_wyjsciowy, full_cirkul_sajz, full_cirkul_kolor);
            
                                } else {
            
                                    ui.painter().circle_stroke(btn_pozycjonowanie_debug_folder_wyjsciowy, pusty_cirkul_sajz, (pusty_cirkul_ma_stroke,pusty_cirkul_kolor));
            
                                }
                                
    
                            });
    
                            if self.debug_output_idx_path.len() >= 28{
    
                                ui.add(egui::Label::new(&self.debug_output_idx_path).selectable(false));
    
                            }
    
                            if self.toggle_encryption == 0{
    
                                ui.add_space(9.);
                                ui.vertical_centered(|ui|{
    
    
                                    ui.add(egui::Label::new(RichText::new(&self.current_language.szyfrowanie_haslo_wylaczone.to_string()).color(fiolet_ciemny)).selectable(false));
    
                                });
    
                                ui.add_space(9.);
    
                            }else{
    
                                ui.add(egui::Label::new(&self.current_language.szyfrowanie_haslo_tytul.to_string()).selectable(false));
                                let password_input = TextEdit::singleline(&mut self.debug_idx_password)
                                    .password(true)
                                    .hint_text(&self.current_language.szyfrowanie_haslo.to_string());
                                ui.add(password_input);
    
                            };
                            let adv_idx_deszyfr = false;
                            ui.vertical_centered(|ui|{
                            if adv_idx_deszyfr{
                                if ui.button("Deszyfruj IDX").clicked() {
    
                                    if self.debug_output_idx_path.is_empty() ||self.debug_output_idx_path.is_empty() {
    
                                        eprintln!("[Ui/deszyfruj/idx_button :: LocalTime:{}]\n---> Brak wymaganych danych do deszyfrowania!\n", Local::now().format("%H:%M:%S"));
    
                                    } else {
    
                                        let debug_idx_file = Path::new(&self.debug_imput_idx_file);
                                        let debug_output_folder = Path::new(&self.debug_output_idx_path);
                    
                                        match decrypt_copy::decrypt_idx_to_text_file(&debug_idx_file, &debug_output_folder,  &self.debug_idx_password, self.toggle_encryption) {
                                            Ok(_) => println!("Plik odszyfrowany pomyślnie!"),
                                            Err(e) => eprintln!("Błąd deszyfrowania: {}", e),
                                        }
    
                                    }
    
                                } // zamknięcie button
                            } else {
                                ui.add(egui::Label::new(RichText::new("Deszyfruj IDX").strikethrough()).selectable(false));
                            }
                            });
                        

                    //zakladka debug
                    }
                    
                })
            });//zamkniecie right panel


        }); // zamknięcie central_panel
        if self.settings_toggle{
            egui::TopBottomPanel::top("settings").show(ctx, |ui| {
                ui.vertical_centered(|ui|{
                    ui.add(egui::Label::new(RichText::new(&self.current_language.ustawienia_tytul.to_string()).size(cd)).selectable(false));
            });
                ui.add(egui::Label::new(&self.current_language.ustawienia_skrot.to_string()).selectable(false));
            });
        }    




    } // zamknięcie fn update

} // zamknięcie Appencja




