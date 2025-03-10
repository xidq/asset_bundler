use std::path::PathBuf;
// use eframe::App;
use egui::{Color32, Pos2, Response, RichText, Rect, Vec2, Context, ComboBox, Sense};
use egui::TextEdit;
use image::open;
// use std::sync::mpsc;
use rfd::FileDialog;
use std::sync::{
    Arc, 
    Mutex
};
use crate::encrypt_asset_setting::get_template_extensions;
// use std::io;
#[cfg(not(feature = "raw"))]
use crate::ui::ui_play_sound::play_finish_sound;
use crate::{
    ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka,
    

},
utils::loading::animacja};


pub fn ui_left_panel_foty_laczenie(
    proxy_self: &mut Appencja,
    ctx: &Context,ui: &mut egui::Ui,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32
){

    let reset_image =         egui::Image::new(egui::include_image!("../br/1245738_reset-icon-png.png"))
                                         .sense(egui::Sense::click())
                                         .max_height(14.0)
                                         .max_width(14.0)
                                         .maintain_aspect_ratio(true)
                                         .fit_to_original_size(1.)
                                         .tint(Color32::RED)
                                     // .rounding(10.0));

    ;
    // let restert_foto = egui::include_image!("../br/1245738_reset-icon-png.png");
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;
    let aspect_ratio_check =
    (proxy_self.ui_laczenie_specyfic_stosunek_czerwony == proxy_self.ui_laczenie_specyfic_stosunek_zielony) ||
    (proxy_self.ui_laczenie_specyfic_stosunek_czerwony == proxy_self.ui_laczenie_specyfic_stosunek_niebieski) ||
    (proxy_self.ui_laczenie_specyfic_stosunek_zielony == proxy_self.ui_laczenie_specyfic_stosunek_niebieski);




    let (linki_czek,link_czerwony, link_zielony, link_niebieski) = match (
        !proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.as_os_str().is_empty(),
        !proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.as_os_str().is_empty(),
        !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.as_os_str().is_empty()
    ) {
        (true,true,true) => (true,&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.clone(),&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone(),&proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.clone()),
        (true,true,false) => (true,&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.clone(),&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone(), { &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_czerwony,proxy_self.ui_laczenie_specyfic_wysokość_czerwony)) }),
        (true,false,true) => (true,&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.clone(), { &PathBuf::from(&format!("xyz/{}/{}",&proxy_self.ui_laczenie_specyfic_szerokość_czerwony,proxy_self.ui_laczenie_specyfic_wysokość_czerwony)) },&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone()),
        (true,false,false) => (true,&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.clone(),{ &PathBuf::from(&format!("xyz/{}/{}",&proxy_self.ui_laczenie_specyfic_szerokość_czerwony,proxy_self.ui_laczenie_specyfic_wysokość_czerwony)) }, { &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_czerwony,proxy_self.ui_laczenie_specyfic_wysokość_czerwony)) }),
        (false,true,true) => (true,{ &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_zielony,&proxy_self.ui_laczenie_specyfic_wysokość_zielony)) },&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone(), &proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.clone()),
        (false,true,false) => (true,{ &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_zielony,&proxy_self.ui_laczenie_specyfic_wysokość_zielony)) },&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone(), { &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_zielony,proxy_self.ui_laczenie_specyfic_wysokość_zielony)) }),
        (false,false,true) => (true,{ &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_niebieski,&proxy_self.ui_laczenie_specyfic_wysokość_niebieski)) }, { &PathBuf::from(&format!("xyz/{}/{}",proxy_self.ui_laczenie_specyfic_szerokość_niebieski,proxy_self.ui_laczenie_specyfic_wysokość_niebieski)) },&proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.clone()),
        _ => (false,&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.clone(),&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.clone(),&proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.clone())
    };












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
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
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
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0;
            if let Some(path) = FileDialog::new().add_filter("Images", &get_template_extensions("images_converter")).pick_file() {

                proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony = path.clone();
            }


                            // Załaduj zdjęcie i oblicz aspect ratio
            if let Ok(image) = open(&proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony) {
                proxy_self.ui_laczenie_specyfic_szerokość_czerwony = image.width();
                proxy_self.ui_laczenie_specyfic_wysokość_czerwony = image.height();

                // Oblicz aspect ratio
                proxy_self.ui_laczenie_specyfic_stosunek_czerwony = proxy_self.ui_laczenie_specyfic_szerokość_czerwony as f32 / proxy_self.ui_laczenie_specyfic_wysokość_czerwony as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.ui_laczenie_specyfic_stosunek_czerwony))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
            .color(Color32::RED))
                .selectable(false)
            );
        }

        ui.add_space(proxy_self.formatowanie_spacja_srednia);

        // if ui.add(egui::Label::new(RichText::new("RESET").color(Color32::RED).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false).sense(Sense::click())).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony = Default::default(); }
        if ui.add(reset_image.clone()).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony = Default::default(); }

        
    });
    if !proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy().is_empty(){
        if proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy().len() >= 50 {
            let xxxx = &proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy();
            let startu = &xxxx[..=15];
            let endu = &xxxx[xxxx.len()-30 ..];
            ui.add(egui::Label::new(RichText::new(
                format!("{}/.../{}",startu,endu))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);


        }else if !proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy().is_empty(){
        //napis sciezki
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
        }

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }


    //przycisk2

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general.to_owned()+" g")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
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
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0;
            if let Some(path) = FileDialog::new().add_filter("Images", &get_template_extensions("images_converter")).pick_file() {

                proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony = path.clone();

            }
            if let Ok(image) = open(&proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony) {
                proxy_self.ui_laczenie_specyfic_szerokość_zielony = image.width();
                proxy_self.ui_laczenie_specyfic_wysokość_zielony = image.height();

                // Oblicz aspect ratio
                proxy_self.ui_laczenie_specyfic_stosunek_zielony = proxy_self.ui_laczenie_specyfic_szerokość_zielony as f32 / proxy_self.ui_laczenie_specyfic_wysokość_zielony as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.ui_laczenie_specyfic_stosunek_zielony))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                .color(Color32::RED))
                .selectable(false)
            );
        }

        ui.add_space(proxy_self.formatowanie_spacja_srednia);


        // if ui.add(egui::Label::new(RichText::new("RESET").color(Color32::RED).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false).sense(Sense::click())).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony = Default::default(); }
        if ui.add(reset_image.clone()).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony = Default::default(); }

    });
    if !proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy().is_empty(){
        //napis sciezki
        if proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy().len() >= 50 {
            let xxxx = &proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy();
            let startu = &xxxx[..=15];
            let endu = &xxxx[xxxx.len()-30 ..];
            ui.add(egui::Label::new(RichText::new(
                format!("{}/.../{}",startu,endu))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);


        }else if !proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy().is_empty(){
            ui.add(egui::Label::new(
            RichText::new(
                proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
        }

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }




    //przycisk3

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_plik_general.to_owned()+" b")
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
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
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0;
            if let Some(path) = FileDialog::new().add_filter("Images", &get_template_extensions("images_converter")).pick_file() {

                proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski = path.clone();

            }
            if let Ok(image) = open(&proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski) {
                proxy_self.ui_laczenie_specyfic_szerokość_niebieski = image.width();
                proxy_self.ui_laczenie_specyfic_wysokość_niebieski = image.height();

                // Oblicz aspect ratio
                proxy_self.ui_laczenie_specyfic_stosunek_niebieski = proxy_self.ui_laczenie_specyfic_szerokość_niebieski as f32 / proxy_self.ui_laczenie_specyfic_wysokość_niebieski as f32;

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

        if !aspect_ratio_check{
            ui.add(egui::Label::new(RichText::new(
                format!("{}{}",proxy_self.current_language.err_aspect_ratio,proxy_self.ui_laczenie_specyfic_stosunek_niebieski))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                .color(Color32::RED))
                .selectable(false)
            );
        }

        ui.add_space(proxy_self.formatowanie_spacja_srednia);

        // if ui.add(egui::Label::new(RichText::new("RESET").color(Color32::RED).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false).sense(Sense::click())).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski = Default::default(); }

        if ui.add(reset_image.clone()).clicked() {proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony = Default::default(); }

    });
    if !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().is_empty(){
        //napis sciezki
        if proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().len() >= 50 {
            let xxxx = &proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy();
            let startu = &xxxx[..=15];
            let endu = &xxxx[xxxx.len()-30 ..];
            ui.add(egui::Label::new(RichText::new(
                format!("{}/.../{}",startu,endu))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);


        }else if !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().is_empty(){
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
        }

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }




    //przycisk4

    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy)
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
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
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)
            )
        );

        if btn_foty_folder_wejsciowy.clicked() {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0;
            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy = path.clone();

            }
        }

        let btn_pozycjonowanie_foty_folder_wejsciowy = Pos2::new(
            btn_foty_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_foty_folder_wejsciowy.rect.min.y+(btn_foty_folder_wejsciowy.rect.size().y / 2.));

        //dodawanie kółek
        if !proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy().is_empty(){

            ui.painter()
            .circle_filled(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_foty_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

        if proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy().len() < 28{
            //napis sciezki
            ui.add(egui::Label::new(
                RichText::new(proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

        }
    });
    if proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy().len() >=28{
        //napis sciezki
        if proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy().len() >= 50 {
            let xxxx = &proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy();
            let startu = &xxxx[..=15];
            let endu = &xxxx[xxxx.len()-30 ..];
            ui.add(egui::Label::new(RichText::new(
                format!("{}/.../{}",startu,endu))
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);


        }else if !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().is_empty(){
        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy())
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
        }

    }else{
        ui.add_space(proxy_self.formatowanie_spacja_duza);
    }


    // ui.separator();
    // ui.add_space(proxy_self.formatowanie_spacja_srednia);










    // ui.vertical_centered_justified(|ui|{

    //     ui.add(egui::Label::new(
    //         RichText::new(
    //             "Głębia bitów")
    //             .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
    //             .selectable(false)
    //         );
    // });
    // ui.add_space(proxy_self.formatowanie_spacja_mala);

    // ui.columns(2,|column|{
    //     column[0].vertical_centered_justified(|ui|{
    //         if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_bity, 8, RichText::new("8bit").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)).color(Color32::WHITE)).clicked(){
    //             proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
    //         };
    //     });
    //     column[1].vertical_centered_justified(|ui|{
    //         if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_bity, 16, RichText::new("16bit").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)).color(Color32::WHITE)).clicked(){
    //             proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
    //         };
    //     });
    // });


    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_rozszerzenie_tytul)
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.columns(3, |column|{
        column[0].vertical_centered_justified(|ui|{
            if ComboBox::from_id_salt("kakbjkbtskj"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
            .width(80.)
            .selected_text(RichText::new(
                match proxy_self.ui_laczenie_specyfic_dane_rozszerzenia{
                    0 => "jpg",
                    1=> "png",
                    2=>"png 16bit",
                    3=>"webp",
                    4=>"tga",
                    _ => proxy_self.current_language.err_value_overflow
                }
                ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .show_ui(ui, |ui| {

                if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_rozszerzenia, 0, RichText::new("jpg").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                    .clicked(){proxy_self.ui_laczenie_specyfic_dane_jakosc=Appencja::default().ui_laczenie_specyfic_dane_jakosc};
                if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_rozszerzenia, 1, RichText::new("png").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                    .clicked(){proxy_self.ui_laczenie_specyfic_dane_jakosc=1};
                if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_rozszerzenia, 2, RichText::new("png16").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .clicked(){proxy_self.ui_laczenie_specyfic_dane_jakosc=1};
                if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_rozszerzenia, 3, RichText::new("webp").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .clicked(){proxy_self.ui_laczenie_specyfic_dane_jakosc=Appencja::default().ui_laczenie_specyfic_dane_jakosc};
                if ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_rozszerzenia, 4, RichText::new("tga").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .clicked(){proxy_self.ui_laczenie_specyfic_dane_jakosc=Appencja::default().ui_laczenie_specyfic_dane_jakosc};
                }).response.clicked(){
                    proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
                }

        });
        column[1].vertical_centered_justified(|ui|{
            match proxy_self.ui_laczenie_specyfic_dane_rozszerzenia{
                0=>{
                        // &mut proxy_self.ui_laczenie_specyfic_dane_jakosc=75;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                            if ui.add(egui::Slider::new(&mut proxy_self.ui_laczenie_specyfic_dane_jakosc, 0..=100).show_value(true))
                            .double_clicked(){
                                proxy_self.ui_laczenie_specyfic_dane_jakosc=Appencja::default().ui_laczenie_specyfic_dane_jakosc
                            }
                        });
                    },
                1|2=>{
                        // proxy_self.ui_laczenie_specyfic_dane_jakosc=1;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                            if ui.add(egui::Slider::new(&mut proxy_self.ui_laczenie_specyfic_dane_jakosc, 0..=2).show_value(true))
                            .double_clicked(){
                                proxy_self.ui_laczenie_specyfic_dane_jakosc=1
                            }
                        });
                    },
                3=>{
                        // proxy_self.ui_laczenie_specyfic_dane_jakosc=75;
                        ui.vertical(|ui|{
                            ui.spacing_mut().slider_width = 50.0;
                            if ui.add(egui::Slider::new(&mut proxy_self.ui_laczenie_specyfic_dane_jakosc, 0..=100).show_value(true))
                            .double_clicked(){
                                proxy_self.ui_laczenie_specyfic_dane_jakosc=Appencja::default().ui_laczenie_specyfic_dane_jakosc
                            }
                        });
                    },
                _=> {ui.label("");}
            }
        });
        column[2].vertical_centered_justified(|ui|{
            match proxy_self.ui_laczenie_specyfic_dane_rozszerzenia{
                1|2=>{
                    if ComboBox::from_id_salt("kakbgrfgdskj"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                    .width(80.)
                    .selected_text(RichText::new(
                        match proxy_self.ui_laczenie_specyfic_dane_filter_png{
                            0 => proxy_self.current_language.png_specyfic_filter_png_none,
                            1=> proxy_self.current_language.png_specyfic_filter_png_sub,
                            2=>proxy_self.current_language.png_specyfic_filter_png_up,
                            3=>proxy_self.current_language.png_specyfic_filter_png_avg,
                            4=>proxy_self.current_language.png_specyfic_filter_png_paeth,
                            5=>proxy_self.current_language.png_specyfic_filter_png_adaptive,
                            _ => proxy_self.current_language.err_value_overflow
                        }
                        ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                        .show_ui(ui, |ui| {
        
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 0, RichText::new(proxy_self.current_language.png_specyfic_filter_png_none).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 1, RichText::new(proxy_self.current_language.png_specyfic_filter_png_sub).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 2, RichText::new(proxy_self.current_language.png_specyfic_filter_png_up).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 3, RichText::new(proxy_self.current_language.png_specyfic_filter_png_avg).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 4, RichText::new(proxy_self.current_language.png_specyfic_filter_png_paeth).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter_png, 5, RichText::new(proxy_self.current_language.png_specyfic_filter_png_adaptive).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        }).response.clicked(){
                            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
                        }},

                _=> {ui.label("");}
            }
        });
    });


    match proxy_self.ui_laczenie_specyfic_dane_rozszerzenia{
        0|1|3|4 => proxy_self.ui_laczenie_specyfic_dane_bity = 8,
        2 => proxy_self.ui_laczenie_specyfic_dane_bity = 16,
        _ => proxy_self.ui_laczenie_specyfic_dane_bity = 69
    };






    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_rozdzielczosc_tytul)
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.vertical_centered_justified(|ui|{
        if ComboBox::from_id_salt("gafdgsd"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
        .width(320.)
        .selected_text(RichText::new(
            match proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc{
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
            ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 0, RichText::new("16384px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 1, RichText::new("8192px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 2, RichText::new("4096px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 3, RichText::new("2048px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 4, RichText::new("1024px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 5, RichText::new("512px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 6, RichText::new("256px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 7, RichText::new("128px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 8, RichText::new("64px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
                ui.selectable_value(
                    &mut proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc, 9, RichText::new("32px")
                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                );
        }).response.clicked(){
                proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
            }

    });





    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.separator();
    ui.add_space(proxy_self.formatowanie_spacja_srednia);



    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.general_ui_filter_png_tytul)
                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
    });
    ui.add_space(proxy_self.formatowanie_spacja_mala);
    ui.vertical_centered_justified(|ui|{
        if ComboBox::from_id_salt("afgsh"/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
        .width(320.)
        .selected_text(RichText::new(
            match proxy_self.ui_laczenie_specyfic_dane_filter{
                0 => proxy_self.current_language.image_specyfic_filter_nearest,
                1=> proxy_self.current_language.image_specyfic_filter_triangle,
                2=> proxy_self.current_language.image_specyfic_filter_catmullrom,
                3=>proxy_self.current_language.image_specyfic_filter_gaussian,
                4=>proxy_self.current_language.image_specyfic_filter_lanczos3,
                _ => proxy_self.current_language.err_value_overflow
            }
            ).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter, 0, RichText::new(proxy_self.current_language.image_specyfic_filter_nearest).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter, 1, RichText::new(proxy_self.current_language.image_specyfic_filter_triangle).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter, 2, RichText::new(proxy_self.current_language.image_specyfic_filter_catmullrom).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter, 3, RichText::new(proxy_self.current_language.image_specyfic_filter_gaussian).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                ui.selectable_value(&mut proxy_self.ui_laczenie_specyfic_dane_filter, 4, RichText::new(proxy_self.current_language.image_specyfic_filter_lanczos3).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                }).response.clicked(){
                proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0
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
        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
    });        

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    let output_name_input = TextEdit::singleline(&mut proxy_self.ui_laczenie_specyfic_nazwa_pliku)
        .char_limit(50)
        .min_size(egui::Vec2{x:200.,y:30.})
        .hint_text(proxy_self.current_language.general_ui_nazwa.to_string())
        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki));
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


    let sprawdzacz_przycisku_fotx=
        (!proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony.to_string_lossy().is_empty() ||
        !proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony.to_string_lossy().is_empty() ||
        !proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski.to_string_lossy().is_empty()) &&
        !proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.to_string_lossy().is_empty() &&
            linki_czek && aspect_ratio_check;

    let tekst_przycisku_kompresji = if sprawdzacz_przycisku_fotx{

        &proxy_self.current_language.szyfrowanie_przycisk_ok}
        else
        {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};


    let hjgfkjlh = sprawdzacz_przycisku_fotx && proxy_self.ui_laczenie_specyfic_status_przetwarzania !=1;
    let tx_clonea = proxy_self.tx.clone();
    let fotox_butt_col = match (sprawdzacz_przycisku_fotx,proxy_self.ui_laczenie_specyfic_status_przetwarzania){
        (true,0)=>Color32::DARK_GREEN,
        (true,1)=>zolty_ciemny,
        _=>szarawy_ciemny
    };
    match proxy_self.rx.try_recv() {
        Ok(Ok(ghdfjsasx)) => {
            let danene = ghdfjsasx.lock().unwrap();
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 2;
            proxy_self.ui_laczenie_specyfic_statystyki_czas_sekundy = danene[0] as u64;
            proxy_self.ui_laczenie_specyfic_statystyki_czas_milisekundy = danene[1] as u32;
            #[cfg(not(feature = "raw"))]
            play_finish_sound(proxy_self.ui_ustawienia_glosnosc);
        }
        Ok(Err(e)) => {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 3;
            proxy_self.ui_laczenie_specyfic_error_3 = e.to_string();
            // eprintln!("Błąd: {}", e);
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            ctx.request_repaint();
            (proxy_self.general_ui_loading,proxy_self.general_ui_loading_tekst)=animacja(proxy_self.general_ui_loading);

        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            proxy_self.ui_laczenie_specyfic_status_przetwarzania = 4;
            proxy_self.ui_laczenie_specyfic_error_3 = "Disconected".to_string();
        }
    }
    ui.add_space( proxy_self.formatowanie_spacja_srednia);
    let t_p_d_fotox= match proxy_self.ui_laczenie_specyfic_status_przetwarzania{
        0 => RichText::new(tekst_przycisku_kompresji.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        1 => RichText::new(proxy_self.general_ui_loading_tekst.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,1)).color(Color32::BLACK),
        2 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        3 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        4 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        _ => RichText::new("".to_string())
    };



    if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.general_ui_szerokosc_okna / 4.),y:proxy_self.formatowanie_wysokosc_przycisku},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d_fotox
        )
        .min_size(egui::vec2((proxy_self.general_ui_szerokosc_okna / 2.) - 100.0, 40.0))
        .corner_radius(10.)
        .fill(fotox_butt_col))
        .clicked() && hjgfkjlh{
        #[cfg(feature = "statystyki")]
            println!("kliklem przycisk!");



            // if hjgfkjlh{
                proxy_self.ui_laczenie_specyfic_status_przetwarzania = 0;
                proxy_self.general_ui_loading = 0;
                let arc_z_path = Arc::new(Mutex::new(vec![
                    link_czerwony.clone(),
                    link_zielony.clone(),
                    link_niebieski.clone(),
                    proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy.clone()
                    ]));
        #[cfg(feature = "statystyki")]
                println!("{:?}",arc_z_path);
                let arc_z_string = Arc::new(Mutex::new(vec![
                    proxy_self.ui_laczenie_specyfic_nazwa_pliku.clone()
                    ]));
        #[cfg(feature = "statystyki")]
                    println!("{:?}",arc_z_string);
                let arc_z_u8 = Arc::new(Mutex::new(vec![
                    proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc,
                    proxy_self.ui_laczenie_specyfic_dane_rozszerzenia,
                    proxy_self.ui_laczenie_specyfic_dane_bity,
                    proxy_self.ui_laczenie_specyfic_dane_filter,
                    proxy_self.ui_laczenie_specyfic_dane_jakosc,
                    proxy_self.ui_laczenie_specyfic_dane_filter_png
                    ]));
        #[cfg(feature = "statystyki")]
                    println!("{:?}",arc_z_u8);




                std::thread::spawn(move || {
                    let result = crate::image_channel_bundler::image_channel_bundler_main::image_channel_bundler(
                        arc_z_path,
                        arc_z_string,
                        arc_z_u8,
                    );
                    #[cfg(feature = "statystyki")]
                    println!("ui w thread!");
                    
                    match tx_clonea.send(result) {
                        Ok(_) =>{
                            #[cfg(feature = "statystyki")]
                             println!("Wysłano wynik")
                        },

                        Err(_e) => {
                            #[cfg(feature = "statystyki")]
                            eprintln!("Błąd wysyłania: {}", _e) },

                    }
                });
        #[cfg(feature = "statystyki")]
                println!("ui po thread!");
                
                proxy_self.ui_laczenie_specyfic_status_przetwarzania = 1;

            }
        // }



    #[cfg(feature = "statystyki")]
match (proxy_self.general_ui_przelacz_tryb_debug, proxy_self.general_ui_licznik_czasu_debug){
    (true, 61..=u8::MAX) => {proxy_self.general_ui_licznik_czasu_debug = 0},
    (true, 60) => {
        println!("arc_z_path: R {:?},G {:?},B {:?},Out {:?}",
        proxy_self.ui_laczenie_specyfic_sciezka_plik_czerwony,
        proxy_self.ui_laczenie_specyfic_sciezka_plik_zielony,
        proxy_self.ui_laczenie_specyfic_sciezka_plik_niebieski,
        proxy_self.ui_laczenie_specyfic_sciezka_folder_wyjsciowy
    );
    println!("arc_z_string: {}",
        proxy_self.ui_laczenie_specyfic_nazwa_pliku
    );
    println!("arc_z_u8: res {}, ext {}, depth{}, filter{}, quality{}, png {}",
        proxy_self.ui_laczenie_specyfic_dane_rozdzielczosc,
        proxy_self.ui_laczenie_specyfic_dane_rozszerzenia,
        proxy_self.ui_laczenie_specyfic_dane_bity,
        proxy_self.ui_laczenie_specyfic_dane_filter,
        proxy_self.ui_laczenie_specyfic_dane_jakosc,
        proxy_self.ui_laczenie_specyfic_dane_filter_png
    );
    println!("-------------------------------------------------------------------");

        proxy_self.general_ui_licznik_czasu_debug += 1 ;
    },
    (true, 0..60) => {proxy_self.general_ui_licznik_czasu_debug += 1; }
    _ => {}
}


}