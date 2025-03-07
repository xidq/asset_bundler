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
use crate::encrypt_bez_async_i_bez_chacha20;
use crate::ui::{
    ui_change_font::wybrana_aktualna_czcionka,
    ui_defaults::Appencja,
    ui_play_sound::play_finish_sound
};


pub fn ui_left_panel_encrypt(proxy_self: &mut Appencja,ctx: &Context,ui: &mut egui::Ui, fiolet_ciemny:Color32,zolty_ciemny:Color32,szarawy_ciemny:Color32){
    let margines_na_wybor_formatu_foty = proxy_self.formatowanie_spacja_srednia;


                        //  _             _         
                        // | |_ ___ ___ _| |___ ___ 
                        // |   | -_| .'| . | -_|  _|     font:triangles  http://patorjk.com/software/taag/#p=display&f=Rectangles&t=header
                        // |_|_|___|__,|___|___|_| 

                        // ui.vertical_centered(|ui| {

                        //     ui.add(egui::Label::new(RichText::new(proxy_self.current_language.szyfrowanie_naglowek).size(proxy_self.formatowanie_rozmiar_czcionki_duza)).selectable(false));

                        // });

                        // ui.add_space( proxy_self.formatowanie_spacja_srednia);

                        //  _                 _              _   _   
                        // |_|___ ___ ___ ___| |_    ___ ___| |_| |_ 
                        // | |   | . | . |  _|  _|  | . | .'|  _|   |
                        // |_|_|_|  _|___|_| |_|    |  _|__,|_| |_|_|
                        //       |_|                |_|             
                        ui.horizontal(|ui|{
                            ui.add_space(margines_na_wybor_formatu_foty);
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wejsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.horizontal(|ui|{

                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);
                            
                            let btn_szyfrowanie_folder_wejsciowy : Response = ui.button(
                                RichText::new(proxy_self.current_language.general_ui_wybierz_folder)
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))
                            );

                            if btn_szyfrowanie_folder_wejsciowy.clicked() {
                                proxy_self.general_ui_status_przetwarzania = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {

                                    proxy_self.ui_pack_specyfic_folder_wejsciowy = path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wejsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wejsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
                                btn_szyfrowanie_folder_wejsciowy.rect.min.y+(btn_szyfrowanie_folder_wejsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, proxy_self.formatowanie_rozmiar_kolko, proxy_self.formatowanie_kolor_kolko_pelne);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, proxy_self.formatowanie_rozmiar_kolko_puste, (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,proxy_self.formatowanie_kolor_kolko_puste));

                            }



                        });
                        if proxy_self.ui_pack_specyfic_folder_wejsciowy.len() >= 50 {
                            let xxxx = &proxy_self.ui_pack_specyfic_folder_wejsciowy;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
    

                        }else if !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty(){

                            ui.add(egui::Label::new(RichText::new(&proxy_self.ui_pack_specyfic_folder_wejsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));

                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                        }else{
                            ui.add_space(proxy_self.formatowanie_spacja_duza);
                        }

                        //          _           _              _   _   
                        //  ___ _ _| |_ ___ _ _| |_    ___ ___| |_| |_ 
                        // | . | | |  _| . | | |  _|  | . | .'|  _|   |
                        // |___|___|_| |  _|___|_|    |  _|__,|_| |_|_|
                        //             |_|            |_|             
                        ui.horizontal(|ui|{
                            ui.add_space(margines_na_wybor_formatu_foty);
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.horizontal(|ui|{



                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

                            let btn_szyfrowanie_folder_wyjsciowy = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_folder).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));

                            if btn_szyfrowanie_folder_wyjsciowy.clicked() {
                                proxy_self.general_ui_status_przetwarzania = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    // Zapisujemy wybrany folder
                                    proxy_self.ui_pack_specyfic_folder_wyjsciowy = path.to_string_lossy().to_string();
                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.x - proxy_self.formatowanie_offset_kolko,
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.y+(btn_szyfrowanie_folder_wyjsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, proxy_self.formatowanie_rozmiar_kolko, proxy_self.formatowanie_kolor_kolko_pelne);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, proxy_self.formatowanie_rozmiar_kolko_puste, (proxy_self.formatowanie_rozmiar_kolko_puste_stroke,proxy_self.formatowanie_kolor_kolko_puste));

                            }

                        });
                        

                        if proxy_self.ui_pack_specyfic_folder_wyjsciowy.len() >= 50 {
                            let xxxx = &proxy_self.ui_pack_specyfic_folder_wyjsciowy;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
    

                        }else if !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty(){

                            ui.add(egui::Label::new(RichText::new(&proxy_self.ui_pack_specyfic_folder_wyjsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));

                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);

                        }else{
                            ui.add_space(proxy_self.formatowanie_spacja_duza);
                        }

                        ui.separator();



                        //  ___ _ _                          
                        // |  _|_| |___    ___ ___ _____ ___ 
                        // |  _| | | -_|  |   | .'|     | -_|
                        // |_| |_|_|___|  |_|_|__,|_|_|_|___|

                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_nazwa_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });        

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        let ui_pack_specyfic_nazwa_pliku_input = TextEdit::singleline(&mut proxy_self.ui_pack_specyfic_nazwa_pliku)
                            .char_limit(50)
                            .min_size(egui::Vec2{x:200.,y:30.})
                            .hint_text(proxy_self.current_language.general_ui_nazwa.to_string())
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki));
                        ui.vertical_centered_justified(|ui|{
                            ui.add(ui_pack_specyfic_nazwa_pliku_input);
                        });

                        ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_srednia);







                        //                                    _         
                        //  ___ ___ _____ ___ ___ ___ ___ ___|_|___ ___ 
                        // |  _| . |     | . |  _| -_|_ -|_ -| | . |   |
                        // |___|___|_|_|_|  _|_| |___|___|___|_|___|_|_|
                        //               |_|        
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_kompresja_tytul.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });
                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.columns(2,|column|{
                            column[0].vertical_centered_justified(|ui|{

                                let c_c_b = ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybor_kompresji, 0, RichText::new(proxy_self.current_language.general_ui_label_brak).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                if c_c_b.clicked(){proxy_self.general_ui_status_przetwarzania=0};
                            });
                            // ui.add(egui::Label::new("||").selectable(false));
                            // let c_c_z = ui.add(egui::Label::new(RichText::new("Zstd").strikethrough());
                            // if c_c_z.hovered(){
                            //     ui.add(egui::Label::new("Ma problemy egzystencjonalne");
                            // };
                            column[1].vertical_centered_justified(|ui|{
                                let c_c_z = ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybor_kompresji, 1, RichText::new("Zstd").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                if c_c_z.clicked(){proxy_self.general_ui_status_przetwarzania=0};
                            });

                        });


                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        
                        ui.horizontal(|ui|{
                            ui.add_space(margines_na_wybor_formatu_foty);
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.szyfrowanie_kompresja.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                            let compression_empty_let:bool = proxy_self.ui_pack_specyfic_wybor_kompresji>=1;

                            ui.add_enabled(compression_empty_let,|ui: &mut egui::Ui|{
                                ui.add(egui::Slider::new(&mut proxy_self.ui_pack_specyfic_poziom_kompresji, 1..=22).text(""))

                            });
                        
                        });
                        ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        ui.separator();
                        
                        ui.add_space( proxy_self.formatowanie_spacja_srednia);

                        //                          _   _         
                        //  ___ ___ ___ ___ _ _ ___| |_|_|___ ___
                        // | -_|   |  _|  _| | | . |  _| | . |   |
                        // |___|_|_|___|_| |_  |  _|_| |_|___|_|_|
                        //                 |___|_|                

                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.szyfrowanie_szyfrowanie_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(2,|column|{
                            column[0].vertical_centered_justified(|ui|{
                                let c_e_b =ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybor_szyfrowania, 0,  RichText::new(proxy_self.current_language.general_ui_label_brak).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                if c_e_b.clicked(){proxy_self.general_ui_status_przetwarzania=0}
                            });
                            // ui.add(egui::Label::new("||").selectable(false));
                            column[1].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut proxy_self.ui_pack_specyfic_wybor_szyfrowania, 1,  RichText::new("chacha, do not use!").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)).strikethrough());
                            // if c_e_x.clicked() {proxy_self.general_ui_status_przetwarzania=0}
                            // let c_e_x = ui.add(egui::Label::new(RichText::new("chacha").strikethrough());
                                // if c_e_x.hovered(){
                                //     ui.add(egui::Label::new(&proxy_self.current_language.problem_egzystencjonalny.to_string()).selectable(false));
                                // };
                            });


                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(1, |column|{
                            if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 0{
                                column[0].vertical_centered_justified(|ui|{
                                    let przerwa_tam_gdzie_haslo = (30. + proxy_self.formatowanie_rozmiar_czcionki_srednia + proxy_self.formatowanie_spacja_mala - proxy_self.formatowanie_rozmiar_czcionki_duza - 1.) / 2.;

                                    ui.add_space( przerwa_tam_gdzie_haslo);
                                    ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_wylaczone).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)).color(fiolet_ciemny)).selectable(false));
                                

                                    ui.add_space( przerwa_tam_gdzie_haslo);
                                });

                            }else{
                                column[0].vertical_centered_justified(|ui|{
                                    ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_tytul.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                                    ui.add_space( proxy_self.formatowanie_spacja_mala);
                                    let password_input = TextEdit::singleline(&mut proxy_self.ui_pack_specyfic_password)
                                    .password(true)
                                    .hint_text(RichText::new(proxy_self.current_language.general_ui_haslo.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                                    .min_size(Vec2{x:200.,y:30.});
                                    ui.add(password_input);
                                });

                            }
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_srednia);
                        ui.separator();
                        ui.add_space(proxy_self.formatowanie_spacja_srednia);

                        //  _                 _     _       
                        // | |_ ___ _____ ___| |___| |_ ___ 
                        // |  _| -_|     | . | | .'|  _| -_|
                        // |_| |___|_|_|_|  _|_|__,|_| |___|
                        //               |_|                

                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_szablony_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);

                        ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                            .width(proxy_self.general_ui_szerokosc_okna / 4.)
                            .selected_text(RichText::new( match proxy_self.ui_pack_specyfic_template.as_str(){
                                "none" => proxy_self.current_language.szablony_wybor_0,
                                "assets" => proxy_self.current_language.szablony_wybor_1,
                                "images" => proxy_self.current_language.szablony_wybor_2,
                                "audio" => proxy_self.current_language.szablony_wybor_3,
                                "3d_model" => proxy_self.current_language.szablony_wybor_4,
                                "documents" => proxy_self.current_language.szablony_wybor_5,
                                "raw_photos" => proxy_self.current_language.szablony_wybor_6,
                                _ => proxy_self.current_language.err_value_overflow
                            }/*proxy_self.ui_pack_specyfic_template.clone()*/)
                        .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)))
                            .show_ui(ui, |ui| {

                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "none".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_0).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "assets".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_1).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "images".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_2).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "audio".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_3).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "3d_model".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_4).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "documents".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_5).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.ui_pack_specyfic_template, "raw_photos".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_6).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki)));
                        
                        });
                        

                        //  _       _   _           
                        // | |_ _ _| |_| |_ ___ ___ 
                        // | . | | |  _|  _| . |   |
                        // |___|___|_| |_| |___|_|_|
                        
                        let sprawdzacz_plikow_kompresji= !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty() && !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
                               

                        let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

                            proxy_self.current_language.szyfrowanie_przycisk_ok}
                            else
                            {proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                        
                        // ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        // let t_p_d= match proxy_self.general_ui_status_przetwarzania{
                        //     0 => tekst_przycisku_kompresji,
                        //     1 => &&general_ui_loading_tekst as &str,
                        //     2 => &proxy_self.current_language.szyfrowanie_przycisk_koniec,
                        //     3 => &proxy_self.current_language.szyfrowanie_przycisk_3,
                        //     4 => &proxy_self.current_language.szyfrowanie_przycisk_4,
                        //     _ => ""
                        // };


                        let blblblblblblbl = !proxy_self.ui_pack_specyfic_folder_wejsciowy.is_empty() && proxy_self.general_ui_status_przetwarzania !=1 && !proxy_self.ui_pack_specyfic_folder_wyjsciowy.is_empty() && !proxy_self.ui_pack_specyfic_nazwa_pliku.is_empty();
                        let szyfr_butt_col = match (
                            sprawdzacz_plikow_kompresji,
                            proxy_self.general_ui_status_przetwarzania
                        ){
                            (true,0) => Color32::DARK_GREEN,
                            (true,1) => zolty_ciemny,
                            _ =>szarawy_ciemny
                        };

                        match proxy_self.rx.try_recv() {
                            Ok(Ok(afasdaf)) => {
                                let danene = afasdaf.lock().unwrap();
                                proxy_self.general_ui_status_przetwarzania = 2;
                                proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki = danene[0];
                                proxy_self.ui_pack_specyfic_statystyki_czas_sekundy = danene[1] as u64;
                                proxy_self.ui_pack_specyfic_statystyki_czas_milisekundy = danene[2] as u32;
                                play_finish_sound(proxy_self.ui_ustawienia_glosnosc);
                            }
                            Ok(Err(e)) => {
                                proxy_self.general_ui_status_przetwarzania = 3;
                                proxy_self.ui_pack_specyfic_error_3 = e.to_string();
                                // eprintln!("Błąd: {}", e);
                            }
                            Err(std::sync::mpsc::TryRecvError::Empty) => {
                                ctx.request_repaint();
                                match proxy_self.general_ui_loading{

                                    781 => {proxy_self.general_ui_loading = 0; proxy_self.general_ui_loading_tekst="[______]"},
                                    751..=780 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[=_____]"},
                                    721..=750 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[==____]"},
                                    691..=720 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[===___]"},
                                    661..=690 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[====__]"},
                                    631..=660 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[=====_]"},
                                    601..=630 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[======]"},
                                    571..=600 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[<=====]"},
                                    541..=570 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[_<====]"},
                                    511..=540 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[__<===]"},
                                    481..=510 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[___<==]"},
                                    451..=480 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[____<=]"},
                                    421..=450 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[_____<]"},
                                    391..=420 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[______]"},
                                    361..=390 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[_____=]"},
                                    331..=360 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[____==]"},
                                    301..=330 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[___===]"},
                                    271..=300 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[__====]"},
                                    241..=270 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[_=====]"},
                                    211..=240 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[======]"},
                                    181..=210 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[=====>]"},
                                    151..=180 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[====>_]"},
                                    121..=150 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[===>__]"},
                                    91..=120 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[==>___]"},
                                    61..=90 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[=>____]"},
                                    31..=60 => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[>_____]"},
                                    _ => {proxy_self.general_ui_loading += 1; proxy_self.general_ui_loading_tekst="[______]"},
                                }  

                            }
                            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                                proxy_self.general_ui_status_przetwarzania = 4;
                                proxy_self.ui_pack_specyfic_error_3 = "Disconected".to_string();
                            }
                        }
                        ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        let t_p_d= match proxy_self.general_ui_status_przetwarzania{
                            0 => RichText::new(tekst_przycisku_kompresji.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
                            1 => RichText::new(proxy_self.general_ui_loading_tekst.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,1)).color(Color32::BLACK),
                            2 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
                            3 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_3.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
                            4 => RichText::new(proxy_self.current_language.szyfrowanie_przycisk_4.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki)),
                            _ => RichText::new("".to_string())
                        };


                        // ui.put(Rect::from_center_size(Pos2{x:1.,y:500.},Vec2{x:1.,y:2.}), egui::Label::new("gadfgs"));
                        if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.general_ui_szerokosc_okna / 4.),y:proxy_self.formatowanie_wysokosc_przycisku},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d
                           )
                        // ui.vertical_centered(|ui|{
                        //     if ui.add(egui::Button::new(t_p_d
                            // .size( proxy_self.formatowanie_rozmiar_czcionki_duza))
                            .min_size(egui::vec2((proxy_self.general_ui_szerokosc_okna / 2.) - 100.0, 40.0))
                            .corner_radius(10.)
                            .fill(szyfr_butt_col))
                            .clicked() && blblblblblblbl{



                                // if blblblblblblbl{
                                    proxy_self.general_ui_status_przetwarzania = 0;
                                    proxy_self.general_ui_loading = 0;
                                    let rozszerzenie_plikku = if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 0 {"jrz"} else if proxy_self.ui_pack_specyfic_wybor_szyfrowania == 1 {"jrzs"} else{"bin"};
                                    let output_file = format!("{}/{}.{}" , proxy_self.ui_pack_specyfic_folder_wyjsciowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);
                                    let index_file = format!("{}/{}{}.idx", proxy_self.ui_pack_specyfic_folder_wyjsciowy, proxy_self.ui_pack_specyfic_nazwa_pliku,rozszerzenie_plikku);

                                    let arc_z_u8 = Arc::new(Mutex::new(vec![proxy_self.ui_pack_specyfic_wybor_szyfrowania,proxy_self.ui_pack_specyfic_wybor_kompresji,proxy_self.ui_pack_specyfic_poziom_kompresji]));
                                    let arc_z_bool = Arc::new(Mutex::new(vec![proxy_self.ui_ustawienia_tworzenie_pliku_lua]));
                                    let arc_z_str = Arc::new(Mutex::new(vec![proxy_self.ui_pack_specyfic_folder_wejsciowy.clone().to_string(),output_file.clone().to_string(),index_file.clone().to_string(),proxy_self.ui_pack_specyfic_template.clone().to_string(),proxy_self.ui_pack_specyfic_password.clone().to_string()]));
                                        
                                        

                                    let tx_clone = proxy_self.tx.clone();
                                    std::thread::spawn(move || {
                                        let result = encrypt_bez_async_i_bez_chacha20::encrypt_folder(
                                            arc_z_str,
                                            arc_z_u8,
                                            arc_z_bool,
                                        );
                                        
                                        match tx_clone.send(result) {
                                            Ok(_) => println!("Wysłano wynik"),
                                            Err(e) => eprintln!("Błąd wysyłania: {}", e),
                                        }
                                    });
                                    
                                    proxy_self.general_ui_status_przetwarzania = 1;

                                }
                            // }

                                            

                            // });
}