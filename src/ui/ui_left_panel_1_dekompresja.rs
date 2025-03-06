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
use crate::decrypt_copy;
use crate::ui::{
    ui_defaults::Appencja,
    ui_play_sound::play_finish_sound,
    ui_change_font::wybrana_aktualna_czcionka
};

pub fn ui_left_panel_decrypt(proxy_self: &mut Appencja,ctx: &Context,ui: &mut egui::Ui, fiolet_ciemny:Color32,zolty_ciemny:Color32,szarawy_ciemny:Color32){
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;

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
                        ui.add_space(margines_na_wybor_formatu_foty);
                        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_plik_dat)
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                            .selectable(false)
                        );
                    });
                    ui.add_space( proxy_self.formatowanie_spacja_mala);
                    ui.horizontal(|ui|{

                        ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

                        let btn_deszyfrowanie_folder_plik_dat = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_plik_dat.to_string())
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        if btn_deszyfrowanie_folder_plik_dat.clicked() {
                            if let Some(path) = FileDialog::new().add_filter(proxy_self.current_language.general_ui_wybierz_plik_dat.to_string(), &["jrz","jrzs"]).pick_file() {

                                proxy_self.imput_de_dat_folder_path = path.to_string_lossy().to_string();
                    
                                // Sprawdzanie, czy wybrany plik ma rozszerzenie .jrzs
                                if path.extension().map(|ext| ext == "jrzs").unwrap_or(false) {
                                    proxy_self.toggle_de_encryption = 1; // Zmienna is_jrzs ustawiona na true
                                } else {
                                    proxy_self.toggle_de_encryption = 0; // Jeśli nie jest .jrzs, ustaw false
                                }
                            }

                        }

                        let btn_pozycjonowanie_general_ui_wybierz_plik_dat = Pos2::new(
                            btn_deszyfrowanie_folder_plik_dat.rect.min.x - proxy_self.offset_cirkul,
                            btn_deszyfrowanie_folder_plik_dat.rect.min.y+(btn_deszyfrowanie_folder_plik_dat.rect.size().y / 2.));
                        
                        if !proxy_self.imput_de_dat_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_general_ui_wybierz_plik_dat, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_general_ui_wybierz_plik_dat, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                        }



                    });
                    if proxy_self.imput_de_dat_folder_path.len() >= 50 {
                        let xxxx = &proxy_self.imput_de_dat_folder_path;
                        let startu = &xxxx[..=15];
                        let endu = &xxxx[xxxx.len()-30 ..];
                        ui.add(egui::Label::new(RichText::new(
                            format!("{}/.../{}",startu,endu))
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                    }else if !proxy_self.imput_de_dat_folder_path.is_empty(){
                        

                        ui.add(egui::Label::new(RichText::new(&proxy_self.imput_de_dat_folder_path).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                    }else{
                        ui.add_space(proxy_self.formatowanie_spacja_duza);
                    }

                    //                     _   _        ___ _ _     
                    //  ___ ___ ___ ___   |_|_| |_ _   |  _|_| |___ 
                    // | . | . | -_|   |  | | . |_'_|  |  _| | | -_|
                    // |___|  _|___|_|_|  |_|___|_,_|  |_| |_|_|___|
                    //     |_|                                      

                    ui.horizontal(|ui|{
                        ui.add_space(margines_na_wybor_formatu_foty);
                        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_plik_idx).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                    });
                    ui.add_space( proxy_self.formatowanie_spacja_mala);

                    ui.horizontal(|ui|{

                        ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

                        let btn_deszyfrowanie_folder_plik_idx = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_plik_idx.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));

                        if btn_deszyfrowanie_folder_plik_idx.clicked() {

                            if let Some(path) = FileDialog::new().add_filter(proxy_self.current_language.general_ui_wybierz_plik_idx.to_string(), &["idx"]).pick_file() {

                                proxy_self.imput_de_idx_folder_path = path.to_string_lossy().to_string();

                            }
                        }

                        let btn_pozycjonowanie_general_ui_wybierz_plik_idx = Pos2::new(
                            btn_deszyfrowanie_folder_plik_idx.rect.min.x - proxy_self.offset_cirkul,
                            btn_deszyfrowanie_folder_plik_idx.rect.min.y+(btn_deszyfrowanie_folder_plik_idx.rect.size().y / 2.));
                        
                        if !proxy_self.imput_de_idx_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_general_ui_wybierz_plik_idx, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_general_ui_wybierz_plik_idx, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                        }




                    });


                    if proxy_self.imput_de_idx_folder_path.len() >= 50 {
                        let xxxx = &proxy_self.imput_de_idx_folder_path;
                        let startu = &xxxx[..=15];
                        let endu = &xxxx[xxxx.len()-30 ..];
                        ui.add(egui::Label::new(RichText::new(
                            format!("{}/.../{}",startu,endu))
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
                    }else if !proxy_self.imput_de_idx_folder_path.is_empty(){
                        ui.add(egui::Label::new(RichText::new(&proxy_self.imput_de_idx_folder_path).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                    }else{
                        ui.add_space(proxy_self.formatowanie_spacja_duza);
                    }



                    //          _           _      ___     _   _         
                    //  ___ _ _| |_ ___ _ _| |_   |  _|___| |_| |___ ___ 
                    // | . | | |  _| . | | |  _|  |  _| . | | . | -_|  _|
                    // |___|___|_| |  _|___|_|    |_| |___|_|___|___|_|  
                    //             |_|                                   

                    ui.horizontal(|ui|{
                        ui.add_space(margines_na_wybor_formatu_foty);
                        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                    });

                    ui.add_space( proxy_self.formatowanie_spacja_mala);

                    ui.horizontal(|ui|{

                        ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

                        let btn_deszyfrowanie_folder_folder_wyjsciowy = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_folder).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        if btn_deszyfrowanie_folder_folder_wyjsciowy.clicked() {
                            
                            if let Some(path) = FileDialog::new().pick_folder() {
                                proxy_self.output_de_folder_path = path.to_string_lossy().to_string()+"/";
                            }

                        }


                        let btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy = Pos2::new(
                            btn_deszyfrowanie_folder_folder_wyjsciowy.rect.min.x - proxy_self.offset_cirkul,
                            btn_deszyfrowanie_folder_folder_wyjsciowy.rect.min.y+(btn_deszyfrowanie_folder_folder_wyjsciowy.rect.size().y / 2.));
                        
                        if !proxy_self.output_de_folder_path.is_empty(){

                            ui.painter().circle_filled(btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                        } else {

                            ui.painter().circle_stroke(btn_pozycjonowanie_deszyfrowanie_folder_wyjsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                        }



                    });

                    if proxy_self.output_de_folder_path.len() >= 50 {
                        let xxxx = &proxy_self.output_de_folder_path;
                        let startu = &xxxx[..=15];
                        let endu = &xxxx[xxxx.len()-30 ..];
                        ui.add(egui::Label::new(RichText::new(
                            format!("{}/.../{}",startu,endu))
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
                    }else if !proxy_self.output_de_folder_path.is_empty() {
                        ui.add(egui::Label::new(RichText::new(&proxy_self.output_de_folder_path).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
                    }else{
                        ui.add_space(proxy_self.formatowanie_spacja_duza);
                    }

                    //                          _   _         
                    //  ___ ___ ___ ___ _ _ ___| |_|_|___ ___ 
                    // | -_|   |  _|  _| | | . |  _| | . |   |
                    // |___|_|_|___|_| |_  |  _|_| |_|___|_|_|
                    //                 |___|_|               

                    ui.add_space(proxy_self.formatowanie_spacja_mala);
                    ui.columns(1, |column|{
                    column[0].vertical_centered_justified(|ui|{
                        let przerwa_tam_gdzie_haslo = (30. + proxy_self.formatowanie_rozmiar_czcionki_srednia + proxy_self.formatowanie_spacja_mala - proxy_self.formatowanie_rozmiar_czcionki_duza - 1.) / 2.;

                        if proxy_self.toggle_de_encryption==0{
                            ui.add_space( przerwa_tam_gdzie_haslo);



                                ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_wylaczone.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)).color(fiolet_ciemny)).selectable(false));

                            


                        }else{

                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_tytul.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            ui.add_space( proxy_self.formatowanie_spacja_mala);
                            let password_input = TextEdit::singleline(&mut proxy_self.de_password)
                                .password(true)
                                .hint_text(proxy_self.current_language.general_ui_haslo.to_string())
                                .min_size(Vec2{x:200.,y:30.});
                            ui.add(password_input);

                        };
                        });
                    });

                    ui.add_space( proxy_self.formatowanie_spacja_srednia);

                    //  _       _   _           
                    // | |_ _ _| |_| |_ ___ ___ 
                    // | . | | |  _|  _| . |   |
                    // |___|___|_| |_| |___|_|_|

                    let sprawdzacz_plikow_dekompresja: bool = 
                    std::path::Path::new(&proxy_self.imput_de_dat_folder_path)
                        .extension().is_some_and(|ext| ext == "jrz" || ext == "jrzs") &&
                    std::path::Path::new(&proxy_self.imput_de_idx_folder_path)
                        .extension().is_some_and(|ext| ext == "idx") &&
                    !proxy_self.output_de_folder_path.is_empty();
        

                    let sprawdzacz_plikow_kompresji= !proxy_self.imput_folder_path.is_empty() && !proxy_self.output_folder_path.is_empty() && !proxy_self.output_name.is_empty();
                    let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

                        &proxy_self.current_language.deszyfrowanie_przycisk_ok}
                        else
                        {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                                            
                    // let deszyfr_tekst_check= if sprawdzacz_plikow_dekompresja {

                    //     &proxy_self.current_language.deszyfrowanie_przycisk_ok.to_string()
                        
                    // } else {

                    //     &proxy_self.current_language.szyfrowanie_przycisk_nie_ok.to_string()

                    // };
                    let de_t_p_d= match proxy_self.de_czy_to_juz_koniec{
                        0 => RichText::new(tekst_przycisku_kompresji.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)),
                        1 => RichText::new(proxy_self.de_loading_anim).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,1)).color(Color32::BLACK),
                        2 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)),
                        3 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)),
                        4 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)),
                        _ => RichText::new("")
                    };

                    match proxy_self.rx.try_recv() {
                        Ok(Ok(kgfghkh)) => {
                            let danene = kgfghkh.lock().unwrap();
                            proxy_self.de_czy_to_juz_koniec = 2;
                            proxy_self.de_przetworzone_pliki = danene[0];
                            proxy_self.de_timestamp_sekundy = danene[1] as u64;
                            proxy_self.de_timestamp_milisekundy = danene[2] as u32;
                            play_finish_sound(proxy_self.ustawienia_glosnosc);
                        }
                        Ok(Err(e)) => {
                            proxy_self.de_czy_to_juz_koniec = 3;
                            proxy_self.szyfrowanie_err_3 = e.to_string();
                            // eprintln!("Błąd: {}", e);
                        }
                        Err(std::sync::mpsc::TryRecvError::Empty) => {
                            ctx.request_repaint();
                            match proxy_self.de_loading{
                                181 => {proxy_self.de_loading = 0; proxy_self.de_loading_anim=""},
                                151..=180 => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[=====>]"},
                                121..=150 => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[====>-]"},
                                91..=120 => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[===>--]"},
                                61..=90 => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[==>---]"},
                                30..=60 => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[=>----]"},
                                _ => {proxy_self.de_loading += 1; proxy_self.de_loading_anim="[>-----]"},
                            }  
                        }
                        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                            proxy_self.de_czy_to_juz_koniec = 4;
                            proxy_self.szyfrowanie_err_3 = "Disconected".to_string();
                        }
                    }

                    // let deszyfr_but_col = if sprawdzacz_plikow_dekompresja{Color32::DARK_GREEN}else{szarawy_ciemny};
                    let deszyfr_butt_col = match (
                        sprawdzacz_plikow_dekompresja,
                        proxy_self.de_czy_to_juz_koniec
                    ){
                        (true,0) => Color32::DARK_GREEN,
                        (true,1) => zolty_ciemny,
                        _ =>szarawy_ciemny
                    };

                        if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.szerokosc_okna / 4.),y:proxy_self.wysokosc_btn_egzekucyjny},Vec2{x:250.,y:40.}),egui::Button::new(de_t_p_d
                        .color(Color32::GRAY))
                        .min_size(egui::vec2((proxy_self.szerokosc_okna / 2.) - 100.0, 40.0))
                        .corner_radius(10.)
                        .fill(deszyfr_butt_col))
                        .clicked() && sprawdzacz_plikow_dekompresja{

                            // if sprawdzacz_plikow_dekompresja 
                            // {
                                proxy_self.de_czy_to_juz_koniec = 0;

                                let dat_file = std::path::Path::new(&proxy_self.imput_de_dat_folder_path).to_path_buf(); 
                                let idx_file = std::path::Path::new(&proxy_self.imput_de_idx_folder_path).to_path_buf(); 
                                let output_folder = std::path::Path::new(&proxy_self.output_de_folder_path).to_path_buf(); 
                                let arc_z_str = Arc::new(Mutex::new(vec![proxy_self.de_password.clone().to_string()]));
                                let arc_z_path = Arc::new(Mutex::new(vec![dat_file.clone(),idx_file.clone(),output_folder.clone()]));
                                let arc_z_u8 = Arc::new(Mutex::new(vec![proxy_self.toggle_encryption]));

                                let de_tx_clone = proxy_self.tx.clone();
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
                                        
                                        proxy_self.de_czy_to_juz_koniec = 1;
    
                                    }
                                // }
    
                                                
    
                                
}