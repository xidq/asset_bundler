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
use std::{default, sync::{
    Arc, 
    Mutex
}};
use crate::ui::{
    ui_defaults::Appencja,
    ui_play_sound::play_finish_sound,
    ui_change_font::wybrana_aktualna_czcionka
};

pub fn ui_left_panel_foty_przetwarzanie(
    proxy_self: &mut Appencja,
    ctx: &Context,ui: &mut egui::Ui,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32
){
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;
    // ui.add_space(proxy_self.formatowanie_spacja_srednia);

        // ███████  ██████  ██      ██████  ███████ ██████      ██     ██ ███████      ██ ███████  ██████ ██  ██████  ██     ██ ██    ██ 
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██     ██ ██           ██ ██      ██      ██ ██    ██ ██     ██  ██  ██  
        // █████   ██    ██ ██      ██   ██ █████   ██████      ██  █  ██ █████        ██ ███████ ██      ██ ██    ██ ██  █  ██   ████   
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██ ███ ██ ██      ██   ██      ██ ██      ██ ██    ██ ██ ███ ██    ██    
        // ██       ██████  ███████ ██████  ███████ ██   ██      ███ ███  ███████  █████  ███████  ██████ ██  ██████   ███ ███     ██ 

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wejsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.horizontal(|ui|{
                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);
                            let btn_foty_folder_wejsciowy : Response = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_folder).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));

                            if btn_foty_folder_wejsciowy.clicked() {
                                proxy_self.czy_to_juz_koniec = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {

                                    proxy_self.imput_foty_folder_path = path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
                                btn_foty_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
                                btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.imput_foty_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_foty_folder_wejsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_foty_folder_wejsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                            }


                        });
                        if proxy_self.imput_foty_folder_path.len() >= 50 {
                            let xxxx = &proxy_self.imput_foty_folder_path;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
                
                
                        }else if !proxy_self.imput_foty_folder_path.is_empty(){
                            ui.add(egui::Label::new(RichText::new(proxy_self.imput_foty_folder_path.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));

                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                        }else{
                            ui.add_space(proxy_self.formatowanie_spacja_duza);
                        }


                            // ███████  ██████  ██      ██████  ███████ ██████      ██     ██ ██    ██      ██ ███████  ██████ ██  ██████  ██     ██ ██    ██ 
                            // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██     ██  ██  ██       ██ ██      ██      ██ ██    ██ ██     ██  ██  ██  
                            // █████   ██    ██ ██      ██   ██ █████   ██████      ██  █  ██   ████        ██ ███████ ██      ██ ██    ██ ██  █  ██   ████   
                            // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██ ███ ██    ██    ██   ██      ██ ██      ██ ██    ██ ██ ███ ██    ██    
                            // ██       ██████  ███████ ██████  ███████ ██   ██      ███ ███     ██     █████  ███████  ██████ ██  ██████   ███ ███     ██  

                        ui.horizontal(|ui|{
                            ui.add_space(margines_na_wybor_formatu_foty);
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);


                        ui.horizontal(|ui|{
                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);
                            let btn_foty_folder_wyjsciowy : Response = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_folder).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));

                            if btn_foty_folder_wyjsciowy.clicked() {
                                proxy_self.czy_to_juz_koniec_foto = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {
    //(?)
                                    proxy_self.output_foty_folder_path = path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_foty_folder_wyjsciowy = Pos2::new(
                                btn_foty_folder_wyjsciowy.rect.min.x - proxy_self.offset_cirkul,
                                btn_foty_folder_wyjsciowy.rect.min.y+(btn_foty_folder_wyjsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.output_foty_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_foty_folder_wyjsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_foty_folder_wyjsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                            }


                        });
                        if proxy_self.output_foty_folder_path.len() >= 50 {
                            let xxxx = &proxy_self.output_foty_folder_path;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
                
                
                        }else if !proxy_self.output_foty_folder_path.is_empty(){
                            ui.add(egui::Label::new(RichText::new(proxy_self.output_foty_folder_path.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));

                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                        }else{
                            ui.add_space(proxy_self.formatowanie_spacja_duza);
                        }


                            //  █████  ██      ██████  ██   ██  █████  
                            // ██   ██ ██      ██   ██ ██   ██ ██   ██ 
                            // ███████ ██      ██████  ███████ ███████ 
                            // ██   ██ ██      ██      ██   ██ ██   ██ 
                            // ██   ██ ███████ ██      ██   ██ ██   ██


                        ui.separator();

                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_alpha_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });

                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(2,|columns|{
                            columns[0].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_alpha_kolor, 0, RichText::new(proxy_self.current_language.general_colors_white).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::WHITE)).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.selectable_value(&mut proxy_self.foty_alpha_kolor, 1, RichText::new(proxy_self.current_language.general_colors_black).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::BLACK)).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.selectable_value(&mut proxy_self.foty_alpha_kolor, 2, RichText::new(proxy_self.current_language.general_colors_red).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::RED)).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.selectable_value(&mut proxy_self.foty_alpha_kolor, 3, RichText::new(proxy_self.current_language.general_colors_green).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::GREEN)).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.selectable_value(&mut proxy_self.foty_alpha_kolor, 4, RichText::new(proxy_self.current_language.general_colors_blue).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::BLUE)).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            columns[1].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_czy_alpha, 0, RichText::new(proxy_self.current_language.przetwarzanie_bez_alpha).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.selectable_value(&mut proxy_self.foty_czy_alpha, 1, RichText::new(proxy_self.current_language.przetwarzanie_z_alpha).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                
                            });
                        });

                            // ███████ ██ ██      ████████ ██████  ██    ██ 
                            // ██      ██ ██         ██    ██   ██  ██  ██  
                            // █████   ██ ██         ██    ██████    ████   
                            // ██      ██ ██         ██    ██   ██    ██    
                            // ██      ██ ███████    ██    ██   ██    ██    

                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_filter_png_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        
                        ui.columns(5, |column|{
                            column[0].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_filter, 0, RichText::new(proxy_self.current_language.png_specyfic_filter_Nearest).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                            column[1].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_filter, 1, RichText::new(proxy_self.current_language.png_specyfic_filter_Triangle).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                            column[2].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_filter, 2, RichText::new("Catmull\nRom").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                            column[3].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_filter, 3, RichText::new(proxy_self.current_language.png_specyfic_filter_Gaussian).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                            column[4].vertical_centered_justified(|ui|{
                                if ui.selectable_value(&mut proxy_self.foty_filter, 4, RichText::new(proxy_self.current_language.png_specyfic_filter_Lanczos3).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                        });



                            // ██     ██  █████  ██████  ██  █████  ███    ██ ████████ ██    ██ 
                            // ██     ██ ██   ██ ██   ██ ██ ██   ██ ████   ██    ██     ██  ██  
                            // ██  █  ██ ███████ ██████  ██ ███████ ██ ██  ██    ██      ████   
                            // ██ ███ ██ ██   ██ ██   ██ ██ ██   ██ ██  ██ ██    ██       ██    
                            //  ███ ███  ██   ██ ██   ██ ██ ██   ██ ██   ████    ██       ██ 

                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_warianty_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);

                        ui.columns(2, |column|{
                            column[0].vertical(|ui|{
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_jpg, RichText::new("jpg").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                        proxy_self.czy_to_juz_koniec_foto = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_png, RichText::new("png").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_png_16, RichText::new("png 16bit").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                        proxy_self.czy_to_juz_koniec_foto = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_webp, RichText::new("webp").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                        proxy_self.czy_to_juz_koniec_foto = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_webp_lossy, RichText::new("webp lossy").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                        proxy_self.czy_to_juz_koniec_foto = 0
                                    };
                                });
                                ui.horizontal(|ui|{
                                    ui.add_space(margines_na_wybor_formatu_foty);
                                    if ui.checkbox(&mut proxy_self.foty_format_tga, RichText::new("tga").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).clicked(){
                                        proxy_self.czy_to_juz_koniec_foto = 0
                                    };
                                });

                            });
                            column[1].vertical_centered_justified(|ui|{
                                ui.add_enabled(proxy_self.foty_format_jpg,|ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(&mut proxy_self.foty_format_jakosc_jpg, 0..=100))
    
                                });
                                if ui.add_enabled(proxy_self.foty_format_png||proxy_self.foty_format_png_16,|ui: &mut egui::Ui|{
                                    ui.horizontal(|ui|{
                                        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_filter_png_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                                        ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                                        .width(200. / 2.)
                                        .selected_text(RichText::new(
                                            match proxy_self.foty_format_png_filter{
                                                0 => proxy_self.current_language.png_specyfic_filter_png_none,
                                                1=> proxy_self.current_language.png_specyfic_filter_png_sub,
                                                2=>proxy_self.current_language.png_specyfic_filter_png_up,
                                                3=>proxy_self.current_language.png_specyfic_filter_png_avg,
                                                4=>proxy_self.current_language.png_specyfic_filter_png_paeth,
                                                5=>proxy_self.current_language.png_specyfic_filter_png_paeth,
                                                _ => proxy_self.current_language.err_value_overflow
                                            }
                                        ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                                        .show_ui(ui, |ui| {
            
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 0, RichText::new(proxy_self.current_language.png_specyfic_filter_png_none).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 1, RichText::new(proxy_self.current_language.png_specyfic_filter_png_sub).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 2, RichText::new(proxy_self.current_language.png_specyfic_filter_png_up).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 3, RichText::new(proxy_self.current_language.png_specyfic_filter_png_avg).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 4, RichText::new(proxy_self.current_language.png_specyfic_filter_png_paeth).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_png_filter, 5, RichText::new(proxy_self.current_language.png_specyfic_filter_png_paeth).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            }).response

                                        }).response
                                        
                                    // ui.add(egui::Slider::new(&mut proxy_self.foty_format_jakosc_png, 0..=100))
    
                                }).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.add_enabled(proxy_self.foty_format_png||proxy_self.foty_format_png_16,|ui: &mut egui::Ui|{
                                    ui.horizontal(|ui|{
                                        ui.add(egui::Label::new(RichText::new("Kompresja").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                                        ComboBox::from_id_salt("dropbox_png_quality")
                                        .width(140. / 2.)
                                        .selected_text(RichText::new(
                                            match proxy_self.foty_format_jakosc_png{
                                                0 => proxy_self.current_language.png_specyfic_compression_none,
                                                1=> proxy_self.current_language.png_specyfic_compression_default,
                                                2=>proxy_self.current_language.png_specyfic_compression_best,
                                                _ => proxy_self.current_language.err_value_overflow
                                            }
                                        ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                                        .show_ui(ui, |ui| {
            
                                            ui.selectable_value(&mut proxy_self.foty_format_jakosc_png, 0, RichText::new(proxy_self.current_language.png_specyfic_compression_none).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_jakosc_png, 1, RichText::new(proxy_self.current_language.png_specyfic_compression_default).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                            ui.selectable_value(&mut proxy_self.foty_format_jakosc_png, 2, RichText::new(proxy_self.current_language.png_specyfic_compression_best).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                           }).response
                                            
                                        }).response
    
                                }).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };

                                let sfdgdf= ui.add_enabled(false,|ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(&mut proxy_self.foty_format_jakosc_webp, 0..=100))
    
                                });
                                if sfdgdf.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.add_enabled(proxy_self.foty_format_webp_lossy,|ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(&mut proxy_self.foty_format_jakosc_webp_lossy, 0..=100))
    
                                }).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                                if ui.add_enabled(false,|ui: &mut egui::Ui|{
                                    ui.add(egui::Slider::new(&mut proxy_self.foty_format_jakosc_tga, 0..=100))
    
                                }).clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };

                            });
                        });
                            // ██████   ██████  ███████ ██████  ███████ ██ ███████ ██       ██████ ███████  ██████  ███████  ██████ 
                            // ██   ██ ██    ██    ███  ██   ██    ███  ██ ██      ██      ██         ███  ██    ██ ██      ██      
                            // ██████  ██    ██   ███   ██   ██   ███   ██ █████   ██      ██        ███   ██    ██ ███████ ██      
                            // ██   ██ ██    ██  ███    ██   ██  ███    ██ ██      ██      ██       ███    ██    ██      ██ ██      
                            // ██   ██  ██████  ███████ ██████  ███████ ██ ███████ ███████  ██████ ███████  ██████  ███████  ██████
                            ui.add_space(proxy_self.formatowanie_spacja_srednia);
                            ui.separator();
                            ui.add_space(proxy_self.formatowanie_spacja_srednia);
                            ui.vertical_centered_justified(|ui|{
                                ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_rozdzielczosc_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                            });
                            ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(10, |column|{
                            column[0].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_16k_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_16k, RichText::new("16k").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_16k_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_16k{
                                        proxy_self.foty_rozdzielczosc_16k=true
                                    } else if proxy_self.foty_rozdzielczosc_16k{
                                        proxy_self.foty_rozdzielczosc_16k=false
                                    }
                                }
                                if foty_rozdzielczosc_16k_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                            column[1].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_8k_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_8k, RichText::new("8k").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_8k_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_8k{
                                        proxy_self.foty_rozdzielczosc_8k=true
                                    } else if proxy_self.foty_rozdzielczosc_8k{
                                        proxy_self.foty_rozdzielczosc_8k=false
                                    }
                                }
                                if foty_rozdzielczosc_8k_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[2].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_4k_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_4k, RichText::new("4k").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_4k_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_4k{
                                        proxy_self.foty_rozdzielczosc_4k=true
                                    } else if proxy_self.foty_rozdzielczosc_4k{
                                        proxy_self.foty_rozdzielczosc_4k=false
                                    }
                                }
                                if foty_rozdzielczosc_4k_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[3].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_2k_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_2k, RichText::new("2k").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_2k_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_2k{
                                        proxy_self.foty_rozdzielczosc_2k=true
                                    } else if proxy_self.foty_rozdzielczosc_2k{
                                        proxy_self.foty_rozdzielczosc_2k=false
                                    }
                                }
                                if foty_rozdzielczosc_2k_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[4].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_1k_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_1k, RichText::new("1k").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_1k_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_1k{
                                        proxy_self.foty_rozdzielczosc_1k=true
                                    } else if proxy_self.foty_rozdzielczosc_1k{
                                        proxy_self.foty_rozdzielczosc_1k=false
                                    }
                                }
                                if foty_rozdzielczosc_1k_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[5].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_512_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_512, RichText::new("512").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_512_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_512{
                                        proxy_self.foty_rozdzielczosc_512=true
                                    } else if proxy_self.foty_rozdzielczosc_512{
                                        proxy_self.foty_rozdzielczosc_512=false
                                    }
                                }
                                if foty_rozdzielczosc_512_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[6].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_256_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_256, RichText::new("256").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_256_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_256{
                                        proxy_self.foty_rozdzielczosc_256=true
                                    } else if proxy_self.foty_rozdzielczosc_256{
                                        proxy_self.foty_rozdzielczosc_256=false
                                    }
                                }
                                if foty_rozdzielczosc_256_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[7].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_128_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_128, RichText::new("128").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_128_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_128{
                                        proxy_self.foty_rozdzielczosc_128=true
                                    } else if proxy_self.foty_rozdzielczosc_128{
                                        proxy_self.foty_rozdzielczosc_128=false
                                    }
                                }
                                if foty_rozdzielczosc_128_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[8].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_64_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_64, RichText::new("64").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_64_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_64{
                                        proxy_self.foty_rozdzielczosc_64=true
                                    } else if proxy_self.foty_rozdzielczosc_64{
                                        proxy_self.foty_rozdzielczosc_64=false
                                    }
                                }
                                if foty_rozdzielczosc_64_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });
                            column[9].vertical_centered_justified(|ui|{
                                let foty_rozdzielczosc_32_wybor = ui.selectable_label(proxy_self.foty_rozdzielczosc_32, RichText::new("32").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if foty_rozdzielczosc_32_wybor.clicked(){
                                    if !proxy_self.foty_rozdzielczosc_32{
                                        proxy_self.foty_rozdzielczosc_32=true
                                    } else if proxy_self.foty_rozdzielczosc_32{
                                        proxy_self.foty_rozdzielczosc_32=false
                                    }
                                }
                                if foty_rozdzielczosc_32_wybor.clicked(){
                                    proxy_self.czy_to_juz_koniec_foto = 0
                                };
                            });

                        });




                        // ██████  ██████  ███████ ██    ██  ██████ ██ ███████ ██   ██ 
                        // ██   ██ ██   ██    ███   ██  ██  ██      ██ ██      ██  ██  
                        // ██████  ██████    ███     ████   ██      ██ ███████ █████   
                        // ██      ██   ██  ███       ██    ██      ██      ██ ██  ██  
                        // ██      ██   ██ ███████    ██     ██████ ██ ███████ ██   ██ 


                        let sprawdzacz_przycisku_fot= if !proxy_self.imput_foty_folder_path.is_empty() && 
                            !proxy_self.output_foty_folder_path.is_empty() && 
                            (
                                proxy_self.foty_format_jpg || proxy_self.foty_format_png || 
                                proxy_self.foty_format_png || proxy_self.foty_format_png_16 || 
                                proxy_self.foty_format_tga || proxy_self.foty_format_webp || 
                                proxy_self.foty_format_webp_lossy
                            ) && (
                                proxy_self.foty_rozdzielczosc_16k || proxy_self.foty_rozdzielczosc_8k ||
                                proxy_self.foty_rozdzielczosc_4k || proxy_self.foty_rozdzielczosc_2k ||
                                proxy_self.foty_rozdzielczosc_1k || proxy_self.foty_rozdzielczosc_512 ||
                                proxy_self.foty_rozdzielczosc_256 || proxy_self.foty_rozdzielczosc_128 ||
                                proxy_self.foty_rozdzielczosc_64 || proxy_self.foty_rozdzielczosc_32
                            ){
                            true
                        }else{
                            false
                        };

                        let tekst_przycisku_kompresji = if sprawdzacz_przycisku_fot{

                            &proxy_self.current_language.szyfrowanie_przycisk_ok}
                            else
                            {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                        

                        let hjgfkjlh = if !proxy_self.imput_foty_folder_path.is_empty() && proxy_self.czy_to_juz_koniec_foto !=1 && !proxy_self.output_foty_folder_path.is_empty() &&  !proxy_self.zapis_pracuje{true}else{false};
                        let foto_butt_col = if sprawdzacz_przycisku_fot == true && proxy_self.czy_to_juz_koniec_foto ==0{Color32::DARK_GREEN}else if sprawdzacz_przycisku_fot == true && proxy_self.czy_to_juz_koniec_foto ==1{zolty_ciemny}else{szarawy_ciemny};

                        match proxy_self.rx.try_recv() {
                            Ok(Ok(ghdfjsas)) => {
                                let danene = ghdfjsas.lock().unwrap();
                                proxy_self.czy_to_juz_koniec_foto = 2;
                                proxy_self.foto_utworzone_pliki = danene[1];
                                proxy_self.foto_przetworzone_pliki = danene[0];
                                proxy_self.foto_timestamp_sekundy = danene[2] as u64;
                                proxy_self.foto_timestamp_milisekundy = danene[3] as u32;
                                play_finish_sound(proxy_self.ustawienia_glosnosc);
                            }
                            Ok(Err(e)) => {
                                proxy_self.czy_to_juz_koniec_foto = 3;
                                proxy_self.szyfrowanie_err_3 = e.to_string();
                                // eprintln!("Błąd: {}", e);
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => {
                                ctx.request_repaint();
                                match proxy_self.sz_loading{
                                    181 => {proxy_self.sz_loading = 0; proxy_self.sz_loading_anim=""},
                                    151..=180 => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[=====>]"},
                                    121..=150 => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[====>-]"},
                                    91..=120 => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[===>--]"},
                                    61..=90 => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[==>---]"},
                                    30..=60 => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[=>----]"},
                                    _ => {proxy_self.sz_loading += 1; proxy_self.sz_loading_anim="[>-----]"},
                                }  

                            }
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                proxy_self.czy_to_juz_koniec_foto = 4;
                                proxy_self.szyfrowanie_err_3 = "Disconected".to_string();
                            }
                        }
                        ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        let t_p_d_foto= match proxy_self.czy_to_juz_koniec_foto{
                            0 => RichText::new(tekst_przycisku_kompresji.to_string()),
                            1 => RichText::new(proxy_self.sz_loading_anim.to_string()).monospace().color(Color32::BLACK),
                            2 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
                            3 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
                            4 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
                            _ => RichText::new("".to_string())
                        };



                        if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.szerokosc_okna / 4.),y:proxy_self.wysokosc_btn_egzekucyjny},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d_foto
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                            .min_size(egui::vec2(250.0, 40.0))
                            .corner_radius(10.)
                            .fill(foto_butt_col))
                            .clicked() {



                                if hjgfkjlh{
                                    proxy_self.czy_to_juz_koniec_foto = 0;
                                    let arc_z_foto_rozdzielczosc = Arc::new(Mutex::new(vec![
                                        proxy_self.foty_rozdzielczosc_16k,
                                        proxy_self.foty_rozdzielczosc_8k,
                                        proxy_self.foty_rozdzielczosc_4k,
                                        proxy_self.foty_rozdzielczosc_2k,
                                        proxy_self.foty_rozdzielczosc_1k,
                                        proxy_self.foty_rozdzielczosc_512,
                                        proxy_self.foty_rozdzielczosc_256,
                                        proxy_self.foty_rozdzielczosc_128,
                                        proxy_self.foty_rozdzielczosc_64,
                                        proxy_self.foty_rozdzielczosc_32
                                        ]));
                                    let arc_z_foto_wybor_formatu = Arc::new(Mutex::new(vec![
                                        proxy_self.foty_format_jpg,
                                        proxy_self.foty_format_png,
                                        proxy_self.foty_format_png_16,
                                        proxy_self.foty_format_webp,
                                        proxy_self.foty_format_webp_lossy,
                                        proxy_self.foty_format_tga
                                        ]));
                                    let arc_z_foto_wybor_formatu_jakosc = Arc::new(Mutex::new(vec![
                                        proxy_self.foty_format_jakosc_jpg,
                                        proxy_self.foty_format_jakosc_png,
                                        proxy_self.foty_format_jakosc_png,
                                        proxy_self.foty_format_jakosc_webp,
                                        proxy_self.foty_format_jakosc_webp_lossy,
                                        proxy_self.foty_format_jakosc_tga
                                        ]));
                                    let arc_z_foto_paths = Arc::new(Mutex::new(vec![
                                        proxy_self.imput_foty_folder_path.clone(),
                                        proxy_self.output_foty_folder_path.clone()
                                        ]));
                                    let arc_z_foto_dodatkowe_ustawienia = Arc::new(Mutex::new(vec![
                                        proxy_self.foty_filter,
                                        proxy_self.foty_czy_alpha,
                                        proxy_self.foty_alpha_kolor,
                                        proxy_self.foty_format_png_filter,
                                        ]));

                                    let tx_clone = proxy_self.tx.clone();
                                    std::thread::spawn(move || {
                                        let result = crate::image_actions::image_actions_main::convert_images(
                                            arc_z_foto_wybor_formatu,
                                            arc_z_foto_wybor_formatu_jakosc,
                                            arc_z_foto_rozdzielczosc,
                                            arc_z_foto_paths,
                                            arc_z_foto_dodatkowe_ustawienia,
                                        );
                                        
                                        match tx_clone.send(result) {
                                            Ok(_) => println!("Wysłano wynik"),
                                            Err(e) => eprintln!("Błąd wysyłania: {}", e),
                                        }
                                    });
                                    
                                    proxy_self.czy_to_juz_koniec_foto = 1;

                                }}

                                            

                            
}