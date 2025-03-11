use egui::{
    Color32, 
    Pos2, 
    Response, 
    RichText,
    Rect,
    Vec2,
    Context,
    ComboBox
};
use rfd::FileDialog;
use std::sync::{
    Arc, 
    Mutex
};
#[cfg(not(feature = "raw"))]
use crate::ui::ui_play_sound::play_finish_sound;
use crate::{dodaj_duży_label, dodaj_duży_richtext, dodaj_średni_label, dodaj_średni_richtext, ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka,
}, utils::loading::animacja};
// use crate::macros::*;
// use crate::ui::items_formatting::dodaj_średni_label;

pub fn ui_left_panel_foty_przetwarzanie(
    proxy_self: &mut Appencja,
    ctx: &Context,ui: &mut egui::Ui,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32
){
    let margines_na_wybór_formatu_foty = proxy_self.formatowanie_spacja_średnia;
    // ui.add_space(proxy_self.formatowanie_spacja_średnia);

        // ███████  ██████  ██      ██████  ███████ ██████      ██     ██ ███████      ██ ███████  ██████ ██  ██████  ██     ██ ██    ██ 
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██     ██ ██           ██ ██      ██      ██ ██    ██ ██     ██  ██  ██  
        // █████   ██    ██ ██      ██   ██ █████   ██████      ██  █  ██ █████        ██ ███████ ██      ██ ██    ██ ██  █  ██   ████   
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██ ███ ██ ██      ██   ██      ██ ██      ██ ██    ██ ██ ███ ██    ██    
        // ██       ██████  ███████ ██████  ███████ ██   ██      ███ ███  ███████  █████  ███████  ██████ ██  ██████   ███ ███     ██ 

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(
            proxy_self.current_language.general_ui_wybierz_folder_wejściowy));
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.horizontal(|ui|{
                            ui.add_space(

                                proxy_self.formatowanie_spacja_duża +
                                    margines_na_wybór_formatu_foty);

                            let btn_foty_folder_wejściowy : Response =
                                ui.button(dodaj_średni_richtext!(
                                    proxy_self.current_language.general_ui_wybierz_folder));

                            if btn_foty_folder_wejściowy.clicked() {

                                proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0;

                                if let Some(path) = FileDialog::new().pick_folder() {

                                    proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy =
                                        path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_foty_folder_wejściowy = Pos2::new(
                                btn_foty_folder_wejściowy.rect.min.x -
                                    proxy_self.formatowanie_offset_kolko,
                                btn_foty_folder_wejściowy.rect.min.y +
                                    (btn_foty_folder_wejściowy.rect.size().y / 2.));
                            
                            if !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy
                                .is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_foty_folder_wejściowy,
                                       proxy_self.formatowanie_rozmiar_kolko,
                                       proxy_self.formatowanie_kolor_kolko_pelne);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_foty_folder_wejściowy,
                                       proxy_self.formatowanie_rozmiar_kolko_puste,
                                       (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                                        proxy_self.formatowanie_kolor_kolko_puste));

                            }


                        });
                        if proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy.len() >= 50 {

                            let xxxx =
                                &proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            let tekst_ścieżka_wyjściowa = format!("{}/.../{}",startu,endu);
                            ui.add(dodaj_średni_label!(&tekst_ścieżka_wyjściowa));

                            ui.add_space(
                                proxy_self.formatowanie_spacja_duża -
                                    proxy_self.formatowanie_rozmiar_czcionki_średnia - 4.);
                
                
                        }else if !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy
                            .is_empty(){

                            ui.add(dodaj_średni_label!(
                                &proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy));

                            ui.add_space(
                                proxy_self.formatowanie_spacja_duża -
                                    proxy_self.formatowanie_rozmiar_czcionki_średnia - 4.);

                        }else{
                            ui.add_space(proxy_self.formatowanie_spacja_duża);
                        }


                            // ███████  ██████  ██      ██████  ███████ ██████      ██     ██ ██    ██      ██ ███████  ██████ ██  ██████  ██     ██ ██    ██ 
                            // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██     ██  ██  ██       ██ ██      ██      ██ ██    ██ ██     ██  ██  ██  
                            // █████   ██    ██ ██      ██   ██ █████   ██████      ██  █  ██   ████        ██ ███████ ██      ██ ██    ██ ██  █  ██   ████   
                            // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██ ███ ██    ██    ██   ██      ██ ██      ██ ██    ██ ██ ███ ██    ██    
                            // ██       ██████  ███████ ██████  ███████ ██   ██      ███ ███     ██     █████  ███████  ██████ ██  ██████   ███ ███     ██  

                        ui.horizontal(|ui|{
                            
                            ui.add_space(margines_na_wybór_formatu_foty);
                            ui.add(dodaj_duży_label!(
                                proxy_self.current_language.general_ui_wybierz_folder_wyjściowy));

                        });
    
                        ui.add_space(proxy_self.formatowanie_spacja_mala);


                        ui.horizontal(|ui|{
                            ui.add_space(proxy_self.formatowanie_spacja_duża +
                                margines_na_wybór_formatu_foty);

                            let btn_foty_folder_wyjściowy : Response =
                                ui.button(dodaj_średni_richtext!(
                                    proxy_self.current_language.general_ui_wybierz_folder));

                            if btn_foty_folder_wyjściowy.clicked() {

                                proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0;

                                if let Some(path) = FileDialog::new().pick_folder() {
    //(?)
                                    proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy =
                                        path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_foty_folder_wyjściowy = Pos2::new(
                                btn_foty_folder_wyjściowy.rect.min.x -
                                    proxy_self.formatowanie_offset_kolko,
                                btn_foty_folder_wyjściowy.rect.min.y +
                                    (btn_foty_folder_wyjściowy.rect.size().y / 2.));
                            
                            if !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy
                                .is_empty(){

                                ui.painter().circle_filled(
                                    btn_pozycjonowanie_foty_folder_wyjściowy,
                                    proxy_self.formatowanie_rozmiar_kolko,
                                    proxy_self.formatowanie_kolor_kolko_pelne);

                            } else {

                                ui.painter().circle_stroke(
                                    btn_pozycjonowanie_foty_folder_wyjściowy,
                                    proxy_self.formatowanie_rozmiar_kolko_puste,
                                    (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                                     proxy_self.formatowanie_kolor_kolko_puste));

                            }


                        });
                        if proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy.len() >= 50 {
                            let xxxx = 
                                &proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
                            ui.add_space( 
                                proxy_self.formatowanie_spacja_duża -
                                    proxy_self.formatowanie_rozmiar_czcionki_średnia - 4.);
                
                
                        }else if !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy
                            .is_empty(){
                            
                            ui.add(dodaj_średni_label!(
                                proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy.to_owned()));

                            ui.add_space(
                                proxy_self.formatowanie_spacja_duża -
                                    proxy_self.formatowanie_rozmiar_czcionki_średnia - 4.);

                        }else{
                            
                            ui.add_space(proxy_self.formatowanie_spacja_duża);
                            
                        }


                            //  █████  ██      ██████  ██   ██  █████  
                            // ██   ██ ██      ██   ██ ██   ██ ██   ██ 
                            // ███████ ██      ██████  ███████ ███████ 
                            // ██   ██ ██      ██      ██   ██ ██   ██ 
                            // ██   ██ ███████ ██      ██   ██ ██   ██


                        ui.separator();

                        ui.add_space(proxy_self.formatowanie_spacja_średnia);
                        ui.vertical_centered_justified(|ui|{
                            
                            ui.add(dodaj_duży_label!(
                                proxy_self.current_language.general_ui_alpha_tytul));
                            
                        });

                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(2,|columns|{
                            columns[0].vertical_centered_justified(|ui|{
                                // if ui.selectable_value(dodaj_selectable_val_tekst!()).clicked(){};
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                    0,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.general_colors_white)
                                        .color(Color32::WHITE)).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                    1,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.general_colors_black)
                                        .color(Color32::BLACK)).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                    2,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.general_colors_red)
                                        .color(Color32::RED)).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                    3,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.general_colors_green)
                                        .color(Color32::GREEN)).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                    4,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.general_colors_blue)
                                        .color(Color32::BLUE)).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            columns[1].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha,
                                    0,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.przetwarzanie_bez_alpha))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_alpha,
                                    1,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.przetwarzanie_z_alpha))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                
                            });
                        });

                            // ███████ ██ ██      ████████ ██████  ██    ██ 
                            // ██      ██ ██         ██    ██   ██  ██  ██  
                            // █████   ██ ██         ██    ██████    ████   
                            // ██      ██ ██         ██    ██   ██    ██    
                            // ██      ██ ███████    ██    ██   ██    ██    

                        ui.add_space(proxy_self.formatowanie_spacja_średnia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_średnia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(dodaj_duży_label!(
                                proxy_self.current_language.general_ui_filter_png_tytul));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        
                        ui.columns(5, |column|{
                            column[0].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_filter,
                                    0,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.image_specyfic_filter_nearest))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                            column[1].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_filter,
                                    1,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.image_specyfic_filter_triangle))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                            column[2].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_filter,
                                    2,
                                    dodaj_średni_richtext!("Catmull\nRom"))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                            column[3].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_filter,
                                    3,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.image_specyfic_filter_gaussian))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                            column[4].vertical_centered_justified(|ui|{
                                if ui.selectable_value(
                                    &mut proxy_self.ui_konwersja_specyfic_dane_filter,
                                    4,
                                    dodaj_średni_richtext!(
                                        proxy_self.current_language.image_specyfic_filter_lanczos3))
                                    .clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                        });



                            // ██     ██  █████  ██████  ██  █████  ███    ██ ████████ ██    ██ 
                            // ██     ██ ██   ██ ██   ██ ██ ██   ██ ████   ██    ██     ██  ██  
                            // ██  █  ██ ███████ ██████  ██ ███████ ██ ██  ██    ██      ████   
                            // ██ ███ ██ ██   ██ ██   ██ ██ ██   ██ ██  ██ ██    ██       ██    
                            //  ███ ███  ██   ██ ██   ██ ██ ██   ██ ██   ████    ██       ██ 

                        ui.add_space(proxy_self.formatowanie_spacja_średnia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_średnia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_warianty_tytul));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);

                        ui.columns(2, |column|{
                            column[0].vertical(|ui|{
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_jpg,
                                        dodaj_średni_richtext!("jpg")).clicked(){
                                        proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_png,
                                        dodaj_średni_richtext!("png")).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_png_16,
                                        dodaj_średni_richtext!("png 16bit")).clicked(){
                                        proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossless,
                                        dodaj_średni_richtext!("webp")).clicked(){
                                        proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossy,
                                        dodaj_średni_richtext!("webp lossy")).clicked(){
                                        proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybór_formatu_foty);
                                    if ui.checkbox(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_bool_tga,
                                        dodaj_średni_richtext!("tga")).clicked(){
                                        proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    };
                                });

                            });
                            column[1].vertical_centered_justified(|ui|{
                                ui.add_enabled(proxy_self.ui_konwersja_specyfic_dane_bool_jpg,|ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(&mut proxy_self.ui_konwersja_specyfic_dane_jakosc_jpg, 0..=100))
    
                                });
                                if ui.add_enabled(proxy_self.ui_konwersja_specyfic_dane_bool_png||
                                      proxy_self.ui_konwersja_specyfic_dane_bool_png_16,
                                                  |ui: &mut egui::Ui|{
                                    ui.horizontal(|ui|{
                                        ui.add(dodaj_średni_label!(proxy_self.current_language.general_ui_filter_png_tytul));
                                        ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                                        .width(200. / 2.)
                                        .selected_text(dodaj_średni_richtext!(
                                            match proxy_self.ui_konwersja_specyfic_dane_filter_png{
                                                0 => proxy_self.current_language.png_specyfic_filter_png_none,
                                                1=> proxy_self.current_language.png_specyfic_filter_png_sub,
                                                2=>proxy_self.current_language.png_specyfic_filter_png_up,
                                                3=>proxy_self.current_language.png_specyfic_filter_png_avg,
                                                4=>proxy_self.current_language.png_specyfic_filter_png_paeth,
                                                5=>proxy_self.current_language.png_specyfic_filter_png_adaptive,
                                                _ => proxy_self.current_language.err_value_overflow
                                            }
                                        ))
                                        .show_ui(ui, |ui| {
            
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                0,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_none));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                1,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_sub));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                2,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_up));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                3,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_avg));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                4,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_paeth));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                                5,
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_filter_png_adaptive));
                                            }).response

                                        }).response
                                        
                                    // ui.add(egui::Slider::new(&mut proxy_self.ui_konwersja_specyfic_dane_jakosc_png, 0..=100))
    
                                }).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.add_enabled(proxy_self.ui_konwersja_specyfic_dane_bool_png||
                                      proxy_self.ui_konwersja_specyfic_dane_bool_png_16,
                                                  |ui: &mut egui::Ui|{
                                    ui.horizontal(|ui|{
                                        ui.add(dodaj_średni_label!(
                                            proxy_self.current_language.general_ui_kompresja_tekst));
                                        ComboBox::from_id_salt("dropbox_png_quality")
                                        .width(140. / 2.)
                                        .selected_text(dodaj_średni_richtext!(
                                            match proxy_self.ui_konwersja_specyfic_dane_jakosc_png{
                                                0 => proxy_self.current_language.png_specyfic_compression_none,
                                                1=> proxy_self.current_language.png_specyfic_compression_default,
                                                2=>proxy_self.current_language.png_specyfic_compression_best,
                                                _ => proxy_self.current_language.err_value_overflow
                                            }
                                        ))
                                        .show_ui(ui, |ui| {
            
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_png, 
                                                0, 
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_compression_none));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_png, 
                                                1, 
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_compression_default));
                                            ui.selectable_value(
                                                &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_png, 
                                                2, 
                                                dodaj_średni_richtext!(
                                                    proxy_self.current_language.png_specyfic_compression_best));
                                           }).response
                                            
                                        }).response
    
                                }).clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };

                                let sfdgdf= 
                                    ui.add_enabled(false,
                                                   |ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossless, 
                                        0..=100))
    
                                });
                                if sfdgdf.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                                if ui.add_enabled(
                                    proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossy,
                                    |ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossy, 
                                        0..=100))
    
                                }).clicked(){
                                    
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                    
                                };
                                if ui.add_enabled(false,
                                                  |ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(
                                        &mut proxy_self.ui_konwersja_specyfic_dane_jakosc_tga, 
                                        0..=100))
    
                                }).clicked(){
                                    
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };

                            });
                        });
                            // ██████   ██████  ███████ ██████  ███████ ██ ███████ ██       ██████ ███████  ██████  ███████  ██████ 
                            // ██   ██ ██    ██    ███  ██   ██    ███  ██ ██      ██      ██         ███  ██    ██ ██      ██      
                            // ██████  ██    ██   ███   ██   ██   ███   ██ █████   ██      ██        ███   ██    ██ ███████ ██      
                            // ██   ██ ██    ██  ███    ██   ██  ███    ██ ██      ██      ██       ███    ██    ██      ██ ██      
                            // ██   ██  ██████  ███████ ██████  ███████ ██ ███████ ███████  ██████ ███████  ██████  ███████  ██████
                            ui.add_space(proxy_self.formatowanie_spacja_średnia);
                            ui.separator();
                            ui.add_space(proxy_self.formatowanie_spacja_średnia);
                            ui.vertical_centered_justified(|ui|{
                                ui.add(dodaj_duży_label!(
                                    proxy_self.current_language.general_ui_rozdzielczość_tytul));
                            });
                            ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(10, |column|{
                            column[0].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_16k_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k,
                                        dodaj_średni_richtext!("16k"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_16k_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_16k_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                            column[1].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_8k_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k,
                                                        dodaj_średni_richtext!("8k"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_8k_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_8k_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[2].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_4k_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k,
                                        dodaj_średni_richtext!("4k"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_4k_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_4k_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[3].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_2k_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k,
                                        dodaj_średni_richtext!("2k"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_2k_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_2k_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[4].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_1k_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k,
                                        dodaj_średni_richtext!("1k"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_1k_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_1k_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[5].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_512_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512,
                                        dodaj_średni_richtext!("512"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_512_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_512_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[6].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_256_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256,
                                        dodaj_średni_richtext!("256"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_256_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_256_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[7].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_128_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128,
                                        dodaj_średni_richtext!("128"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_128_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_128_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[8].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_64_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64,
                                        dodaj_średni_richtext!("64"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_64_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_64_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });
                            column[9].vertical_centered_justified(|ui|{
                                let ui_konwersja_specyfic_dane_rozdzielczość_32_wybór =
                                    ui.selectable_label(
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32,
                                        dodaj_średni_richtext!("32"));
                                if ui_konwersja_specyfic_dane_rozdzielczość_32_wybór.clicked(){
                                    if !proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32=true
                                    } else if proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32{
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32=false
                                    }
                                }
                                if ui_konwersja_specyfic_dane_rozdzielczość_32_wybór.clicked(){
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0
                                };
                            });

                        });




                        // ██████  ██████  ███████ ██    ██  ██████ ██ ███████ ██   ██ 
                        // ██   ██ ██   ██    ███   ██  ██  ██      ██ ██      ██  ██  
                        // ██████  ██████    ███     ████   ██      ██ ███████ █████   
                        // ██      ██   ██  ███       ██    ██      ██      ██ ██  ██  
                        // ██      ██   ██ ███████    ██     ██████ ██ ███████ ██   ██ 


                        let sprawdzacz_przycisku_fot= 
                            !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy.is_empty() &&
                            !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy.is_empty() &&
                            (
                                proxy_self.ui_konwersja_specyfic_dane_bool_jpg || 
                                    proxy_self.ui_konwersja_specyfic_dane_bool_png || 
                                proxy_self.ui_konwersja_specyfic_dane_bool_png_16 || 
                                    proxy_self.ui_konwersja_specyfic_dane_bool_tga || 
                                proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossless || 
                                    proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossy
                            ) && (
                                proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k ||
                                    proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k ||
                                proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k ||
                                    proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k ||
                                proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k ||
                                    proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512 ||
                                proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256 ||
                                    proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128 ||
                                proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64 ||
                                    proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32
                            );

                        let tekst_przycisku_kompresji = if sprawdzacz_przycisku_fot{

                            &proxy_self.current_language.szyfrowanie_przycisk_ok}
                            else
                            {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                        

                        let hjgfkjlh = 
                            !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy.is_empty() &&
                                proxy_self.ui_konwersja_specyfic_status_przetwarzania !=1 && 
                                !proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy.is_empty();
    
                        let foto_butt_col = match (
                            sprawdzacz_przycisku_fot,
                            proxy_self.ui_konwersja_specyfic_status_przetwarzania
                        ){
                            (true,0) => Color32::DARK_GREEN,
                            (true,1) => zolty_ciemny,
                            _ =>szarawy_ciemny
                        };
                        match proxy_self.rx.try_recv() {
                            Ok(Ok(ghdfjsas)) => {
                                let danene = ghdfjsas.lock().unwrap();
                                proxy_self.ui_konwersja_specyfic_status_przetwarzania = 2;
                                proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki = 
                                    danene[1];
                                proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki = 
                                    danene[0];
                                proxy_self.ui_konwersja_specyfic_statystyki_czas_sekundy = 
                                    danene[2] as u64;
                                proxy_self.ui_konwersja_specyfic_statystyki_czas_milisekundy = 
                                    danene[3] as u32;
                                #[cfg(not(feature = "raw"))]
                                play_finish_sound(proxy_self.ui_ustawienia_glosnosc);
                            }
                            Ok(Err(e)) => {
                                proxy_self.ui_konwersja_specyfic_status_przetwarzania = 3;
                                proxy_self.ui_konwersja_specyfic_error_3= e.to_string();
                                // eprintln!("Błąd: {}", e);
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => {
                                ctx.request_repaint();
                                (proxy_self.general_ui_loading,
                                 proxy_self.general_ui_loading_tekst) =
                                    animacja(proxy_self.general_ui_loading);

                            }
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                proxy_self.ui_konwersja_specyfic_status_przetwarzania = 4;
                                proxy_self.ui_konwersja_specyfic_error_3 = "Disconected".to_string();
                            }
                        }
                        ui.add_space( proxy_self.formatowanie_spacja_średnia);
                        let t_p_d_foto= match proxy_self.ui_konwersja_specyfic_status_przetwarzania{
                            0 => dodaj_duży_richtext!(
                                tekst_przycisku_kompresji.to_string()),
                            1 => RichText::new(
                                proxy_self.general_ui_loading_tekst.to_string())
                                .font(
                                    wybrana_aktualna_czcionka(
                                        proxy_self.formatowanie_rozmiar_czcionki_duża,
                                        1)).color(Color32::BLACK),
                            2 => dodaj_duży_richtext!(
                                proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
                            3 => dodaj_duży_richtext!(
                                proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
                            4 => dodaj_duży_richtext!(
                                proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
                            _ => dodaj_duży_richtext!(proxy_self.current_language.err_value_overflow)
                        };



                        if ui.put(Rect::from_center_size(
                            Pos2{
                                x:(proxy_self.general_ui_szerokosc_okna / 4.),
                                y:proxy_self.formatowanie_wysokosc_przycisku},
                            Vec2{x:250.,y:40.}),egui::Button::new(t_p_d_foto
                            )
                            .min_size(egui::vec2((
                                             proxy_self.general_ui_szerokosc_okna / 2.) - 100.0,
                                         40.0))
                            .corner_radius(10.)
                            .fill(foto_butt_col))
                            .clicked() && hjgfkjlh{



                                // if hjgfkjlh{
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 0;
                                    proxy_self.general_ui_loading = 0;
                                    let arc_z_foto_rozdzielczość =
                                        Arc::new(Mutex::new(vec![
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64,
                                        proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32
                                        ]));
                                    let arc_z_foto_wybór_formatu =
                                        Arc::new(Mutex::new(vec![
                                        proxy_self.ui_konwersja_specyfic_dane_bool_jpg,
                                        proxy_self.ui_konwersja_specyfic_dane_bool_png,
                                        proxy_self.ui_konwersja_specyfic_dane_bool_png_16,
                                        proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossless,
                                        proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossy,
                                        proxy_self.ui_konwersja_specyfic_dane_bool_tga
                                        ]));
                                    let arc_z_foto_wybór_formatu_jakosc =
                                        Arc::new(Mutex::new(vec![
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_jpg,
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_png,
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_png,
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossless,
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossy,
                                        proxy_self.ui_konwersja_specyfic_dane_jakosc_tga
                                        ]));
                                    let arc_z_foto_paths =
                                        Arc::new(Mutex::new(vec![
                                        proxy_self.ui_konwersja_specyfic_ścieżka_folder_wejściowy.clone(),
                                        proxy_self.ui_konwersja_specyfic_ścieżka_folder_wyjściowy.clone()
                                        ]));
                                    let arc_z_foto_dodatkowe_ustawienia =
                                        Arc::new(Mutex::new(vec![
                                        proxy_self.ui_konwersja_specyfic_dane_filter,
                                        proxy_self.ui_konwersja_specyfic_dane_alpha,
                                        proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                        proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                        ]));

                                    let tx_clone = proxy_self.tx.clone();
                                    std::thread::spawn(move || {
                                        let result =
                                            crate::image_actions::image_actions_main::convert_images(
                                            arc_z_foto_wybór_formatu,
                                            arc_z_foto_wybór_formatu_jakosc,
                                            arc_z_foto_rozdzielczość,
                                            arc_z_foto_paths,
                                            arc_z_foto_dodatkowe_ustawienia,
                                        );
                                        
                                        match tx_clone.send(result) {
                                            Ok(_) => {
                                                #[cfg(debug_assertions)]
                                                println!("Wysłano wynik") },
                                            Err(_e) => {
                                                #[cfg(debug_assertions)]
                                                eprintln!("Błąd wysyłania: {}", _e) },
                                        }
                                    });
                                    
                                    proxy_self.ui_konwersja_specyfic_status_przetwarzania = 1;

                                // }
                            }

                                            

    
                            
}