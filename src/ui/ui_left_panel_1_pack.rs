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
// use egui::WidgetText::RichText;
#[cfg(not(feature = "raw"))]
use crate::ui::ui_play_sound::play_finish_sound;

use crate::{ui::{
    ui_change_font::wybrana_aktualna_czcionka,
    ui_defaults::Appencja,
}, utils::loading::animacja, encrypt_bez_async_i_bez_chacha20::encrypt_folder, dodaj_średni_richtext, dodaj_średni_label, dodaj_duży_label, dodaj_duży_richtext};
// use crate::ui::items_formatting::dodaj_średni_label;

pub fn ui_left_panel_encrypt(
    proxy_self: &mut Appencja,
    ctx: &Context,
    ui: &mut egui::Ui, 
    _fiolet_ciemny:Color32,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32){
        
    let margines_na_wybór_formatu_foty = proxy_self.formatowanie_spacja_średnia;



    // ██ ███    ██ ██████  ██    ██ ████████     ███████  ██████  ██      ██████  ███████ ██████  
    // ██ ████   ██ ██   ██ ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██ ██ ██  ██ ██████  ██    ██    ██        █████   ██    ██ ██      ██   ██ █████   ██████  
    // ██ ██  ██ ██ ██      ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██ ██   ████ ██       ██████     ██        ██       ██████  ███████ ██████  ███████ ██   ██ 


    ui.horizontal(|ui|{

        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_wybierz_folder_wejściowy));

    });

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duża+margines_na_wybór_formatu_foty);
        

        //button text
        let btn_szyfrowanie_folder_wejściowy : Response = ui.button(dodaj_średni_richtext!(proxy_self.current_language.general_ui_wybierz_folder));


        //takin' patch into string, just why not?
        if btn_szyfrowanie_folder_wejściowy.clicked() {

            //reset button state
            proxy_self.ui_pack_specyfic_status_przetwarzania = 0;

            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.ui_pack_specyfic_folder_wejściowy = path.to_string_lossy().to_string();

            }
        }


        //taking offset for circles
        let btn_pozycjonowanie_folder_wejściowy = Pos2::new(
            btn_szyfrowanie_folder_wejściowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_szyfrowanie_folder_wejściowy.rect.min.y+(btn_szyfrowanie_folder_wejściowy.rect.size().y / 2.)
        );
        

        //matchin' check circle
        match proxy_self.ui_pack_specyfic_folder_wejściowy.is_empty(){
            false => {
                ui.painter().circle_filled(btn_pozycjonowanie_folder_wejściowy,
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
                );},
            true => {
                ui.painter().circle_stroke(btn_pozycjonowanie_folder_wejściowy,
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                    proxy_self.formatowanie_kolor_kolko_puste)
                );}
        };

    }); //end of input folder path


    //adding place to see path or smth
    if proxy_self.ui_pack_specyfic_folder_wejściowy.len() >= 50 {

        let xxxx = &proxy_self.ui_pack_specyfic_folder_wejściowy;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else if !proxy_self.ui_pack_specyfic_folder_wejściowy.is_empty(){

        ui.add(dodaj_średni_label!(&proxy_self.ui_pack_specyfic_folder_wejściowy));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else{

        ui.add_space(proxy_self.formatowanie_spacja_duża);

    }


    //  ██████  ██    ██ ████████ ██████  ██    ██ ████████     ███████  ██████  ██      ██████  ███████ ██████  
    // ██    ██ ██    ██    ██    ██   ██ ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    // ██    ██ ██    ██    ██    ██████  ██    ██    ██        █████   ██    ██ ██      ██   ██ █████   ██████  
    // ██    ██ ██    ██    ██    ██      ██    ██    ██        ██      ██    ██ ██      ██   ██ ██      ██   ██ 
    //  ██████   ██████     ██    ██       ██████     ██        ██       ██████  ███████ ██████  ███████ ██   ██ 


    //title for output path
    ui.horizontal(|ui|{

        ui.add_space(margines_na_wybór_formatu_foty);
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_wybierz_folder_wyjściowy));

    });

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.horizontal(|ui|{

        ui.add_space(proxy_self.formatowanie_spacja_duża+margines_na_wybór_formatu_foty);

        let btn_szyfrowanie_folder_wyjściowy = ui.button(dodaj_średni_richtext!(proxy_self.current_language.general_ui_wybierz_folder));

        if btn_szyfrowanie_folder_wyjściowy.clicked() {

            //another reset for button state
            proxy_self.ui_pack_specyfic_status_przetwarzania = 0;

            if let Some(path) = FileDialog::new().pick_folder() {

                proxy_self.ui_pack_specyfic_folder_wyjściowy = path.to_string_lossy().to_string();

            }

        }

        let btn_pozycjonowanie_szyfrowanie_folder_wyjściowy = Pos2::new(
            btn_szyfrowanie_folder_wyjściowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
            btn_szyfrowanie_folder_wyjściowy.rect.min.y+(btn_szyfrowanie_folder_wyjściowy.rect.size().y / 2.));
        
        //CIRCLES!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        if !proxy_self.ui_pack_specyfic_folder_wyjściowy.is_empty(){

            ui.painter().circle_filled(
                btn_pozycjonowanie_szyfrowanie_folder_wyjściowy,
                proxy_self.formatowanie_rozmiar_kolko, 
                proxy_self.formatowanie_kolor_kolko_pelne
            );

        } else {

            ui.painter().circle_stroke(
                btn_pozycjonowanie_szyfrowanie_folder_wyjściowy,
                proxy_self.formatowanie_rozmiar_kolko_puste, 
                (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,
                    proxy_self.formatowanie_kolor_kolko_puste)
            );

        }

    }); //end of output folder path
    
    // and another display of path
    if proxy_self.ui_pack_specyfic_folder_wyjściowy.len() >= 50 {

        let xxxx = &proxy_self.ui_pack_specyfic_folder_wyjściowy;
        let startu = &xxxx[..=15];
        let endu = &xxxx[xxxx.len()-30 ..];
        ui.add(dodaj_średni_label!(format!("{}/.../{}",startu,endu)));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);


    }else if !proxy_self.ui_pack_specyfic_folder_wyjściowy.is_empty(){

        ui.add(dodaj_średni_label!(&proxy_self.ui_pack_specyfic_folder_wyjściowy));
        ui.add_space(proxy_self.formatowanie_spacja_duża-proxy_self.formatowanie_rozmiar_czcionki_średnia-4.);

    }else{

        ui.add_space(proxy_self.formatowanie_spacja_duża);

    }

    ui.separator();


    // ███████ ██ ██      ███████     ███    ██  █████  ███    ███ ███████ 
    // ██      ██ ██      ██          ████   ██ ██   ██ ████  ████ ██      
    // █████   ██ ██      █████       ██ ██  ██ ███████ ██ ████ ██ █████   
    // ██      ██ ██      ██          ██  ██ ██ ██   ██ ██  ██  ██ ██      
    // ██      ██ ███████ ███████     ██   ████ ██   ██ ██      ██ ███████


    ui.add_space(proxy_self.formatowanie_spacja_średnia);
    ui.vertical_centered_justified(|ui|{

        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_nazwa_tytul));
            
    });        

    ui.add_space( proxy_self.formatowanie_spacja_mala);

    let ui_pack_specyfic_nazwa_pliku_input = TextEdit::singleline(
        &mut proxy_self.ui_pack_specyfic_nazwa_pliku)
        .char_limit(50)
        .min_size(egui::Vec2{x:200.,y:30.})
        .hint_text(proxy_self.current_language.general_ui_nazwa.to_string())
        .font(wybrana_aktualna_czcionka(
            proxy_self.formatowanie_rozmiar_czcionki_duża,
            proxy_self.formatowanie_wybór_czcionki)
        );
    ui.vertical_centered_justified(|ui|{
        ui.add(ui_pack_specyfic_nazwa_pliku_input);
    });

    ui.add_space( proxy_self.formatowanie_spacja_średnia);

    ui.separator();

    ui.add_space(proxy_self.formatowanie_spacja_średnia);


    //  ██████  ██████  ███    ███ ██████  ██████  ███████ ███████ ███████ ██  ██████  ███    ██ 
    // ██      ██    ██ ████  ████ ██   ██ ██   ██ ██      ██      ██      ██ ██    ██ ████   ██ 
    // ██      ██    ██ ██ ████ ██ ██████  ██████  █████   ███████ ███████ ██ ██    ██ ██ ██  ██ 
    // ██      ██    ██ ██  ██  ██ ██      ██   ██ ██           ██      ██ ██ ██    ██ ██  ██ ██ 
    //  ██████  ██████  ██      ██ ██      ██   ██ ███████ ███████ ███████ ██  ██████  ██   ████


    ui.vertical_centered_justified(|ui|{
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_kompresja_tytul));
    });
    ui.add_space( proxy_self.formatowanie_spacja_mala);

    ui.columns(2,|column|{

        column[0].vertical_centered_justified(|ui|{

            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybór_kompresji,
                0, 
                dodaj_średni_richtext!(proxy_self.current_language.general_ui_label_brak))
                    .clicked(){
                        proxy_self.ui_pack_specyfic_status_przetwarzania=0
                    };

        });

        column[1].vertical_centered_justified(|ui|{
            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybór_kompresji, 1,
                dodaj_średni_richtext!("Zstd")).clicked(){
                        proxy_self.ui_pack_specyfic_status_przetwarzania=0
                    };

        });

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);
    
    ui.horizontal(|ui|{

        ui.add_space(margines_na_wybór_formatu_foty);

        ui.add(dodaj_średni_label!(proxy_self.current_language.szyfrowanie_kompresja));

        let compression_empty_let:bool = proxy_self.ui_pack_specyfic_wybór_kompresji>=1;

        ui.add_enabled(compression_empty_let,|ui: &mut egui::Ui|{
            ui.add(egui::Slider::new(
                &mut proxy_self.ui_pack_specyfic_poziom_kompresji, 1..=22).text(""))

        });
    
    });

    ui.add_space( proxy_self.formatowanie_spacja_średnia);

    ui.separator();
    
    ui.add_space( proxy_self.formatowanie_spacja_średnia);


    // ███████ ███    ██  ██████ ██████  ██    ██ ██████  ████████ ██  ██████  ███    ██ 
    // ██      ████   ██ ██      ██   ██  ██  ██  ██   ██    ██    ██ ██    ██ ████   ██ 
    // █████   ██ ██  ██ ██      ██████    ████   ██████     ██    ██ ██    ██ ██ ██  ██ 
    // ██      ██  ██ ██ ██      ██   ██    ██    ██         ██    ██ ██    ██ ██  ██ ██ 
    // ███████ ██   ████  ██████ ██   ██    ██    ██         ██    ██  ██████  ██   ████ 


    ui.vertical_centered_justified(|ui|{

        ui.add(dodaj_duży_label!(proxy_self.current_language.szyfrowanie_szyfrowanie_tytul));

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    ui.columns(2,|column|{

        column[0].vertical_centered_justified(|ui|{

            if ui.selectable_value(
                &mut proxy_self.ui_pack_specyfic_wybór_szyfrowania,
                0,  
                dodaj_średni_richtext!(
                proxy_self.current_language.general_ui_label_brak)
            ).clicked(){
                proxy_self.ui_pack_specyfic_status_przetwarzania=0
            }

        });

        column[1].vertical_centered_justified(|ui|{

            if ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybór_szyfrowania, 1,
                dodaj_średni_richtext!("chacha, do not use!")
                    .strikethrough()
            ).clicked(){
                proxy_self.ui_pack_specyfic_status_przetwarzania=0
            }

        });

    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    ui.columns(1, |column|{

        if proxy_self.ui_pack_specyfic_wybór_szyfrowania == 0{
            column[0].vertical_centered_justified(|ui|{

                let przerwa_tam_gdzie_haslo =
                    (30. + 
                        proxy_self.formatowanie_rozmiar_czcionki_średnia +
                        proxy_self.formatowanie_spacja_mala - 
                        proxy_self.formatowanie_rozmiar_czcionki_duża -
                        1.) / 2.
                ;
                ui.add_space( przerwa_tam_gdzie_haslo);
                ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_haslo_wylaczone));
                ui.add_space( przerwa_tam_gdzie_haslo);

            });

        }else{

            column[0].vertical_centered_justified(|ui|{
                ui.add(dodaj_średni_label!(proxy_self.current_language.general_ui_haslo_tytul.to_string()));
                ui.add_space( proxy_self.formatowanie_spacja_mala);
                let password_input = TextEdit::singleline(&mut proxy_self.ui_pack_specyfic_password)
                .password(true)
                .hint_text(dodaj_średni_richtext!(proxy_self.current_language.general_ui_haslo.to_string()))
                               .min_size(Vec2{x:200.,y:30.});
                ui.add(password_input);

            });

        }

    });

    ui.add_space(proxy_self.formatowanie_spacja_średnia);

    ui.separator();
    
    ui.add_space(proxy_self.formatowanie_spacja_średnia);


    // ████████ ███████ ███    ███ ██████  ██       █████  ████████ ███████ 
    //    ██    ██      ████  ████ ██   ██ ██      ██   ██    ██    ██      
    //    ██    █████   ██ ████ ██ ██████  ██      ███████    ██    █████   
    //    ██    ██      ██  ██  ██ ██      ██      ██   ██    ██    ██      
    //    ██    ███████ ██      ██ ██      ███████ ██   ██    ██    ███████


    ui.vertical_centered_justified(|ui|{
        ui.add(dodaj_duży_label!(proxy_self.current_language.general_ui_szablony_tytul));
    });

    ui.add_space(proxy_self.formatowanie_spacja_mala);

    let dane_do_template = vec![
        ("none",proxy_self.current_language.szablony_wybór_0),
        ("assets",proxy_self.current_language.szablony_wybór_1),
        ("images",proxy_self.current_language.szablony_wybór_2),
        ("audio",proxy_self.current_language.szablony_wybór_3),
        ("3d_model",proxy_self.current_language.szablony_wybór_4),
        ("documents",proxy_self.current_language.szablony_wybór_5),
        ("raw_photos",proxy_self.current_language.szablony_wybór_6),
    ];



    ui.columns(2, |column|{
        column[0].vertical_centered_justified(|ui|{
            ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                .width(proxy_self.general_ui_szerokosc_okna / 5.)
                .selected_text(dodaj_średni_richtext!(

                    // names_match_loop(&dane_do_template,&proxy_self.ui_pack_specyfic_template)



                    match proxy_self.ui_pack_specyfic_template.as_str(){
                    "none" => proxy_self.current_language.szablony_wybór_0,
                    "assets" => { proxy_self.ui_pack_specyfic_statyczne_id = true;
                        proxy_self.current_language.szablony_wybór_1 },
                    "images" => proxy_self.current_language.szablony_wybór_2,
                    "audio" => proxy_self.current_language.szablony_wybór_3,
                    "3d_model" => proxy_self.current_language.szablony_wybór_4,
                    "documents" => proxy_self.current_language.szablony_wybór_5,
                    "raw_photos" => proxy_self.current_language.szablony_wybór_6,
                    _ => proxy_self.current_language.err_value_overflow
                    }

            )
            )
                .show_ui(ui, |ui| {

                    // Generating dropdown fields
                    for (szablon,tekst) in dane_do_template{
                        ui.selectable_value(
                            &mut proxy_self.ui_pack_specyfic_template,
                            szablon.to_string(), dodaj_średni_richtext!
                            (tekst));

                    }

            });
        });
        column[1].vertical_centered_justified(|ui|{
            ui.checkbox(&mut proxy_self.ui_pack_specyfic_statyczne_id,dodaj_średni_richtext!(proxy_self.current_language.łączenie_statyczny_id) );
        });
    });
    

    // ██████  ██    ██ ████████ ████████  ██████  ███    ██ 
    // ██   ██ ██    ██    ██       ██    ██    ██ ████   ██ 
    // ██████  ██    ██    ██       ██    ██    ██ ██ ██  ██ 
    // ██   ██ ██    ██    ██       ██    ██    ██ ██  ██ ██ 
    // ██████   ██████     ██       ██     ██████  ██   ████ 

    
    let sprawdzacz_plikow_kompresji= !proxy_self.ui_pack_specyfic_folder_wejściowy.is_empty() && !proxy_self.ui_pack_specyfic_folder_wyjściowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
            

    let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

        proxy_self.current_language.szyfrowanie_przycisk_ok}
        else
        {proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
    
    // ui.add_space( proxy_self.formatowanie_spacja_średnia);
    // let t_p_d= match proxy_self.ui_pack_specyfic_status_przetwarzania{
    //     0 => tekst_przycisku_kompresji,
    //     1 => &&general_ui_loading_tekst as &str,
    //     2 => &proxy_self.current_language.szyfrowanie_przycisk_koniec,
    //     3 => &proxy_self.current_language.szyfrowanie_przycisk_3,
    //     4 => &proxy_self.current_language.szyfrowanie_przycisk_4,
    //     _ => ""
    // };


    let blblblblblblbl = !proxy_self.ui_pack_specyfic_folder_wejściowy.is_empty() && proxy_self.ui_pack_specyfic_status_przetwarzania !=1 && !proxy_self.ui_pack_specyfic_folder_wyjściowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
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
    ui.add_space( proxy_self.formatowanie_spacja_średnia);
    let t_p_d= match proxy_self.ui_pack_specyfic_status_przetwarzania{
        0 => dodaj_duży_richtext!(tekst_przycisku_kompresji.to_string()),
        1 => RichText::new(proxy_self.general_ui_loading_tekst.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duża,1)).color(Color32::BLACK),
        2 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
        3 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
        4 => dodaj_duży_richtext!(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
        _ => dodaj_duży_richtext!("")
    };


    // ui.put(Rect::from_center_size(Pos2{x:1.,y:500.},Vec2{x:1.,y:2.}), egui::Label::new("gadfgs"));
    if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.general_ui_szerokosc_okna / 4.),y:proxy_self.formatowanie_wysokosc_przycisku},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d)
    // ui.vertical_centered(|ui|{
    //     if ui.add(egui::Button::new(t_p_d
        // .size( proxy_self.formatowanie_rozmiar_czcionki_duża))
        .min_size(egui::vec2((proxy_self.general_ui_szerokosc_okna / 2.) - 100.0, 40.0))
        .corner_radius(10.)
        .fill(szyfr_butt_col))
        .clicked() && blblblblblblbl{



            // if blblblblblblbl{
                proxy_self.ui_pack_specyfic_status_przetwarzania = 0;
                proxy_self.general_ui_loading = 0;
                let rozszerzenie_plikku = if proxy_self.ui_pack_specyfic_wybór_szyfrowania == 0 {"jrz"} else if proxy_self.ui_pack_specyfic_wybór_szyfrowania == 1 {"jrzs"} else{"bin"};
                let output_file = format!("{}/{}.{}" , proxy_self.ui_pack_specyfic_folder_wyjściowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);
                let index_file = format!("{}/{}{}.idx", proxy_self.ui_pack_specyfic_folder_wyjściowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);

                let arc_z_u8 = Arc::new(
                    Mutex::new(
                        vec![
                            proxy_self.ui_pack_specyfic_wybór_szyfrowania,
                            proxy_self.ui_pack_specyfic_wybór_kompresji,
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
                            proxy_self.ui_pack_specyfic_folder_wejściowy.clone().to_string(),
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
                            #[cfg(debug_assertions)]
                            println!("Wysłano wynik")
                        },

                        Err(_e) => {
                            #[cfg(debug_assertions)]
                            eprintln!("Błąd wysyłania: {}", _e)
                        },
                    }
                });

                proxy_self.ui_pack_specyfic_status_przetwarzania = 1;

            }
        // }

                        

        // });
}