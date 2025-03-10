use egui::{
    Color32,  
    Pos2, 
    Rect, 
    Response, 
    RichText, 
    Vec2,
    TextEdit,
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

use crate::{
        ui::{
        ui_change_font::wybrana_aktualna_czcionka,
        ui_defaults::Appencja,

    },
    utils::loading::animacja,
    // utils::ui_specyfic::names_match_loop,
    encrypt_bez_async_i_bez_chacha20::encrypt_folder
};



pub fn ui_left_panel_encrypt(
    proxy_self: &mut Appencja,
    ctx: &Context,
    ui: &mut egui::Ui, 
    fiolet_ciemny:Color32,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32){
        
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;



    // ██ ███    ██ ██████  ██    ██ ████████     ███████  ██████  ██      ██████  ███████ ██████  
    // ██ ████   ██ ██   ██ ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██ ██ ██  ██ ██████  ██    ██    ██        █████   ██    ██ ██      ██   ██ █████   ██████  
    // ██ ██  ██ ██ ██      ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██ ██   ████ ██       ██████     ██        ██       ██████  ███████ ██████  ███████ ██   ██ 


    ui.horizontal(|ui|{

        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.general_ui_wybierz_folder_wejsciowy)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );

    });

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);
        

        //button text
        let btn_szyfrowanie_folder_wejsciowy : Response = ui.button(
            RichText::new(proxy_self.current_language.general_ui_wybierz_folder)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki))
        );


        //takin' patch into string, just why not?
        if btn_szyfrowanie_folder_wejsciowy.clicked() {

            //reset button state
            proxy_self.ui_pack_specyfic_status_przetwarzania = 0;

            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.ui_pack_specyfic_folder_wejsciowy = path.to_string_lossy().to_string();

            }
        }


        //taking offset for circles
        let btn_pozycjonowanie_folder_wejsciowy = Pos2::new(
            btn_szyfrowanie_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_szyfrowanie_folder_wejsciowy.rect.min.y+(btn_szyfrowanie_folder_wejsciowy.rect.size().y / 2.)
        );
        

        //matchin' check circle
        match proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty(){
            false => {
                ui.painter().circle_filled(btn_pozycjonowanie_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
                );},
            true => {
                ui.painter().circle_stroke(btn_pozycjonowanie_folder_wejsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                    proxy_self.formatowanie_kolor_kolko_puste)
                );}
        };

    }); //end of input folder path


    //adding place to see path or smth
    if proxy_self.ui_pack_specyfic_folder_wejsciowy.len() >= 50 {

        let xxxx = &proxy_self.ui_pack_specyfic_folder_wejsciowy;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(egui::Label::new(RichText::new(
            format!("{}/.../{}",startu,endu))
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
        );
        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else if !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty(){

        ui.add(egui::Label::new(RichText::new(
            &proxy_self.ui_pack_specyfic_folder_wejsciowy)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{

        ui.add_space(proxy_self.formatowanie_spacja_duza);

    }


    //  ██████  ██    ██ ████████ ██████  ██    ██ ████████     ███████  ██████  ██      ██████  ███████ ██████  
    // ██    ██ ██    ██    ██    ██   ██ ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██    ██ ██    ██    ██    ██████  ██    ██    ██        █████   ██    ██ ██      ██   ██ █████   ██████  
    // ██    ██ ██    ██    ██    ██      ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    //  ██████   ██████     ██    ██       ██████     ██        ██       ██████  ███████ ██████  ███████ ██   ██ 


    //title for output path
    ui.horizontal(|ui|{

        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
        );

    });

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

        let btn_szyfrowanie_folder_wyjsciowy = ui.button(RichText::new(
            proxy_self.current_language.general_ui_wybierz_folder)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)));

        if btn_szyfrowanie_folder_wyjsciowy.clicked() {

            //another reset for button state
            proxy_self.ui_pack_specyfic_status_przetwarzania = 0;

            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.ui_pack_specyfic_folder_wyjsciowy = path.to_string_lossy().to_string();

            }

        }

        let btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy = Pos2::new(
            btn_szyfrowanie_folder_wyjsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_szyfrowanie_folder_wyjsciowy.rect.min.y+(btn_szyfrowanie_folder_wyjsciowy.rect.size().y / 2.));
        
        //CIRCLES!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        if !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty(){

            ui.painter().circle_filled(
                btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, 
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, 
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                    proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

    }); //end of output folder path
    
    // and another display of path
    if proxy_self.ui_pack_specyfic_folder_wyjsciowy.len() >= 50 {

        let xxxx = &proxy_self.ui_pack_specyfic_folder_wyjsciowy;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(egui::Label::new(RichText::new(
            format!("{}/.../{}",startu,endu))
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);


    }else if !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty(){

        ui.add(egui::Label::new(RichText::new(
            &proxy_self.ui_pack_specyfic_folder_wyjsciowy)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
        ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

    }else{

        ui.add_space(proxy_self.formatowanie_spacja_duza);

    }

    ui.separator();


    // ███████ ██ ██      ███████     ███    ██  █████  ███    ███ ███████ 
    // ██      ██ ██      ██          ████   ██ ██   ██ ████  ████ ██      
    // █████   ██ ██      █████       ██ ██  ██ ███████ ██ ████ ██ █████   
    // ██      ██ ██      ██          ██  ██ ██ ██   ██ ██  ██  ██ ██      
    // ██      ██ ███████ ███████     ██   ████ ██   ██ ██      ██ ███████


    ui.add_space(proxy_self.formatowanie_spacja_srednia);
    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.general_ui_nazwa_tytul)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
            );
            
    });        

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    let ui_pack_specyfic_nazwa_pliku_input = TextEdit::singleline(
        &mut proxy_self.ui_pack_specyfic_nazwa_pliku)
        .char_limit(50)
        .min_size(egui::Vec2{x:200.,y:30.})
        .hint_text(proxy_self.current_language.general_ui_nazwa.to_string())
        .font(wybrana_aktualna_czcionka(
            proxy_self.formatowanie_rozmiar_czcionki_duza,
            proxy_self.formatowanie_wybor_czcionki)
        );
    ui.vertical_centered_justified(|ui|{
        ui.add(ui_pack_specyfic_nazwa_pliku_input);
    });

    ui.add_space( proxy_self.formatowanie_spacja_srednia);

    ui.separator();

    ui.add_space(proxy_self.formatowanie_spacja_srednia);


    //  ██████  ██████  ███    ███ ██████  ██████  ███████ ███████ ███████ ██  ██████  ███    ██ 
    // ██      ██    ██ ████  ████ ██   ██ ██   ██ ██      ██      ██      ██ ██    ██ ████   ██ 
    // ██      ██    ██ ██ ████ ██ ██████  ██████  █████   ███████ ███████ ██ ██    ██ ██ ██  ██ 
    // ██      ██    ██ ██  ██  ██ ██      ██   ██ ██           ██      ██ ██ ██    ██ ██  ██ ██ 
    //  ██████  ██████  ██      ██ ██      ██   ██ ███████ ███████ ███████ ██  ██████  ██   ████


    ui.vertical_centered_justified(|ui|{
        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.general_ui_kompresja_tytul.to_string())
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
        );
    });
    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.columns(2,|column|{

        column[0].vertical_centered_justified(|ui|{

            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybor_kompresji, 
                0, 
                RichText::new(
                    proxy_self.current_language.general_ui_label_brak)
                    .font(wybrana_aktualna_czcionka(
                        proxy_self.formatowanie_rozmiar_czcionki_srednia,
                        proxy_self.formatowanie_wybor_czcionki))
                    )
                    .clicked(){
                        proxy_self.ui_pack_specyfic_status_przetwarzania=0
                    };

        });

        column[1].vertical_centered_justified(|ui|{
            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybor_kompresji, 1, RichText::new(
                    "Zstd")
                    .font(wybrana_aktualna_czcionka(
                        proxy_self.formatowanie_rozmiar_czcionki_srednia,
                        proxy_self.formatowanie_wybor_czcionki))
                    ).clicked(){
                        proxy_self.ui_pack_specyfic_status_przetwarzania=0
                    };

        });

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);
    
    ui.horizontal(|ui|{
        ui.add_space(margines_na_wybor_formatu_foty);
        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.szyfrowanie_kompresja.to_string())
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_srednia,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false)
        );

        let compression_empty_let:bool = proxy_self.ui_pack_specyfic_wybor_kompresji>=1;

        ui.add_enabled(compression_empty_let,|ui: &mut egui::Ui|{
            ui.add(egui::Slider::new(
                &mut proxy_self.ui_pack_specyfic_poziom_kompresji, 1..=22).text(""))

        });
    
    });

    ui.add_space( proxy_self.formatowanie_spacja_srednia);

    ui.separator();
    
    ui.add_space( proxy_self.formatowanie_spacja_srednia);


    // ███████ ███    ██  ██████ ██████  ██    ██ ██████  ████████ ██  ██████  ███    ██ 
    // ██      ████   ██ ██      ██   ██  ██  ██  ██   ██    ██    ██ ██    ██ ████   ██ 
    // █████   ██ ██  ██ ██      ██████    ████   ██████     ██    ██ ██    ██ ██ ██  ██ 
    // ██      ██  ██ ██ ██      ██   ██    ██    ██         ██    ██ ██    ██ ██  ██ ██ 
    // ███████ ██   ████  ██████ ██   ██    ██    ██         ██    ██  ██████  ██   ████ 


    ui.vertical_centered_justified(|ui|{

        ui.add(egui::Label::new(
            RichText::new(
                proxy_self.current_language.szyfrowanie_szyfrowanie_tytul)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                    .selectable(false)
        );

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    ui.columns(2,|column|{

        column[0].vertical_centered_justified(|ui|{

            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybor_szyfrowania, 
                0,  
                RichText::new(
                proxy_self.current_language.general_ui_label_brak)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_srednia,
                    proxy_self.formatowanie_wybor_czcionki))
            ).clicked(){
                proxy_self.ui_pack_specyfic_status_przetwarzania=0
            }

        });

        column[1].vertical_centered_justified(|ui|{

            if ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybor_szyfrowania, 1,  RichText::new(
                "chacha, do not use!")
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                    .strikethrough()
            ).clicked(){
                proxy_self.ui_pack_specyfic_status_przetwarzania=0
            }

        });

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    ui.columns(1, |column|{

        if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 0{
            column[0].vertical_centered_justified(|ui|{

                let przerwa_tam_gdzie_haslo =
                    (30. + 
                        proxy_self.formatowanie_rozmiar_czcionki_srednia + 
                        proxy_self.formatowanie_spacja_mala - 
                        proxy_self.formatowanie_rozmiar_czcionki_duza - 
                        1.) / 2.
                ;
                ui.add_space( przerwa_tam_gdzie_haslo);
                ui.add(egui::Label::new(RichText::new(
                    proxy_self.current_language.general_ui_haslo_wylaczone)
                    .font(wybrana_aktualna_czcionka(
                        proxy_self.formatowanie_rozmiar_czcionki_duza,
                        proxy_self.formatowanie_wybor_czcionki))
                        .color(fiolet_ciemny))
                        .selectable(false)
                    );
                ui.add_space( przerwa_tam_gdzie_haslo);

            });

        }else{

            column[0].vertical_centered_justified(|ui|{
                ui.add(egui::Label::new(RichText::new(
                    proxy_self.current_language.general_ui_haslo_tytul.to_string())
                    .font(wybrana_aktualna_czcionka(
                        proxy_self.formatowanie_rozmiar_czcionki_srednia,
                        proxy_self.formatowanie_wybor_czcionki)))
                        .selectable(false)
                );
                ui.add_space( proxy_self.formatowanie_spacja_mala);
                let password_input = TextEdit::singleline(&mut proxy_self.ui_pack_specyfic_password)
                .password(true)
                .hint_text(RichText::new(proxy_self.current_language.general_ui_haslo.to_string())
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_srednia,
                    proxy_self.formatowanie_wybor_czcionki))
                ).min_size(Vec2{x:200.,y:30.});
                ui.add(password_input);

            });

        }

    });

    ui.add_space(proxy_self.formatowanie_spacja_srednia);

    ui.separator();
    
    ui.add_space(proxy_self.formatowanie_spacja_srednia);


    // ████████ ███████ ███    ███ ██████  ██       █████  ████████ ███████ 
    //    ██    ██      ████  ████ ██   ██ ██      ██   ██    ██    ██      
    //    ██    █████   ██ ████ ██ ██████  ██      ███████    ██    █████   
    //    ██    ██      ██  ██  ██ ██      ██      ██   ██    ██    ██      
    //    ██    ███████ ██      ██ ██      ███████ ██   ██    ██    ███████


    ui.vertical_centered_justified(|ui|{
        ui.add(egui::Label::new(RichText::new(
            proxy_self.current_language.general_ui_szablony_tytul)
            .font(wybrana_aktualna_czcionka(
                proxy_self.formatowanie_rozmiar_czcionki_duza,
                proxy_self.formatowanie_wybor_czcionki)))
                .selectable(false));
    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    let dane_do_template = vec![
        ("none",proxy_self.current_language.szablony_wybor_0),
        ("assets",proxy_self.current_language.szablony_wybor_1),
        ("images",proxy_self.current_language.szablony_wybor_2),
        ("audio",proxy_self.current_language.szablony_wybor_3),
        ("3d_model",proxy_self.current_language.szablony_wybor_4),
        ("documents",proxy_self.current_language.szablony_wybor_5),
        ("raw_photos",proxy_self.current_language.szablony_wybor_6),
    ];



    ui.columns(2, |column|{
        column[0].vertical_centered_justified(|ui|{
            ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                .width(proxy_self.general_ui_szerokosc_okna / 5.)
                .selected_text(RichText::new(

                    // names_match_loop(&dane_do_template,&proxy_self.ui_pack_specyfic_template)



                    match proxy_self.ui_pack_specyfic_template.as_str(){
                    "none" => proxy_self.current_language.szablony_wybor_0,
                    "assets" => { proxy_self.ui_pack_specyfic_statyczne_id = true;
                        proxy_self.current_language.szablony_wybor_1 },
                    "images" => proxy_self.current_language.szablony_wybor_2,
                    "audio" => proxy_self.current_language.szablony_wybor_3,
                    "3d_model" => proxy_self.current_language.szablony_wybor_4,
                    "documents" => proxy_self.current_language.szablony_wybor_5,
                    "raw_photos" => proxy_self.current_language.szablony_wybor_6,
                    _ => proxy_self.current_language.err_value_overflow
                    }

            )
            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                .show_ui(ui, |ui| {

                    // Generating dropdown fields
                    for (szablon,tekst) in dane_do_template{
                        ui.selectable_value(
                            &mut proxy_self.ui_pack_specyfic_template,
                            szablon.to_string(), RichText::new(
                                tekst)
                            .font(wybrana_aktualna_czcionka(
                                proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));

                    }

            });
        });
        column[1].vertical_centered_justified(|ui|{
            ui.checkbox(&mut proxy_self.ui_pack_specyfic_statyczne_id,"Enable Static Id" );
        });
    });
    

    // ██████  ██    ██ ████████ ████████  ██████  ███    ██ 
    // ██   ██ ██    ██    ██       ██    ██    ██ ████   ██ 
    // ██████  ██    ██    ██       ██    ██    ██ ██ ██  ██ 
    // ██   ██ ██    ██    ██       ██    ██    ██ ██  ██ ██ 
    // ██████   ██████     ██       ██     ██████  ██   ████ 

    
    let sprawdzacz_plikow_kompresji= !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty() && !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
            

    let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

        proxy_self.current_language.szyfrowanie_przycisk_ok}
        else
        {proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
    
    // ui.add_space( proxy_self.formatowanie_spacja_srednia);
    // let t_p_d= match proxy_self.ui_pack_specyfic_status_przetwarzania{
    //     0 => tekst_przycisku_kompresji,
    //     1 => &&general_ui_loading_tekst as &str,
    //     2 => &proxy_self.current_language.szyfrowanie_przycisk_koniec,
    //     3 => &proxy_self.current_language.szyfrowanie_przycisk_3,
    //     4 => &proxy_self.current_language.szyfrowanie_przycisk_4,
    //     _ => ""
    // };


    let blblblblblblbl = !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty() && proxy_self.ui_pack_specyfic_status_przetwarzania !=1 && !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
    let szyfr_butt_col = match (
        sprawdzacz_plikow_kompresji,
        proxy_self.ui_pack_specyfic_status_przetwarzania
    ){
        (true,0) => Color32::DARK_GREEN,
        (true,1) => zolty_ciemny,
        _ =>szarawy_ciemny
    };

    match proxy_self.rx.try_recv() {
        Ok(Ok(afasdaf)) => {
            let danene = afasdaf.lock().unwrap();
            proxy_self.ui_pack_specyfic_status_przetwarzania = 2;
            proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki = danene[0];
            proxy_self.ui_pack_specyfic_statystyki_czas_sekundy = danene[1] as u64;
            proxy_self.ui_pack_specyfic_statystyki_czas_milisekundy = danene[2] as u32;
            #[cfg(not(feature = "raw"))]
            play_finish_sound(proxy_self.ui_ustawienia_glosnosc);
        }
        Ok(Err(e)) => {
            proxy_self.ui_pack_specyfic_status_przetwarzania = 3;
            proxy_self.ui_pack_specyfic_error_3 = e.to_string();
            // eprintln!("Błąd: {}", e);
        }
        Err(std::sync::mpsc::TryRecvError::Empty) => {
            ctx.request_repaint();
            (proxy_self.general_ui_loading,proxy_self.general_ui_loading_tekst)=animacja(proxy_self.general_ui_loading);

        }
        Err(std::sync::mpsc::TryRecvError::Disconnected) => {
            proxy_self.ui_pack_specyfic_status_przetwarzania = 4;
            proxy_self.ui_pack_specyfic_error_3 = "Disconected".to_string();
        }
    }
    ui.add_space( proxy_self.formatowanie_spacja_srednia);
    let t_p_d= match proxy_self.ui_pack_specyfic_status_przetwarzania{
        0 => RichText::new(tekst_przycisku_kompresji.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        1 => RichText::new(proxy_self.general_ui_loading_tekst.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,1)).color(Color32::BLACK),
        2 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        3 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        4 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
        _ => RichText::new("".to_string())
    };


    // ui.put(Rect::from_center_size(Pos2{x:1.,y:500.},Vec2{x:1.,y:2.}), egui::Label::new("gadfgs"));
    if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.general_ui_szerokosc_okna / 4.),y:proxy_self.formatowanie_wysokosc_przycisku},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d)
    // ui.vertical_centered(|ui|{
    //     if ui.add(egui::Button::new(t_p_d
        // .size( proxy_self.formatowanie_rozmiar_czcionki_duza))
        .min_size(egui::vec2((proxy_self.general_ui_szerokosc_okna / 2.) - 100.0, 40.0))
        .corner_radius(10.)
        .fill(szyfr_butt_col))
        .clicked() && blblblblblblbl{



            // if blblblblblblbl{
                proxy_self.ui_pack_specyfic_status_przetwarzania = 0;
                proxy_self.general_ui_loading = 0;
                let rozszerzenie_plikku = if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 0 {"jrz"} else if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 1 {"jrzs"} else{"bin"};
                let output_file = format!("{}/{}.{}" , proxy_self.ui_pack_specyfic_folder_wyjsciowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);
                let index_file = format!("{}/{}{}.idx", proxy_self.ui_pack_specyfic_folder_wyjsciowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);

                let arc_z_u8 = Arc::new(
                    Mutex::new(
                        vec![
                            proxy_self.ui_pack_specyfic_wybor_szyfrowania,
                            proxy_self.ui_pack_specyfic_wybor_kompresji,
                            proxy_self.ui_pack_specyfic_poziom_kompresji
                        ]));
                let arc_z_bool = Arc::new(
                    Mutex::new(
                        vec![
                            proxy_self.ui_ustawienia_tworzenie_pliku_lua,
                            proxy_self.ui_pack_specyfic_statyczne_id
                        ]));
                let arc_z_str = Arc::new(
                    Mutex::new(
                        vec![
                            proxy_self.ui_pack_specyfic_folder_wejsciowy.clone().to_string(),
                            output_file.clone().to_string(),
                            index_file.clone().to_string(),
                            proxy_self.ui_pack_specyfic_template.clone().to_string(),
                            proxy_self.ui_pack_specyfic_password.clone().to_string()
                        ]));
                    
                    

                let tx_clone = proxy_self.tx.clone();
                std::thread::spawn(move || {
                    let result = encrypt_folder(
                        arc_z_str,
                        arc_z_u8,
                        arc_z_bool,
                    );
                    
                    match tx_clone.send(result) {

                        Ok(_) => {
                            #[cfg(feature = "statystyki")]
                            println!("Wysłano wynik")
                        },

                        Err(_e) => {
                            #[cfg(feature = "statystyki")]
                            eprintln!("Błąd wysyłania: {}", _e)
                        },
                    }
                });

                proxy_self.ui_pack_specyfic_status_przetwarzania = 1;

            }
        // }

                        

        // });
}