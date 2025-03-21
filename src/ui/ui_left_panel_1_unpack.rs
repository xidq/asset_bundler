use egui::{
    Color32, 
    Pos2, 
    Rect,
    TextEdit, 
    RichText,
    Context, 
    Vec2
};
use rfd::FileDialog;
use std::sync::{
    Arc, 
    Mutex
};
// use egui::WidgetText::RichText;
use crate::{decrypt_copy, dodaj_duży_label, dodaj_duży_richtext, dodaj_średni_label, dodaj_średni_richtext};
#[cfg(not(feature = "raw"))]
use crate::ui::ui_play_sound::play_finish_sound;

use crate::{
    ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka

},
utils::loading::animacja};

pub fn ui_left_panel_decrypt(proxy_self: &mut Appencja,ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32,zolty_ciemny:Color32,szarawy_ciemny:Color32){
    let margines_na_wybór_formatu_foty = proxy_self.formatowanie_spacja_średnia;

                        // ██╗   ██╗███╗   ██╗██████╗  █████╗  ██████╗██╗  ██╗
                        // ██║   ██║████╗  ██║██╔══██╗██╔══██╗██╔════╝██║ ██╔╝
                        // ██║   ██║██╔██╗ ██║██████╔╝███████║██║     █████╔╝ 
                        // ██║   ██║██║╚██╗██║██╔═══╝ ██╔══██║██║     ██╔═██╗ 
                        // ╚██████╔╝██║ ╚████║██║     ██║  ██║╚██████╗██║  ██╗
                        //  ╚═════╝ ╚═╝  ╚═══╝╚═╝     ╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝                


    // ●○⊙◯



    //                       _     _      ___ _ _     
    //  ___ ___ ___ ___    _| |___| |_   |  _|_| |___ 
    // | . | . | -_|   |  | . | .'|  _|  |  _| | | -_|
    // |___|  _|___|_|_|  |___|__,|_|    |_| |_|_|___|
    //     |_|                                        
    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_wybierz_plik_dat));
    });
    ui.add_space( proxy_self.formatowanie_spacja_mala);
    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duża+margines_na_wybór_formatu_foty);

        let btn_deszyfrowanie_folder_plik_dat = ui.button(dodaj_średni_richtext!(proxy_self.current_language.general_ui_wybierz_plik_dat.to_string()));
        if btn_deszyfrowanie_folder_plik_dat.clicked() {
            if let Some(path) = FileDialog::new().add_filter(proxy_self.current_language.general_ui_wybierz_plik_dat.to_string(), &["jrz","jrzs"]).pick_file() {

                proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny = path.to_string_lossy().to_string();
    
                // Sprawdzanie, czy wybrany plik ma rozszerzenie .jrzs
                if path.extension().map(|ext| ext == "jrzs").unwrap_or(false) {
                    proxy_self.ui_unpack_specyfic_zmiana_szyfrowania = 1; // Zmienna is_jrzs ustawiona na true
                } else {
                    proxy_self.ui_unpack_specyfic_zmiana_szyfrowania = 0; // Jeśli nie jest .jrzs, ustaw false
                }
            }

        }

        let btn_pozycjonowanie_general_ui_wybierz_plik_dat = Pos2::new(
            btn_deszyfrowanie_folder_plik_dat.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_deszyfrowanie_folder_plik_dat.rect.min.y+(btn_deszyfrowanie_folder_plik_dat.rect.size().y / 2.));
        
        if !proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny.is_empty(){

            ui.painter().circle_filled(btn_pozycjonowanie_general_ui_wybierz_plik_dat, proxy_self.formatowanie_rozmiar_kolko, proxy_self.formatowanie_kolor_kolko_pelne);

        } else {

            ui.painter().circle_stroke(btn_pozycjonowanie_general_ui_wybierz_plik_dat, proxy_self.formatowanie_rozmiar_kolko_puste, (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,proxy_self.formatowanie_kolor_kolko_puste));

        }



    });
    if proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny.len() >= 50 {
        let xxxx = &proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else if !proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny.is_empty(){
        

        ui.add(dodaj_średni_label!(&proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duża);
    }

    //                     _   _        ___ _ _     
    //  ___ ___ ___ ___   |_|_| |_ _   |  _|_| |___ 
    // | . | . | -_|   |  | | . |_'_|  |  _| | | -_|
    // |___|  _|___|_|_|  |_|___|_,_|  |_| |_|_|___|
    //     |_|                                      

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_wybierz_plik_idx));
    });
    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duża+margines_na_wybór_formatu_foty);

        let btn_deszyfrowanie_folder_plik_idx = ui.button(dodaj_średni_richtext!(proxy_self.current_language.general_ui_wybierz_plik_idx.to_string()));

        if btn_deszyfrowanie_folder_plik_idx.clicked() {

            if let Some(path) = FileDialog::new().add_filter(proxy_self.current_language.general_ui_wybierz_plik_idx.to_string(), &["idx"]).pick_file() {

                proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu = path.to_string_lossy().to_string();

            }
        }

        let btn_pozycjonowanie_general_ui_wybierz_plik_idx = Pos2::new(
            btn_deszyfrowanie_folder_plik_idx.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_deszyfrowanie_folder_plik_idx.rect.min.y+(btn_deszyfrowanie_folder_plik_idx.rect.size().y / 2.));
        
        if !proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu.is_empty(){

            ui.painter().circle_filled(btn_pozycjonowanie_general_ui_wybierz_plik_idx, proxy_self.formatowanie_rozmiar_kolko, proxy_self.formatowanie_kolor_kolko_pelne);

        } else {

            ui.painter().circle_stroke(btn_pozycjonowanie_general_ui_wybierz_plik_idx, proxy_self.formatowanie_rozmiar_kolko_puste, (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,proxy_self.formatowanie_kolor_kolko_puste));

        }




    });


    if proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu.len() >= 50 {
        let xxxx = &proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);
    }else if !proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu.is_empty(){
        ui.add(dodaj_średni_label!(&proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duża);
    }



    //          _           _      ___     _   _         
    //  ___ _ _| |_ ___ _ _| |_   |  _|___| |_| |___ ___ 
    // | . | | |  _| . | | |  _|  |  _| . | | . | -_|  _|
    // |___|___|_| |  _|___|_|    |_| |___|_|___|___|_|  
    //             |_|                                   

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_wybierz_folder_wyjściowy));
    });

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duża+margines_na_wybór_formatu_foty);

        let btn_deszyfrowanie_folder_folder_wyjściowy = ui.button(dodaj_średni_richtext!(proxy_self.current_language.general_ui_wybierz_folder));
        if btn_deszyfrowanie_folder_folder_wyjściowy.clicked() {
            
            if let Some(path) = FileDialog::new().pick_folder() {
                proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy = path.to_string_lossy().to_string()+"/";
            }

        }


        let btn_pozycjonowanie_deszyfrowanie_folder_wyjściowy = Pos2::new(
            btn_deszyfrowanie_folder_folder_wyjściowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_deszyfrowanie_folder_folder_wyjściowy.rect.min.y+(btn_deszyfrowanie_folder_folder_wyjściowy.rect.size().y / 2.));
        
        if !proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy.is_empty(){

            ui.painter().circle_filled(btn_pozycjonowanie_deszyfrowanie_folder_wyjściowy, proxy_self.formatowanie_rozmiar_kolko, proxy_self.formatowanie_kolor_kolko_pelne);

        } else {

            ui.painter().circle_stroke(btn_pozycjonowanie_deszyfrowanie_folder_wyjściowy, proxy_self.formatowanie_rozmiar_kolko_puste, (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,proxy_self.formatowanie_kolor_kolko_puste));

        }



    });

    if proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy.len() >= 50 {
        let xxxx = &proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);
    }else if !proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy.is_empty() {
        ui.add(dodaj_średni_label!(&proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);
    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duża);
    }

    //                          _   _         
    //  ___ ___ ___ ___ _ _ ___| |_|_|___ ___ 
    // | -_|   |  _|  _| | | . |  _| | . |   |
    // |___|_|_|___|_| |_  |  _|_| |_|___|_|_|
    //                 |___|_|               

    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.columns(1, |column|{
    column[0].vertical_centered_justified(|ui|{
        let przerwa_tam_gdzie_haslo = (30. + proxy_self.formatowanie_rozmiar_czcionki_średnia + proxy_self.formatowanie_spacja_mala - proxy_self.formatowanie_rozmiar_czcionki_duża - 1.) / 2.;

        if proxy_self.ui_unpack_specyfic_zmiana_szyfrowania==0{
            ui.add_space( przerwa_tam_gdzie_haslo);



                ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_haslo_wylaczone.to_string()));

            


        }else{

            ui.add(dodaj_średni_label!(proxy_self.current_language.general_ui_haslo_tytul.to_string()));
            ui.add_space( proxy_self.formatowanie_spacja_mala);
            let password_input = TextEdit::singleline(&mut proxy_self.ui_unpack_specyfic_password)
                .password(true)
                .hint_text(proxy_self.current_language.general_ui_haslo.to_string())
                .min_size(Vec2{x:200.,y:30.});
            ui.add(password_input);

        };
        });
    });

    ui.add_space( proxy_self.formatowanie_spacja_średnia);

    //  _       _   _           
    // | |_ _ _| |_| |_ ___ ___ 
    // | . | | |  _|  _| . |   |
    // |___|___|_| |_| |___|_|_|

    let sprawdzacz_plikow_dekompresja: bool = 
    std::path::Path::new(&proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny)
        .extension().is_some_and(|ext| ext == "jrz" || ext == "jrzs") &&
    std::path::Path::new(&proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu)
        .extension().is_some_and(|ext| ext == "idx") &&
    !proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy.is_empty();


    let tekst_przycisku_kompresji = if sprawdzacz_plikow_dekompresja{

        &proxy_self.current_language.deszyfrowanie_przycisk_ok}
        else
        {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                            
    // let deszyfr_tekst_check= if sprawdzacz_plikow_dekompresja {

    //     &proxy_self.current_language.deszyfrowanie_przycisk_ok.to_string()
        
    // } else {

    //     &proxy_self.current_language.szyfrowanie_przycisk_nie_ok.to_string()

    // };
    let de_t_p_d= match proxy_self.ui_unpack_specyfic_status_przetwarzania{
        0 => dodaj_duży_richtext!(tekst_przycisku_kompresji.to_string()),
        1 => RichText::new(proxy_self.general_ui_loading_tekst).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duża,1)).color(Color32::BLACK),
        2 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
        3 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
        4 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
        _ => dodaj_duży_richtext!(proxy_self.current_language.err_value_overflow)
    };

    match proxy_self.rx.try_recv() {
        Ok(Ok(kgfghkh)) => {
            let danene = kgfghkh.lock().unwrap();
            proxy_self.ui_unpack_specyfic_status_przetwarzania = 2;
            proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki = danene[0];
            proxy_self.ui_unpack_specyfic_statystyki_czas_sekundy = danene[1] as u64;
            proxy_self.ui_unpack_specyfic_statystyki_czas_milisekundy = danene[2] as u32;
            #[cfg(not(feature = "raw"))]
            play_finish_sound(proxy_self.ui_ustawienia_glosnosc);
        }
        Ok(Err(e)) => {
            proxy_self.ui_unpack_specyfic_status_przetwarzania = 3;
            proxy_self.ui_unpack_specyfic_error_3 = e.to_string();
            // eprintln!("Błąd: {}", e);
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            ctx.request_repaint();
            (proxy_self.general_ui_loading,proxy_self.general_ui_loading_tekst)=animacja(proxy_self.general_ui_loading);

        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            proxy_self.ui_unpack_specyfic_status_przetwarzania = 4;
            proxy_self.ui_unpack_specyfic_error_3 = "Disconected".to_string();
        }
    }

    // let deszyfr_but_col = if sprawdzacz_plikow_dekompresja{Color32::DARK_GREEN}else{szarawy_ciemny};
    let deszyfr_butt_col = match (
        sprawdzacz_plikow_dekompresja,
        proxy_self.ui_unpack_specyfic_status_przetwarzania
    ){
        (true,0) => Color32::DARK_GREEN,
        (true,1) => zolty_ciemny,
        _ =>szarawy_ciemny
    };

        if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.general_ui_szerokosc_okna / 4.),y:proxy_self.formatowanie_wysokosc_przycisku},Vec2{x:250.,y:40.}),egui::Button::new(de_t_p_d)
        .min_size(egui::vec2((proxy_self.general_ui_szerokosc_okna / 2.) - 100.0, 40.0))
        .corner_radius(10.)
        .fill(deszyfr_butt_col))
        .clicked() && sprawdzacz_plikow_dekompresja{

            // if sprawdzacz_plikow_dekompresja 
            // {
                proxy_self.ui_unpack_specyfic_status_przetwarzania = 0;
                proxy_self.general_ui_loading = 0;

                let dat_file = std::path::Path::new(&proxy_self.ui_unpack_specyfic_ścieżka_plik_binarny).to_path_buf();
                let idx_file = std::path::Path::new(&proxy_self.ui_unpack_specyfic_ścieżka_plik_indeksu).to_path_buf();
                let output_folder = std::path::Path::new(&proxy_self.ui_unpack_specyfic_ścieżka_folder_wyjściowy).to_path_buf();
                let arc_z_str = Arc::new(Mutex::new(vec![proxy_self.ui_unpack_specyfic_password.clone().to_string()]));
                let arc_z_path = Arc::new(Mutex::new(vec![dat_file.clone(),idx_file.clone(),output_folder.clone()]));
                let arc_z_u8 = Arc::new(Mutex::new(vec![proxy_self.ui_unpack_specyfic_zmiana_szyfrowania]));

                let de_tx_clone = proxy_self.tx.clone();
                std::thread::spawn(move || {
                    let de_result = decrypt_copy::decrypt_files(
                                arc_z_path,
                                arc_z_str,
                                arc_z_u8,
                            );
                        
                            match de_tx_clone.send(de_result) {
                                Ok(_) => {
                                    #[cfg(debug_assertions)]
                                    println!("Wysłano wynik") 
                                },
                                Err(_e) => {
                                    #[cfg(debug_assertions)]
                                    eprintln!("Błąd wysyłania: {}", _e)
                                },
                            }
                        });
                        
                        proxy_self.ui_unpack_specyfic_status_przetwarzania = 1;

                    }
                // }

                                                
    
                                
}