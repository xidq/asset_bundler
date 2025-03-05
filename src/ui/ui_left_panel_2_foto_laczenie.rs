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
use egui::TextEdit;
use image::open;

use rfd::FileDialog;
use std::{default, path::PathBuf, sync::{
    Arc, 
    Mutex
}};
use crate::ui::{
    ui_defaults::Appencja,
    ui_play_sound::play_finish_sound,
    ui_change_font::wybrana_aktualna_czcionka
};

pub fn ui_left_panel_foty_laczenie(
    proxy_self: &mut Appencja,
    ctx: &Context,ui: &mut egui::Ui,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32
){
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;
    let aspect_ratio_check = 
    if (proxy_self.pakowanie_foty_red_aspect_ratio == proxy_self.pakowanie_foty_green_aspect_ratio) &&
    (proxy_self.pakowanie_foty_red_aspect_ratio == proxy_self.pakowanie_foty_blue_aspect_ratio){true}else{false};



        // ███████  ██████  ██      ██████  ███████ ██████      ██     ██ ███████      ██ ███████  ██████ ██  ██████  ██     ██ ██    ██ 
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██     ██ ██           ██ ██      ██      ██ ██    ██ ██     ██  ██  ██  
        // █████   ██    ██ ██      ██   ██ █████   ██████      ██  █  ██ █████        ██ ███████ ██      ██ ██    ██ ██  █  ██   ████   
        // ██      ██    ██ ██      ██   ██ ██      ██   ██     ██ ███ ██ ██      ██   ██      ██ ██      ██ ██    ██ ██ ███ ██    ██    
        // ██       ██████  ███████ ██████  ███████ ██   ██      ███ ███  ███████  █████  ███████  ██████ ██  ██████   ███ ███     ██ 

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general.to_owned()+" r")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);

    //przestrzeń przycisku
    ui.horizontal(|ui|{
        ui.add_space(
            proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty
        );
        //button
        let btn_foty_folder_wejsciowy : Response = ui.button(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general)
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.czy_to_juz_koniec = 0;
            if let Some(path) = FileDialog::new().pick_file() {

                proxy_self.imput_r_file_path = path.clone();
            }


                            // Załaduj zdjęcie i oblicz aspect ratio
            if let Ok(image) = open(&proxy_self.imput_r_file_path) {
                let width = image.width();
                let height = image.height();

                // Oblicz aspect ratio
                proxy_self.pakowanie_foty_red_aspect_ratio = width as f32 / height as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.imput_r_file_path.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.full_cirkul_sajz, 
                proxy_self.full_cirkul_kolor
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.pusty_cirkul_sajz, 
                (proxy_self.pusty_cirkul_ma_stroke,
                proxy_self.pusty_cirkul_kolor)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.pakowanie_foty_red_aspect_ratio))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
            .color(Color32::RED))
                .selectable(false)
            );
        }


        
    });
    if !proxy_self.imput_r_file_path.to_string_lossy().is_empty(){
        //napis sciezki
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.imput_r_file_path.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }


    //przycisk2

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general.to_owned()+" g")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);

    //przestrzeń przycisku
    ui.horizontal(|ui|{
        ui.add_space(
            proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty
        );
        //button
        let btn_foty_folder_wejsciowy : Response = ui.button(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general)
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.czy_to_juz_koniec = 0;
            if let Some(path) = FileDialog::new().pick_file() {

                proxy_self.imput_g_file_path = path.clone();

            }
            if let Ok(image) = open(&proxy_self.imput_g_file_path) {
                let width = image.width();
                let height = image.height();

                // Oblicz aspect ratio
                proxy_self.pakowanie_foty_green_aspect_ratio = width as f32 / height as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.imput_g_file_path.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.full_cirkul_sajz, 
                proxy_self.full_cirkul_kolor
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.pusty_cirkul_sajz, 
                (proxy_self.pusty_cirkul_ma_stroke,
                proxy_self.pusty_cirkul_kolor)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.pakowanie_foty_green_aspect_ratio))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                .color(Color32::RED))
                .selectable(false)
            );
        }
    });
    if !proxy_self.imput_g_file_path.to_string_lossy().is_empty(){
        //napis sciezki
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.imput_g_file_path.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }




    //przycisk3

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general.to_owned()+" b")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);

    //przestrzeń przycisku
    ui.horizontal(|ui|{
        ui.add_space(
            proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty
        );
        //button
        let btn_foty_folder_wejsciowy : Response = ui.button(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general)
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.czy_to_juz_koniec = 0;
            if let Some(path) = FileDialog::new().pick_file() {

                proxy_self.imput_b_file_path = path.clone();

            }
            if let Ok(image) = open(&proxy_self.imput_b_file_path) {
                let width = image.width();
                let height = image.height();

                // Oblicz aspect ratio
                proxy_self.pakowanie_foty_blue_aspect_ratio = width as f32 / height as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.imput_b_file_path.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.full_cirkul_sajz, 
                proxy_self.full_cirkul_kolor
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.pusty_cirkul_sajz, 
                (proxy_self.pusty_cirkul_ma_stroke,
                proxy_self.pusty_cirkul_kolor)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.pakowanie_foty_blue_aspect_ratio))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                .color(Color32::RED))
                .selectable(false)
            );
        }
    });
    if !proxy_self.imput_b_file_path.to_string_lossy().is_empty(){
        //napis sciezki
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.imput_b_file_path.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }




    //przycisk4

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy)
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);

    //przestrzeń przycisku
    ui.horizontal(|ui|{
        ui.add_space(
            proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty
        );
        //button
        let btn_foty_folder_wejsciowy : Response = ui.button(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general)
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.czy_to_juz_koniec = 0;
            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.imput_out_folder_path = path.clone();

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.imput_out_folder_path.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.full_cirkul_sajz, 
                proxy_self.full_cirkul_kolor
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.pusty_cirkul_sajz, 
                (proxy_self.pusty_cirkul_ma_stroke,
                proxy_self.pusty_cirkul_kolor)
            );

        }

        if proxy_self.imput_b_file_path.to_string_lossy().len() < 28{
            //napis sciezki
            ui.add(egui::Label::new(
                RichText::new(proxy_self.imput_b_file_path.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .selectable(false)
            );

        }
    });
    if proxy_self.imput_b_file_path.to_string_lossy().len() >=28{
        //napis sciezki
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.imput_b_file_path.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }


    // ui.separator();
    // ui.add_space(proxy_self.formatowanie_spacja_srednia);










    // ui.vertical_centered_justified(|ui|{

    //     ui.add(egui::Label::new(
    //         RichText::new(
    //             "Głębia bitów")
    //             .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
    //             .selectable(false)
    //         );
    // });
    // ui.add_space(proxy_self.formatowanie_spacja_mala);

    // ui.columns(2,|column|{
    //     column[0].vertical_centered_justified(|ui|{
    //         if ui.selectable_value(&mut proxy_self.pakowanie_foty_bit_depth, 8, RichText::new("8bit").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::WHITE)).clicked(){
    //             proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
    //         };
    //     });
    //     column[1].vertical_centered_justified(|ui|{
    //         if ui.selectable_value(&mut proxy_self.pakowanie_foty_bit_depth, 16, RichText::new("16bit").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).color(Color32::WHITE)).clicked(){
    //             proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
    //         };
    //     });
    // });


    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                "Rozszerzenie")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.columns(3, |column|{
        column[0].vertical_centered_justified(|ui|{
            if ComboBox::from_id_salt("kakbjkbtskj"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
            .width(80.)
            .selected_text(RichText::new(
                match proxy_self.pakowanie_foty_rozszerzenie{
                    0 => "jpg",
                    1=> "png",
                    2=>"png 16bit",
                    3=>"webp",
                    4=>"tga",
                    _ => "lol"
                }
                ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                .show_ui(ui, |ui| {

                ui.selectable_value(&mut proxy_self.pakowanie_foty_rozszerzenie, 0, RichText::new("jpg").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_rozszerzenie, 1, RichText::new("png").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_rozszerzenie, 2, RichText::new("png16").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_rozszerzenie, 3, RichText::new("webp").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_rozszerzenie, 4, RichText::new("tga").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                }).response.clicked(){
                    proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
                }

        });
        column[1].vertical_centered_justified(|ui|{
            let sfdsdsgasdfe = match proxy_self.pakowanie_foty_rozszerzenie{
                0=>{
                        proxy_self.pakowanie_foty_jakosc=75;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                            ui.add(egui::Slider::new(&mut proxy_self.pakowanie_foty_jakosc, 0..=100).show_value(true))
                        });
                    },
                1|2=>{
                        proxy_self.pakowanie_foty_jakosc=1;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                            ui.add(egui::Slider::new(&mut proxy_self.pakowanie_foty_jakosc, 0..=2).show_value(true))
                        });
                    },
                3=>{
                        proxy_self.pakowanie_foty_jakosc=75;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                        ui.add(egui::Slider::new(&mut proxy_self.pakowanie_foty_jakosc, 0..=100).show_value(true));
                        });
                    },
                _=> {ui.label("");}
            };
        });
        column[2].vertical_centered_justified(|ui|{
            let sfdsdsgasdfe = match proxy_self.pakowanie_foty_rozszerzenie{
                1|2=>{
                    if ComboBox::from_id_salt("kakbgrfgdskj"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                    .width(80.)
                    .selected_text(RichText::new(
                        match proxy_self.pakowanie_foty_png_filter{
                            0 => proxy_self.current_language.png_specyfic_filter_png_none,
                            1=> proxy_self.current_language.png_specyfic_filter_png_sub,
                            2=>proxy_self.current_language.png_specyfic_filter_png_up,
                            3=>proxy_self.current_language.png_specyfic_filter_png_avg,
                            4=>proxy_self.current_language.png_specyfic_filter_png_paeth,
                            5=>proxy_self.current_language.png_specyfic_filter_png_adaptive,
                            _ => proxy_self.current_language.err_value_overflow
                        }
                        ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                        .show_ui(ui, |ui| {
        
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 0, RichText::new(proxy_self.current_language.png_specyfic_filter_png_none).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 1, RichText::new(proxy_self.current_language.png_specyfic_filter_png_sub).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 2, RichText::new(proxy_self.current_language.png_specyfic_filter_png_up).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 3, RichText::new(proxy_self.current_language.png_specyfic_filter_png_avg).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 4, RichText::new(proxy_self.current_language.png_specyfic_filter_png_paeth).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.pakowanie_foty_png_filter, 5, RichText::new(proxy_self.current_language.png_specyfic_filter_png_adaptive).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        }).response.clicked(){
                            proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
                        }},

                _=> {ui.label("");}
            };
        });
    });


    match proxy_self.pakowanie_foty_png_filter{
        0|1|3|4 => proxy_self.pakowanie_foty_bit_depth = 8,
        2 => proxy_self.pakowanie_foty_bit_depth = 16,
        _ => proxy_self.pakowanie_foty_bit_depth = 69
    };






    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                "Rozdzielczość")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.vertical_centered_justified(|ui|{
        if ComboBox::from_id_salt("gafdgsd"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
        .width(320.)
        .selected_text(RichText::new(
            match proxy_self.pakowanie_foty_rozdzielczosc{
                0 => "16k",
                1=> "8k",
                2=>"4k",
                3=>"2k",
                4=>"1k",
                5=>"512",
                6=>"256",
                7=>"128",
                8=>"64",
                9=>"32",
                _ => proxy_self.current_language.err_value_overflow
            }
            ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 0, RichText::new("16384px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 1, RichText::new("8192px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 2, RichText::new("4096px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 3, RichText::new("2048px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 4, RichText::new("1024px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 5, RichText::new("512px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 6, RichText::new("256px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 7, RichText::new("128px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 8, RichText::new("64px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.pakowanie_foty_rozdzielczosc, 9, RichText::new("32px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                );
        }).response.clicked(){
                proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
            }

    });





    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                "Filtr")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.vertical_centered_justified(|ui|{
        if ComboBox::from_id_salt("afgsh"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
        .width(320.)
        .selected_text(RichText::new(
            match proxy_self.pakowanie_foty_filter{
                0 => proxy_self.current_language.png_specyfic_filter_Nearest,
                1=> proxy_self.current_language.png_specyfic_filter_Triangle,
                2=> proxy_self.current_language.png_specyfic_filter_CatmullRom,
                3=>proxy_self.current_language.png_specyfic_filter_Gaussian,
                4=>proxy_self.current_language.png_specyfic_filter_Lanczos3,
                _ => proxy_self.current_language.err_value_overflow
            }
            ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut proxy_self.pakowanie_foty_filter, 0, RichText::new(proxy_self.current_language.png_specyfic_filter_Nearest).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_filter, 1, RichText::new(proxy_self.current_language.png_specyfic_filter_Triangle).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_filter, 2, RichText::new(proxy_self.current_language.png_specyfic_filter_CatmullRom).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_filter, 3, RichText::new(proxy_self.current_language.png_specyfic_filter_Gaussian).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.pakowanie_foty_filter, 4, RichText::new(proxy_self.current_language.png_specyfic_filter_Lanczos3).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                }).response.clicked(){
                proxy_self.pakowanie_foty_czy_to_juz_koniec = 0
            }

    });





    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();




    //  ___ _ _                          
    // |  _|_| |___    ___ ___ _____ ___ 
    // |  _| | | -_|  |   | .'|     | -_|
    // |_| |_|_|___|  |_|_|__,|_|_|_|___|

    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.vertical_centered_justified(|ui|{
        ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_nazwa_tytul)
        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
    });        

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    let output_name_input = TextEdit::singleline(&mut proxy_self.pakowanie_foty_nazwa)
        .char_limit(50)
        .min_size(egui::Vec2{x:200.,y:30.})
        .hint_text(&proxy_self.current_language.general_ui_nazwa.to_string())
        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki));
    ui.vertical_centered_justified(|ui|{
        ui.add(output_name_input);
    });

    ui.add_space( proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);









    //przycisk out
    // ██████  ██████  ███████ ██    ██  ██████ ██ ███████ ██   ██ 
    // ██   ██ ██   ██    ███   ██  ██  ██      ██ ██      ██  ██  
    // ██████  ██████    ███     ████   ██      ██ ███████ █████   
    // ██      ██   ██  ███       ██    ██      ██      ██ ██  ██  
    // ██      ██   ██ ███████    ██     ██████ ██ ███████ ██   ██ 


    let sprawdzacz_przycisku_fotx= if 
        !proxy_self.imput_r_file_path.to_string_lossy().is_empty() && 
        !proxy_self.imput_g_file_path.to_string_lossy().is_empty() && 
        !proxy_self.imput_b_file_path.to_string_lossy().is_empty() && 
        !proxy_self.imput_out_folder_path.to_string_lossy().is_empty() &&
        aspect_ratio_check{
        true
    }else{
        false
    };

    let tekst_przycisku_kompresji = if sprawdzacz_przycisku_fotx{

        &proxy_self.current_language.szyfrowanie_przycisk_ok}
        else
        {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};


    let hjgfkjlh = if sprawdzacz_przycisku_fotx && proxy_self.pakowanie_foty_czy_to_juz_koniec !=1 {true}else{false};

    let fotox_butt_col = match (sprawdzacz_przycisku_fotx,proxy_self.pakowanie_foty_czy_to_juz_koniec){
        (true,0)=>Color32::DARK_GREEN,
        (true,1)=>zolty_ciemny,
        _=>szarawy_ciemny
    };
    match proxy_self.fotx_rx.try_recv() {
        Ok(Ok(ghdfjsasx)) => {
            let danene = ghdfjsasx.lock().unwrap();
            proxy_self.pakowanie_foty_czy_to_juz_koniec = 2;
            proxy_self.foto_timestamp_sekundy = danene[0] as u64;
            proxy_self.foto_laczenie_timestamp_milisekundy = danene[1] as u32;
            play_finish_sound(proxy_self.ustawienia_glosnosc);
        }
        Ok(Err(e)) => {
            proxy_self.pakowanie_foty_czy_to_juz_koniec = 3;
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
            proxy_self.pakowanie_foty_czy_to_juz_koniec = 4;
            proxy_self.szyfrowanie_err_3 = "Disconected".to_string();
        }
    }
    ui.add_space( proxy_self.formatowanie_spacja_srednia);
    let t_p_d_fotox= match proxy_self.pakowanie_foty_czy_to_juz_koniec{
        0 => RichText::new(tekst_przycisku_kompresji.to_string()),
        1 => RichText::new(proxy_self.sz_loading_anim.to_string()).monospace().color(Color32::BLACK),
        2 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
        3 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
        4 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
        _ => RichText::new("".to_string())
    };



    if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.szerokosc_okna / 4.),y:proxy_self.wysokosc_btn_egzekucyjny},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d_fotox
        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
        .min_size(egui::vec2(250.0, 40.0))
        .corner_radius(10.)
        .fill(fotox_butt_col))
        .clicked() {
            println!("kliklem przycisk!");



            if hjgfkjlh{
                proxy_self.pakowanie_foty_czy_to_juz_koniec = 0;
                let arc_z_path = Arc::new(Mutex::new(vec![
                    proxy_self.imput_r_file_path.clone(),
                    proxy_self.imput_g_file_path.clone(),
                    proxy_self.imput_b_file_path.clone(),
                    proxy_self.imput_out_folder_path.clone()
                    ]));
                println!("{:?}",arc_z_path);
                let arc_z_string = Arc::new(Mutex::new(vec![
                    proxy_self.pakowanie_foty_nazwa.clone()
                    ]));
                    println!("{:?}",arc_z_string);
                let arc_z_u8 = Arc::new(Mutex::new(vec![
                    proxy_self.pakowanie_foty_rozdzielczosc,
                    proxy_self.pakowanie_foty_png_filter,
                    proxy_self.pakowanie_foty_bit_depth,
                    proxy_self.pakowanie_foty_filter,
                    proxy_self.pakowanie_foty_jakosc,
                    proxy_self.pakowanie_foty_png_filter
                    ]));
                    println!("{:?}",arc_z_u8);



                let tx_clonea = proxy_self.fotx_tx.clone();
                std::thread::spawn(move || {
                    let result = crate::image_channel_bundler::image_channel_bundler_main::image_channel_bundler(
                        arc_z_path,
                        arc_z_string,
                        arc_z_u8,
                    );
                    println!("ui w thread!");
                    
                    match tx_clonea.send(result) {
                        Ok(_) => println!("Wysłano wynik"),
                        Err(e) => eprintln!("Błąd wysyłania: {}", e),
                    }
                });
                println!("ui po thread!");
                
                proxy_self.pakowanie_foty_czy_to_juz_koniec = 1;

            }}

                        





}