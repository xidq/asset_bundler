use egui::{Color32/*,FontId, FontFamily*/};

use crate::ui::ui_language;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

pub struct Appencja {

  pub imput_r_file_path: PathBuf,
  pub imput_g_file_path: PathBuf,
  pub imput_b_file_path: PathBuf,
  pub imput_out_folder_path: PathBuf,
  pub imput_folder_path: String,
  pub imput_foty_folder_path: String,
  pub output_foty_folder_path: String,
  pub output_folder_path: String,
  pub imput_de_dat_folder_path: String,
  pub imput_de_idx_folder_path: String,
  pub output_de_folder_path: String,
  pub output_name: String,

  pub password: String,
  pub de_password: String,
  pub template: String,
  pub show_debug_labels: bool,
  pub debug_imput_idx_file:String,
  pub debug_output_idx_path:String,
  pub debug_idx_password:String,
  pub debug_create_lua_file: bool,
  pub toggle_encryption: u8,
  pub toggle_de_encryption: u8,
  pub toggle_language:u8,
  pub current_language: ui_language::Language,
  pub brainroot: bool,
  pub toggle_compression: u8,
  pub poziom_kompresji:u8,
  //pub // rozszerzenie_pliku:&'static str,
  pub kllikker:i32,
  pub rotation_image:f32,

  pub zapis_pracuje:bool,
  pub czy_to_juz_koniec:u8,
  pub czy_to_juz_koniec_foto:u8,
  pub de_czy_to_juz_koniec:u8,

  pub settings_toggle:bool,
  pub wysokosc_btn_egzekucyjny:f32,
  pub szerokosc_okna:f32,


  pub szyfrowanie_err_3:String,
  // pub szyfrowanie_err_4:String,


//    pub // encryption_result: Option<Result<Arc<Mutex<Vec<usize>>>, String>>,
//    pub // encryption_rx: Option<mpsc::Receiver<Result<Arc<Mutex<Vec<usize>>>, String>>>,
  pub tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

  pub de_tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub de_rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

  pub fot_tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub fot_rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

  pub fotx_tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub fotx_rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,

  pub sz_loading: u8,
  pub sz_loading_anim: &'static str,
  pub de_loading_anim: &'static str,
  pub de_loading: u8,
  pub wybor_czcionki: u8,

  pub timestamp_sekundy:u64,
  pub timestamp_milisekundy:u32,
  pub przetworzone_pliki:usize,
  pub de_timestamp_sekundy:u64,
  pub de_timestamp_milisekundy:u32,
  pub de_przetworzone_pliki:usize,

  pub foto_timestamp_sekundy:u64,
  pub foto_timestamp_milisekundy:u32,
  pub foto_przetworzone_pliki:usize,
  pub foto_utworzone_pliki:usize,

  pub foto_laczenie_timestamp_sekundy:u64,
  pub foto_laczenie_timestamp_milisekundy:u32,


  pub ustawienie_lewy_panel_foty_szyfrowanie: bool,
  pub ustawienie_lewy_panel_szyfrowanie_podopcje:u8,
  pub ustawienie_lewy_panel_foty_podopcje:u8,

  pub ustawienia_menu:u8,
  pub toggle_halp:u8,
  // pub toggle_halp_komunikat:bool,
  // pub nie_krzycz_zmniejszacz:f32,
  // pub nie_krzycz_iter:i32,
  pub ustawienia_glosnosc:f32,


  pub foty_czy_alpha:u8,
  pub foty_alpha_kolor:u8,

  pub foty_filter:u8,


  pub foty_format_jpg:bool,
  pub foty_format_png:bool,
  pub foty_format_png_16:bool,
  pub foty_format_webp:bool,
  pub foty_format_webp_lossy:bool,
  pub foty_format_tga:bool,
  // pub foty_format_heic:bool,
  pub foty_format_png_filter:u8,

  pub foty_format_jakosc_jpg:u8,
  pub foty_format_jakosc_png:u8,
  // pub foty_format_jakosc_png_16:u8,
  pub foty_format_jakosc_webp:u8,
  pub foty_format_jakosc_webp_lossy:u8,
  pub foty_format_jakosc_tga:u8,
  // pub foty_format_jakosc_heic:u8,

  pub foty_rozdzielczosc_16k:bool,
  pub foty_rozdzielczosc_8k:bool,
  pub foty_rozdzielczosc_4k:bool,
  pub foty_rozdzielczosc_2k:bool,
  pub foty_rozdzielczosc_1k:bool,
  pub foty_rozdzielczosc_512:bool,
  pub foty_rozdzielczosc_256:bool,
  pub foty_rozdzielczosc_128:bool,
  pub foty_rozdzielczosc_64:bool,
  pub foty_rozdzielczosc_32:bool,
  // pub foty_rozdzielczosc_16:bool,
  // pub foty_rozdzielczosc_8:bool,


  //formatowanie
  pub formatowanie_rozmiar_czcionki_mala:f32,
  pub formatowanie_rozmiar_czcionki_srednia:f32,
  pub formatowanie_rozmiar_czcionki_duza:f32,

  pub formatowanie_spacja_mala:f32,
  pub formatowanie_spacja_srednia:f32,
  pub formatowanie_spacja_duza:f32,

  //kółka
  pub full_cirkul_sajz:f32,
  pub full_cirkul_kolor: Color32,
  pub pusty_cirkul_sajz:f32,
  pub pusty_cirkul_kolor:Color32,
  pub pusty_cirkul_ma_stroke: f32,
  pub offset_cirkul: f32,
  







  //IMAGE PAKOWANIE W JEDEN
  pub pakowanie_foty_bit_depth:u8,
  pub pakowanie_foty_rozszerzenie:u8,
  pub pakowanie_foty_czy_to_juz_koniec:u8,
  pub pakowanie_foty_rozdzielczosc:u8,
  pub pakowanie_foty_filter:u8,
  pub pakowanie_foty_nazwa:String,
  pub pakowanie_foty_jakosc:u8,
  pub pakowanie_foty_png_filter:u8,
  pub pakowanie_foty_red_aspect_ratio:f32,
  pub pakowanie_foty_green_aspect_ratio:f32,
  pub pakowanie_foty_blue_aspect_ratio:f32,
}


// -------------------------------------------------------------------------------------------------------------------------------------------
// ██████  ███████ ███████  █████  ██    ██ ██     ████████ ███████ 
// ██   ██ ██      ██      ██   ██ ██    ██ ██        ██    ██     
// ██   ██ █████   █████   ███████ ██    ██ ██        ██    ███████ 
// ██   ██ ██      ██      ██   ██ ██    ██ ██        ██         ██ 
// ██████  ███████ ██      ██   ██  ██████  ███████   ██    ███████

impl Default for Appencja {

    fn default() -> Self {
      let (tx, rx) = mpsc::channel();
      let (de_tx, de_rx) = mpsc::channel();
      let (fot_tx, fot_rx) = mpsc::channel();
      let (fotx_tx, fotx_rx) = mpsc::channel();
      Appencja {
        imput_r_file_path: PathBuf::new(),
        imput_g_file_path: PathBuf::new(),
        imput_b_file_path: PathBuf::new(),
        imput_out_folder_path: PathBuf::new(),
        imput_folder_path: String::from(""),
        imput_foty_folder_path: String::from(""),
        output_foty_folder_path: String::from(""),
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
        toggle_language:0,
        current_language: ui_language::Language::polish(),
        brainroot:false,
        toggle_compression:0,
        poziom_kompresji:3,
        kllikker:0,
        rotation_image:0.0,

        wybor_czcionki: 0,
        wysokosc_btn_egzekucyjny:803.,
        szerokosc_okna:740.,
        // rozszerzenie_pliku: "dat",
        tx,
        rx,
        de_tx,
        de_rx,
        fot_tx,
        fot_rx,
        fotx_tx,
        fotx_rx,
        sz_loading: 0,
        de_loading: 0,

        sz_loading_anim: "",
        de_loading_anim: "",

        zapis_pracuje: false,
        czy_to_juz_koniec:0,
        czy_to_juz_koniec_foto:0,
        de_czy_to_juz_koniec:0,

        settings_toggle:false,


        szyfrowanie_err_3:"".to_string(),
        // szyfrowanie_err_4:"".to_string(),

        // encryption_result: None,
        // encryption_rx: None,

        przetworzone_pliki:0,
        timestamp_sekundy:0,
        timestamp_milisekundy:0,
        de_timestamp_sekundy:0,
        de_timestamp_milisekundy:0,
        de_przetworzone_pliki:0,
        foto_timestamp_sekundy:0,
        foto_timestamp_milisekundy:0,
        foto_przetworzone_pliki:0,
        foto_utworzone_pliki:0,
        foto_laczenie_timestamp_sekundy:0,
        foto_laczenie_timestamp_milisekundy:0,

        ustawienie_lewy_panel_foty_szyfrowanie:false,
        ustawienie_lewy_panel_szyfrowanie_podopcje:0,
        ustawienie_lewy_panel_foty_podopcje:0,

        //kliki
        ustawienia_menu:0,
        toggle_halp:0,
        // toggle_halp_komunikat:false,
        // nie_krzycz_zmniejszacz:1.,
        // nie_krzycz_iter:0,
        ustawienia_glosnosc:1.,

        foty_czy_alpha:0,
        foty_alpha_kolor:0,
        foty_filter:4,

        foty_format_jpg:false,
        foty_format_png:false,
        foty_format_png_16:false,
        foty_format_webp:false,
        foty_format_webp_lossy:false,
        foty_format_tga:false,
        // foty_format_heic:false,
        foty_format_png_filter:5,

        foty_format_jakosc_jpg:75,
        foty_format_jakosc_png:2,
        // foty_format_jakosc_png_16:2,
        foty_format_jakosc_webp:75,
        foty_format_jakosc_webp_lossy:75,
        foty_format_jakosc_tga:75,
        // foty_format_jakosc_heic:75,
    

        foty_rozdzielczosc_16k:false,
        foty_rozdzielczosc_8k:false,
        foty_rozdzielczosc_4k:false,
        foty_rozdzielczosc_2k:false,
        foty_rozdzielczosc_1k:false,
        foty_rozdzielczosc_512:false,
        foty_rozdzielczosc_256:false,
        foty_rozdzielczosc_128:false,
        foty_rozdzielczosc_64:false,
        foty_rozdzielczosc_32:false,
        // foty_rozdzielczosc_16:false,
        // foty_rozdzielczosc_8:false,

        //formatowanie
        formatowanie_rozmiar_czcionki_mala:10.,
        formatowanie_rozmiar_czcionki_srednia:14.,
        formatowanie_rozmiar_czcionki_duza:18.,

        formatowanie_spacja_mala:5.,
        formatowanie_spacja_srednia:10.,
        formatowanie_spacja_duza:20.,

        //kółka
        full_cirkul_sajz:7.,
        full_cirkul_kolor: Color32::GREEN,
        pusty_cirkul_sajz:5.,
        pusty_cirkul_kolor: Color32::GRAY,
        pusty_cirkul_ma_stroke:1.5,
        offset_cirkul:14.,



          //IMAGE PAKOWANIE W JEDEN
        pakowanie_foty_bit_depth:8,
        pakowanie_foty_rozszerzenie:0,
        pakowanie_foty_czy_to_juz_koniec:0,
        pakowanie_foty_rozdzielczosc:4,
        pakowanie_foty_filter:4,
        pakowanie_foty_nazwa:"".to_string(),
        pakowanie_foty_jakosc:75,
        pakowanie_foty_png_filter:5,
        pakowanie_foty_red_aspect_ratio:1.,
        pakowanie_foty_green_aspect_ratio:1.,
        pakowanie_foty_blue_aspect_ratio:1.,
      }

    }
    
}