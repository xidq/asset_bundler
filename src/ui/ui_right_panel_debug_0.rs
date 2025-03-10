use egui::{
    Color32,  
    // Pos2,
    RichText,
    // TextEdit,
    Context,
};
// use std::path::Path;
// use chrono::Local;
use std::f32::consts::PI;
// use rfd::FileDialog;
// use crate::decrypt_copy;
#[cfg(not(feature = "raw"))]
use crate::ui::ui_play_sound::play_ahh_sound;
use crate::ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka
};


pub fn ui_right_panel_debug_0(proxy_self: &mut Appencja,_ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32){
    #[cfg(not(feature = "raw"))]
    if proxy_self.ui_debug_specyfic_klikacz < 50{
        let to_je_foto = ui.add(egui::Image::new(egui::include_image!("../br/ok.png")).sense(egui::Sense::click())
        .max_height(330.0)
        .max_width(300.0)
        .maintain_aspect_ratio(true)
        .fit_to_original_size(1.)
        .rotate( proxy_self.ui_debug_specyfic_obrot, egui::Vec2::splat(0.5))
        // .rounding(10.0));
        
        );
        if to_je_foto.clicked() {
            play_ahh_sound(proxy_self.ui_ustawienia_glosnosc);
            proxy_self.ui_debug_specyfic_obrot += rand::random_range(0.0..=PI);
            proxy_self.ui_debug_specyfic_klikacz+=1;
            
        }
    }else{
        let to_je_foto = ui.add(egui::Image::new(egui::include_image!("../br/chunky-cat-out.gif")).sense(egui::Sense::click())
        .max_height(330.0)
        .max_width(300.0)
        .maintain_aspect_ratio(true)
        .fit_to_original_size(1.)
        .rotate( proxy_self.ui_debug_specyfic_obrot, egui::Vec2::splat(0.5))
        // .rounding(10.0));
        
        );

        if to_je_foto.clicked() {
            play_ahh_sound(proxy_self.ui_ustawienia_glosnosc);
            proxy_self.ui_debug_specyfic_obrot += rand::random_range(0.0..=PI);
            proxy_self.ui_debug_specyfic_obrot %= 2.0 * PI; 
            proxy_self.ui_debug_specyfic_klikacz+=1;
            
        }
    }

    ui.add(egui::Label::new(proxy_self.ui_debug_specyfic_klikacz.to_string()).selectable(false));


        //  _                   
        // |_|_____ ___ ___ ___ 
        // | |     | .'| . | -_|
        // |_|_|_|_|__,|_  |___|
        //             |___|   





        ui.add(egui::Label::new(proxy_self.current_language.ukryte_sktory).selectable(false));

        ui.add_space(10.);

        ui.separator();

        ui.columns(2,|columns|{
            columns[0].vertical_centered_justified(|ui|{
                ui.add(egui::Label::new(RichText::new("mala czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            });
            columns[1].vertical_centered_justified(|ui|{
                ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_mala,6f32..=14f32));
            });
        });
        ui.columns(2,|columns|{
            columns[0].vertical_centered_justified(|ui|{
                ui.add(egui::Label::new(RichText::new("srednia czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            });
            columns[1].vertical_centered_justified(|ui|{
                ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_srednia,10f32..=18f32));
            });
        });
        ui.columns(2,|columns|{
            columns[0].vertical_centered_justified(|ui|{
                ui.add(egui::Label::new(RichText::new("duza czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            });
            columns[1].vertical_centered_justified(|ui|{
                let slider_response = ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_duza, 14f32..=22f32));

                // Jeśli slider został dwukrotnie kliknięty, ustaw wartość na 18.
                if slider_response.double_clicked() {
                    proxy_self.formatowanie_rozmiar_czcionki_duza = 18.0;
                }
            });
        });

        // ui.add(egui::Label::new(RichText::new(&proxy_self.current_language.debug_deszyfracja_idx.to_string()).strikethrough()).selectable(false));

        // ui.add(egui::Label::new(RichText::new(&proxy_self.current_language.general_ui_wybierz_plik_idx.to_string()).strikethrough()).selectable(false));

        // ui.horizontal(|ui|{

        //     ui.add_space(20.);

        //     let btn_debug_plik_idx = ui.button(&proxy_self.current_language.general_ui_wybierz_plik_idx.to_string());
        //     if btn_debug_plik_idx.clicked() {

        //         if let Some(path) = FileDialog::new().pick_file() {
        //             proxy_self.debug_imput_idx_file = path.to_string_lossy().to_string();
        //         }
        //     }

        //     let btn_pozycjonowanie_debug_plik_idx = Pos2::new(
        //         btn_debug_plik_idx.rect.min.x - proxy_self.offset_cirkul,
        //         btn_debug_plik_idx.rect.min.y+(btn_debug_plik_idx.rect.size().y / 2.));
            
        //     if !proxy_self.debug_imput_idx_file.is_empty(){

        //         ui.painter().circle_filled(btn_pozycjonowanie_debug_plik_idx, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

        //     } else {

        //         ui.painter().circle_stroke(btn_pozycjonowanie_debug_plik_idx, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

        //     }

        //     ui.add(egui::Label::new(&proxy_self.debug_imput_idx_file).selectable(false));

        // });

        // ui.add(egui::Label::new(&proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy.to_string()).selectable(false));

        // ui.horizontal(|ui|{

        //     ui.add_space(20.);

        //     let btn_debug_folder_wyjsciowy = ui.button(&proxy_self.current_language.general_ui_wybierz_folder.to_string());
        //     if btn_debug_folder_wyjsciowy.clicked() {

        //         if let Some(path) = FileDialog::new().pick_folder() {
        //             proxy_self.debug_output_idx_path = path.to_string_lossy().to_string()+"/";
        //         }

        //     }
        //     if proxy_self.debug_output_idx_path.len() < 28{

        //         ui.add(egui::Label::new(&proxy_self.debug_output_idx_path).selectable(false));

        //     }

        //     let btn_pozycjonowanie_debug_folder_wyjsciowy = Pos2::new(
        //         btn_debug_folder_wyjsciowy.rect.min.x - proxy_self.offset_cirkul,
        //         btn_debug_folder_wyjsciowy.rect.min.y+(btn_debug_folder_wyjsciowy.rect.size().y / 2.));
            
        //     if !proxy_self.debug_output_idx_path.is_empty(){

        //         ui.painter().circle_filled(btn_pozycjonowanie_debug_folder_wyjsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

        //     } else {

        //         ui.painter().circle_stroke(btn_pozycjonowanie_debug_folder_wyjsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke, proxy_self.pusty_cirkul_kolor));

        //     }
            

        // });

        // if proxy_self.debug_output_idx_path.len() >= 28{

        //     ui.add(egui::Label::new(&proxy_self.debug_output_idx_path).selectable(false));

        // }

        // if proxy_self.toggle_encryption == 0{

        //     ui.add_space(9.);
        //     ui.vertical_centered(|ui|{


        //         ui.add(egui::Label::new(RichText::new(&proxy_self.current_language.general_ui_haslo_wylaczone.to_string()).color(fiolet_ciemny)).selectable(false));

        //     });

        //     ui.add_space(9.);

        // }else{

        //     ui.add(egui::Label::new(&proxy_self.current_language.general_ui_haslo_tytul.to_string()).selectable(false));
        //     let password_input = TextEdit::singleline(&mut proxy_self.debug_idx_password)
        //         .password(true)
        //         .hint_text(&proxy_self.current_language.general_ui_haslo.to_string());
        //     ui.add(password_input);

        // };
        // let adv_idx_deszyfr = false;
        // ui.vertical_centered(|ui|{
        // if adv_idx_deszyfr{
        //     if ui.button("Deszyfruj IDX").clicked() {

        //         if proxy_self.debug_output_idx_path.is_empty() ||proxy_self.debug_output_idx_path.is_empty() {

        //             eprintln!("[Ui/deszyfruj/idx_button :: LocalTime:{}]\n---> Brak wymaganych danych do deszyfrowania!\n", Local::now().format("%H:%M:%S"));

        //         } else {

        //             let debug_idx_file = Path::new(&proxy_self.debug_imput_idx_file);
        //             let debug_output_folder = Path::new(&proxy_self.debug_output_idx_path);

        //             match decrypt_copy::decrypt_idx_to_text_file(&debug_idx_file, &debug_output_folder,  &proxy_self.debug_idx_password, proxy_self.toggle_encryption) {
        //                 Ok(_) => println!("Plik odszyfrowany pomyślnie!"),
        //                 Err(e) => eprintln!("Błąd deszyfrowania: {}", e),
        //             }

        //         }

        //     } // zamknięcie button
        // } else {
        //     ui.add(egui::Label::new(RichText::new("Deszyfruj IDX").strikethrough()).selectable(false));
        // }
        // });
}