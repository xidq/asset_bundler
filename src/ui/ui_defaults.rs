
use egui::Color32;
use crate::ui::ui_language;
use std::{
  path::PathBuf,
  sync::{mpsc,Arc}
  // , time::Duration,
};



pub struct Appencja {


  //wartości ogólne:
  pub tx: mpsc::Sender<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub rx: mpsc::Receiver<Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error>>,
  pub current_language: ui_language::Language,
  pub general_ui_przelacz_jezyk:u8,
  pub general_ui_loading: u16,
  pub general_ui_loading_tekst:&'static str,
  // pub general_ui_status_przetwarzania:u8,
  pub general_ui_szerokosc_okna:f32,
  pub general_ui_przelacz_tryb_debug: bool,
  #[cfg(feature = "statystyki")]
  pub general_ui_licznik_czasu_debug: u8,
  pub general_ui_przelacz_binarny_zdjecia: bool,
  pub general_ui_przelacz_binarny_opcje:u8,
  pub general_ui_przelacz_zdjecia_opcje:u8,
  pub general_ui_przelacz_brainroot: bool,
  pub general_ui_przelacz_ustawienia:bool,
  pub general_ui_przelacz_info_ustawienia:u8,
  pub general_ui_przelacz_info_opcje:u8,
  #[cfg(feature = "statystyki")]
  pub general_ui_statystyki_klatki:Vec<(u128,u128)>,


  //debug menu specyfic
  pub ui_debug_specyfic_klikacz:i32,
  pub ui_debug_specyfic_obrot:f32,


  //wartości formatowanie:
  //rozmiary i czcionki
  pub formatowanie_wybor_czcionki: u8,
  pub formatowanie_rozmiar_czcionki_mala:f32,
  pub formatowanie_rozmiar_czcionki_srednia:f32,
  pub formatowanie_rozmiar_czcionki_duza:f32,
  pub formatowanie_wysokosc_przycisku:f32,
  //kółko
  pub formatowanie_rozmiar_kolko:f32,
  pub formatowanie_rozmiar_kolko_puste:f32,
  pub formatowanie_rozmiar_kolko_puste_stroke: f32,
  pub formatowanie_kolor_kolko_puste:Color32,
  pub formatowanie_offset_kolko: f32,
  //odstępy
  pub formatowanie_spacja_mala:f32,
  pub formatowanie_spacja_srednia:f32,
  pub formatowanie_spacja_duza:f32,
  //kolory
  pub formatowanie_kolor_kolko_pelne: Color32,


  //wartości specyficzne dla ui pakowania:
  pub ui_pack_specyfic_password: String,
  pub ui_pack_specyfic_template: String,
  pub ui_pack_specyfic_folder_wejsciowy: String,
  pub ui_pack_specyfic_folder_wyjsciowy: String,
  pub ui_pack_specyfic_nazwa_pliku: String,
  pub ui_pack_specyfic_wybor_kompresji: u8,
  pub ui_pack_specyfic_poziom_kompresji: u8,
  pub ui_pack_specyfic_wybor_szyfrowania: u8,
  pub ui_pack_specyfic_statystyki_przetworzone_pliki: usize,
  pub ui_pack_specyfic_statystyki_czas_sekundy:u64,
  pub ui_pack_specyfic_statystyki_czas_milisekundy:u32,
  pub ui_pack_specyfic_error_3:String,
  pub ui_pack_specyfic_status_przetwarzania:u8,
  pub ui_pack_specyfic_statyczne_id:bool,


  //wartości specyficzne dla ui rozpakowywania:
  pub ui_unpack_specyfic_sciezka_plik_binarny: String,
  pub ui_unpack_specyfic_sciezka_plik_indeksu: String,
  pub ui_unpack_specyfic_sciezka_folder_wyjsciowy: String,
  pub ui_unpack_specyfic_zmiana_szyfrowania: u8,
  pub ui_unpack_specyfic_password: String,
  pub ui_unpack_specyfic_statystyki_przetworzone_pliki: usize,
  pub ui_unpack_specyfic_statystyki_czas_sekundy: u64,
  pub ui_unpack_specyfic_statystyki_czas_milisekundy: u32,
  pub ui_unpack_specyfic_error_3:String,
  pub ui_unpack_specyfic_status_przetwarzania:u8,


  //wartości specyficzne dla ui łączenia rgb:
  pub ui_laczenie_specyfic_sciezka_plik_czerwony: PathBuf,
  pub ui_laczenie_specyfic_sciezka_plik_zielony: PathBuf,
  pub ui_laczenie_specyfic_sciezka_plik_niebieski: PathBuf,
  pub ui_laczenie_specyfic_sciezka_folder_wyjsciowy: PathBuf,
  pub ui_laczenie_specyfic_stosunek_czerwony:f32,
  pub ui_laczenie_specyfic_stosunek_zielony:f32,
  pub ui_laczenie_specyfic_stosunek_niebieski:f32,
  pub ui_laczenie_specyfic_dane_rozszerzenia:u8,
  pub ui_laczenie_specyfic_statystyki_czas_sekundy:u64,
  pub ui_laczenie_specyfic_statystyki_czas_milisekundy:u32,
  pub ui_laczenie_specyfic_nazwa_pliku:String,
  pub ui_laczenie_specyfic_dane_filter:u8,
  pub ui_laczenie_specyfic_dane_rozdzielczosc:u8,
  pub ui_laczenie_specyfic_dane_bity:u8,
  pub ui_laczenie_specyfic_dane_filter_png:u8,
  pub ui_laczenie_specyfic_dane_jakosc:u8,
  pub ui_laczenie_specyfic_error_3:String,
  pub ui_laczenie_specyfic_wysokość_czerwony:u32,
  pub ui_laczenie_specyfic_wysokość_zielony:u32,
  pub ui_laczenie_specyfic_wysokość_niebieski:u32,
  pub ui_laczenie_specyfic_szerokość_czerwony:u32,
  pub ui_laczenie_specyfic_szerokość_zielony:u32,
  pub ui_laczenie_specyfic_szerokość_niebieski:u32,
  pub ui_laczenie_specyfic_status_przetwarzania:u8,

  //wartosci specyficzne dla ui konwersji zdjęć
  pub ui_konwersja_specyfic_sciezka_folder_wejsciowy: String,
  pub ui_konwersja_specyfic_sciezka_folder_wyjsciowy: String,
  pub ui_konwersja_specyfic_dane_alpha_kolor:u8,
  pub ui_konwersja_specyfic_dane_alpha:u8,
  pub ui_konwersja_specyfic_dane_filter:u8,
  pub ui_konwersja_specyfic_dane_bool_jpg:bool,
  pub ui_konwersja_specyfic_dane_bool_png:bool,
  pub ui_konwersja_specyfic_dane_bool_png_16:bool,
  pub ui_konwersja_specyfic_dane_bool_webp_lossless:bool,
  pub ui_konwersja_specyfic_dane_bool_webp_lossy:bool,
  pub ui_konwersja_specyfic_dane_bool_tga:bool,
  pub ui_konwersja_specyfic_dane_jakosc_jpg:u8,
  pub ui_konwersja_specyfic_dane_jakosc_png:u8,
  pub ui_konwersja_specyfic_dane_jakosc_webp_lossless:u8,
  pub ui_konwersja_specyfic_dane_jakosc_webp_lossy:u8,
  pub ui_konwersja_specyfic_dane_jakosc_tga:u8,
  pub ui_konwersja_specyfic_dane_filter_png:u8,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_16k:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_8k:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_4k:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_2k:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_1k:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_512:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_256:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_128:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_64:bool,
  pub ui_konwersja_specyfic_dane_rozdzielczosc_32:bool,
  pub ui_konwersja_specyfic_statystyki_czas_sekundy:u64,
  pub ui_konwersja_specyfic_statystyki_czas_milisekundy:u32,
  pub ui_konwersja_specyfic_statystyki_przetworzone_pliki:usize,
  pub ui_konwersja_specyfic_statystyki_utworzone_pliki:usize,
  pub ui_konwersja_specyfic_error_3:String,
  pub ui_konwersja_specyfic_status_przetwarzania:u8,


  //Ustawienia
  pub ui_ustawienia_tworzenie_pliku_lua: bool,
  pub ui_ustawienia_glosnosc:f32,

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

      Appencja {

        //wartości ogólne:
        tx,
        rx,
        current_language: ui_language::Language::polish(),
        general_ui_przelacz_jezyk:0,
        general_ui_loading: 0,
        general_ui_loading_tekst: "",
        // general_ui_status_przetwarzania:0,
        general_ui_szerokosc_okna:740.,
        general_ui_przelacz_tryb_debug: false,
        #[cfg(feature = "statystyki")]
        general_ui_licznik_czasu_debug:0,
        general_ui_przelacz_binarny_zdjecia:false,
        general_ui_przelacz_binarny_opcje:0,
        general_ui_przelacz_zdjecia_opcje:0,
        general_ui_przelacz_brainroot:false,
        general_ui_przelacz_ustawienia:false,
        general_ui_przelacz_info_ustawienia:0,
        general_ui_przelacz_info_opcje:0,
        #[cfg(feature = "statystyki")]
        general_ui_statystyki_klatki:Vec::new(),


        //debug menu specyfic
        ui_debug_specyfic_klikacz:0,
        ui_debug_specyfic_obrot:0.0,


        //wartości formatowanie:
        //rozmiary i czcionki
        formatowanie_wybor_czcionki: 0,
        formatowanie_rozmiar_czcionki_mala:10.,
        formatowanie_rozmiar_czcionki_srednia:14.,
        formatowanie_rozmiar_czcionki_duza:18.,
        formatowanie_wysokosc_przycisku:803.,
        //kółko
        formatowanie_rozmiar_kolko_puste:5.,
        formatowanie_rozmiar_kolko_puste_stroke:1.5,
        formatowanie_offset_kolko:14.,
        formatowanie_rozmiar_kolko:7.,
        //odstępy
        formatowanie_spacja_mala:5.,
        formatowanie_spacja_srednia:10.,
        formatowanie_spacja_duza:20.,
        //kolory
        formatowanie_kolor_kolko_puste: Color32::GRAY,
        formatowanie_kolor_kolko_pelne: Color32::GREEN,


        //wartości specyficzne dla ui pakowania:
        ui_pack_specyfic_folder_wejsciowy: String::from(""),
        ui_pack_specyfic_folder_wyjsciowy: String::from(""),
        ui_pack_specyfic_nazwa_pliku: String::from("output_file"),
        ui_pack_specyfic_password: String::from("lubieplacki6969"),
        ui_pack_specyfic_template: String::from("none"),
        ui_pack_specyfic_wybor_szyfrowania:0,
        ui_pack_specyfic_wybor_kompresji:0,
        ui_pack_specyfic_poziom_kompresji:3,
        ui_pack_specyfic_error_3:"".to_string(),
        ui_pack_specyfic_statystyki_przetworzone_pliki:0,
        ui_pack_specyfic_statystyki_czas_sekundy:0,
        ui_pack_specyfic_statystyki_czas_milisekundy:0,
        ui_pack_specyfic_status_przetwarzania:0,
        ui_pack_specyfic_statyczne_id:false,


        //wartości specyficzne dla ui rozpakowywania:
        ui_unpack_specyfic_sciezka_plik_binarny: String::from(""),
        ui_unpack_specyfic_sciezka_plik_indeksu: String::from(""),
        ui_unpack_specyfic_sciezka_folder_wyjsciowy: String::from(""),
        ui_unpack_specyfic_password: String::from("lubieplacki6969"),
        ui_unpack_specyfic_zmiana_szyfrowania:0,
        ui_unpack_specyfic_error_3:"".to_string(),
        ui_unpack_specyfic_statystyki_czas_sekundy:0,
        ui_unpack_specyfic_statystyki_czas_milisekundy:0,
        ui_unpack_specyfic_statystyki_przetworzone_pliki:0,
        ui_unpack_specyfic_status_przetwarzania:0,


        //wartości specyficzne dla ui łączenia rgb:
        ui_laczenie_specyfic_sciezka_plik_czerwony: PathBuf::new(),
        ui_laczenie_specyfic_sciezka_plik_zielony: PathBuf::new(),
        ui_laczenie_specyfic_sciezka_plik_niebieski: PathBuf::new(),
        ui_laczenie_specyfic_sciezka_folder_wyjsciowy: PathBuf::new(),
        ui_laczenie_specyfic_error_3:"".to_string(),
        ui_laczenie_specyfic_statystyki_czas_sekundy:0,
        ui_laczenie_specyfic_statystyki_czas_milisekundy:0,
        ui_laczenie_specyfic_dane_bity:8,
        ui_laczenie_specyfic_dane_rozszerzenia:0,
        ui_laczenie_specyfic_dane_rozdzielczosc:4,
        ui_laczenie_specyfic_dane_filter:4,
        ui_laczenie_specyfic_nazwa_pliku:"".to_string(),
        ui_laczenie_specyfic_dane_jakosc:75,
        ui_laczenie_specyfic_dane_filter_png:5,
        ui_laczenie_specyfic_stosunek_czerwony:1.,
        ui_laczenie_specyfic_stosunek_zielony:1.,
        ui_laczenie_specyfic_stosunek_niebieski:1.,
        ui_laczenie_specyfic_wysokość_czerwony:0,
        ui_laczenie_specyfic_wysokość_zielony:0,
        ui_laczenie_specyfic_wysokość_niebieski:0,
        ui_laczenie_specyfic_szerokość_czerwony:0,
        ui_laczenie_specyfic_szerokość_zielony:0,
        ui_laczenie_specyfic_szerokość_niebieski:0,
        ui_laczenie_specyfic_status_przetwarzania:0,


        //wartosci specyficzne dla ui konwersji zdjęć
        ui_konwersja_specyfic_sciezka_folder_wejsciowy: String::from(""),
        ui_konwersja_specyfic_sciezka_folder_wyjsciowy: String::from(""),
        ui_konwersja_specyfic_error_3:"".to_string(),
        ui_konwersja_specyfic_statystyki_czas_sekundy:0,
        ui_konwersja_specyfic_statystyki_czas_milisekundy:0,
        ui_konwersja_specyfic_statystyki_przetworzone_pliki:0,
        ui_konwersja_specyfic_statystyki_utworzone_pliki:0,
        ui_konwersja_specyfic_dane_alpha:0,
        ui_konwersja_specyfic_dane_alpha_kolor:0,
        ui_konwersja_specyfic_dane_filter:4,
        ui_konwersja_specyfic_dane_bool_jpg:false,
        ui_konwersja_specyfic_dane_bool_png:false,
        ui_konwersja_specyfic_dane_bool_png_16:false,
        ui_konwersja_specyfic_dane_bool_webp_lossless:false,
        ui_konwersja_specyfic_dane_bool_webp_lossy:false,
        ui_konwersja_specyfic_dane_bool_tga:false,
        ui_konwersja_specyfic_dane_filter_png:5,
        ui_konwersja_specyfic_dane_jakosc_jpg:75,
        ui_konwersja_specyfic_dane_jakosc_png:2,
        ui_konwersja_specyfic_dane_jakosc_webp_lossless:75,
        ui_konwersja_specyfic_dane_jakosc_webp_lossy:75,
        ui_konwersja_specyfic_dane_jakosc_tga:75,
        ui_konwersja_specyfic_dane_rozdzielczosc_16k:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_8k:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_4k:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_2k:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_1k:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_512:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_256:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_128:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_64:false,
        ui_konwersja_specyfic_dane_rozdzielczosc_32:false,
        ui_konwersja_specyfic_status_przetwarzania:0,


        //Ustawienia

        ui_ustawienia_tworzenie_pliku_lua: false, 
        ui_ustawienia_glosnosc:1.,

      }

    }
    
}