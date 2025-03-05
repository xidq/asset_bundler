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
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wejsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.horizontal(|ui|{

                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);
                            
                            let btn_szyfrowanie_folder_wejsciowy : Response = ui.button(
                                RichText::new(proxy_self.current_language.general_ui_wybierz_folder)
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))
                            );

                            if btn_szyfrowanie_folder_wejsciowy.clicked() {
                                proxy_self.czy_to_juz_koniec = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {

                                    proxy_self.imput_folder_path = path.to_string_lossy().to_string();

                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wejsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wejsciowy.rect.min.x - proxy_self.offset_cirkul,
                                btn_szyfrowanie_folder_wejsciowy.rect.min.y+(btn_szyfrowanie_folder_wejsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.imput_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wejsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                            }



                        });
                        if proxy_self.imput_folder_path.len() >= 50 {
                            let xxxx = &proxy_self.imput_folder_path;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
    

                        }else if !proxy_self.imput_folder_path.is_empty(){

                            ui.add(egui::Label::new(RichText::new(&proxy_self.imput_folder_path).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));

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
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_wybierz_folder_wyjsciowy).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.horizontal(|ui|{



                            ui.add_space(proxy_self.formatowanie_spacja_duza+margines_na_wybor_formatu_foty);

                            let btn_szyfrowanie_folder_wyjsciowy = ui.button(RichText::new(proxy_self.current_language.general_ui_wybierz_folder).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));

                            if btn_szyfrowanie_folder_wyjsciowy.clicked() {
                                proxy_self.czy_to_juz_koniec = 0;
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    // Zapisujemy wybrany folder
                                    proxy_self.output_folder_path = path.to_string_lossy().to_string();
                                }
                            }

                            let btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy = Pos2::new(
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.x - proxy_self.offset_cirkul,
                                btn_szyfrowanie_folder_wyjsciowy.rect.min.y+(btn_szyfrowanie_folder_wyjsciowy.rect.size().y / 2.));
                            
                            if !proxy_self.output_folder_path.is_empty(){

                                ui.painter().circle_filled(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, proxy_self.full_cirkul_sajz, proxy_self.full_cirkul_kolor);

                            } else {

                                ui.painter().circle_stroke(btn_pozycjonowanie_szyfrowanie_folder_wyjsciowy, proxy_self.pusty_cirkul_sajz, (proxy_self.pusty_cirkul_ma_stroke,proxy_self.pusty_cirkul_kolor));

                            }

                        });
                        

                        if proxy_self.output_folder_path.len() >= 50 {
                            let xxxx = &proxy_self.output_folder_path;
                            let startu = &xxxx[..=15];
                            let endu = &xxxx[xxxx.len()-30 ..];
                            ui.add(egui::Label::new(RichText::new(
                                format!("{}/.../{}",startu,endu))
                                .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            ui.add_space(proxy_self.formatowanie_spacja_duza-proxy_self.formatowanie_rozmiar_czcionki_srednia-4.);
    

                        }else if !proxy_self.output_folder_path.is_empty(){

                            ui.add(egui::Label::new(RichText::new(&proxy_self.output_folder_path).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));

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
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_nazwa_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });        

                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        let output_name_input = TextEdit::singleline(&mut proxy_self.output_name)
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







                        //                                    _         
                        //  ___ ___ _____ ___ ___ ___ ___ ___|_|___ ___ 
                        // |  _| . |     | . |  _| -_|_ -|_ -| | . |   |
                        // |___|___|_|_|_|  _|_| |___|___|___|_|___|_|_|
                        //               |_|        
                        ui.vertical_centered_justified(|ui|{
                            ui.add(egui::Label::new(RichText::new(&proxy_self.current_language.general_ui_kompresja_tytul.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space( proxy_self.formatowanie_spacja_mala);

                        ui.columns(2,|column|{
                            column[0].vertical_centered_justified(|ui|{

                                let c_c_b = ui.selectable_value(&mut proxy_self.toggle_compression, 0, RichText::new(proxy_self.current_language.general_ui_label_brak).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if c_c_b.clicked(){proxy_self.czy_to_juz_koniec=0};
                            });
                            // ui.add(egui::Label::new("||").selectable(false));
                            // let c_c_z = ui.add(egui::Label::new(RichText::new("Zstd").strikethrough());
                            // if c_c_z.hovered(){
                            //     ui.add(egui::Label::new("Ma problemy egzystencjonalne");
                            // };
                            column[1].vertical_centered_justified(|ui|{
                                let c_c_z = ui.selectable_value(&mut proxy_self.toggle_compression, 1, RichText::new("Zstd").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if c_c_z.clicked(){proxy_self.czy_to_juz_koniec=0};
                            });

                        });


                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        
                        ui.horizontal(|ui|{
                            ui.add_space(margines_na_wybor_formatu_foty);
                            ui.add(egui::Label::new(RichText::new(&proxy_self.current_language.szyfrowanie_kompresja.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                            let compression_empty_let:bool;
                            if proxy_self.toggle_compression>=1{compression_empty_let=true}else{compression_empty_let=false};
                            ui.add_enabled(compression_empty_let,|ui: &mut egui::Ui|{
                                ui.add(egui::Slider::new(&mut proxy_self.poziom_kompresji, 1..=22).text(""))

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
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.szyfrowanie_szyfrowanie_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(2,|column|{
                            column[0].vertical_centered_justified(|ui|{
                                let c_e_b =ui.selectable_value(&mut proxy_self.toggle_encryption, 0,  RichText::new(proxy_self.current_language.general_ui_label_brak).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                if c_e_b.clicked(){proxy_self.czy_to_juz_koniec=0}
                            });
                            // ui.add(egui::Label::new("||").selectable(false));
                            column[1].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut proxy_self.toggle_encryption, 1,  RichText::new("chacha, do not use!").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)).strikethrough());
                            // if c_e_x.clicked() {proxy_self.czy_to_juz_koniec=0}
                            // let c_e_x = ui.add(egui::Label::new(RichText::new("chacha").strikethrough());
                                // if c_e_x.hovered(){
                                //     ui.add(egui::Label::new(&proxy_self.current_language.problem_egzystencjonalny.to_string()).selectable(false));
                                // };
                            });


                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);
                        ui.columns(1, |column|{
                            if proxy_self.toggle_encryption == 0{
                                column[0].vertical_centered_justified(|ui|{
                                    let przerwa_tam_gdzie_haslo = (30. + proxy_self.formatowanie_rozmiar_czcionki_srednia + proxy_self.formatowanie_spacja_mala - proxy_self.formatowanie_rozmiar_czcionki_duza - 1.) / 2.;

                                    ui.add_space( przerwa_tam_gdzie_haslo);
                                    ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_wylaczone).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)).color(fiolet_ciemny)).selectable(false));
                                

                                    ui.add_space( przerwa_tam_gdzie_haslo);
                                });

                            }else{
                                column[0].vertical_centered_justified(|ui|{
                                    ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_haslo_tytul.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
                                    ui.add_space( proxy_self.formatowanie_spacja_mala);
                                    let password_input = TextEdit::singleline(&mut proxy_self.password)
                                    .password(true)
                                    .hint_text(RichText::new(&proxy_self.current_language.general_ui_haslo.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
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
                            ui.add(egui::Label::new(RichText::new(proxy_self.current_language.general_ui_szablony_tytul).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki))).selectable(false));
                        });
                        ui.add_space(proxy_self.formatowanie_spacja_mala);

                        ComboBox::from_label(""/*&proxy_self.current_language.szyfrowanie_wybierz_ustawienia_prekonfigurowane.to_string()*/)
                            .width(proxy_self.szerokosc_okna / 4.)
                            .selected_text(RichText::new(proxy_self.template.clone()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                            .show_ui(ui, |ui| {

                                ui.selectable_value(&mut proxy_self.template, "none".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_0).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "assets".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_1).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "images".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_2).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "audio".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_3).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "3d_model".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_4).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "documents".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_5).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                                ui.selectable_value(&mut proxy_self.template, "raw_photos".to_string(), RichText::new(proxy_self.current_language.szablony_wybor_6).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
                        
                        });
                        

                        //  _       _   _           
                        // | |_ _ _| |_| |_ ___ ___ 
                        // | . | | |  _|  _| . |   |
                        // |___|___|_| |_| |___|_|_|
                        
                        let sprawdzacz_plikow_kompresji= if !proxy_self.imput_folder_path.is_empty() && !proxy_self.output_folder_path.is_empty() && !proxy_self.output_name.is_empty(){
                            true
                        }else{
                            false
                        };
                               

                        let tekst_przycisku_kompresji = if sprawdzacz_plikow_kompresji{

                            &proxy_self.current_language.szyfrowanie_przycisk_ok}
                            else
                            {&proxy_self.current_language.szyfrowanie_przycisk_nie_ok};
                        
                        // ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        // let t_p_d= match proxy_self.czy_to_juz_koniec{
                        //     0 => tekst_przycisku_kompresji,
                        //     1 => &&sz_loading_anim as &str,
                        //     2 => &proxy_self.current_language.szyfrowanie_przycisk_koniec,
                        //     3 => &proxy_self.current_language.szyfrowanie_przycisk_3,
                        //     4 => &proxy_self.current_language.szyfrowanie_przycisk_4,
                        //     _ => ""
                        // };


                        let blblblblblblbl = if !proxy_self.imput_folder_path.is_empty() && proxy_self.czy_to_juz_koniec !=1 && !proxy_self.output_folder_path.is_empty() && !proxy_self.output_name.is_empty() && !proxy_self.zapis_pracuje{true}else{false};
                        let szyfr_butt_col = if sprawdzacz_plikow_kompresji == true && proxy_self.czy_to_juz_koniec ==0{Color32::DARK_GREEN}else if sprawdzacz_plikow_kompresji == true && proxy_self.czy_to_juz_koniec ==1{zolty_ciemny}else{szarawy_ciemny};

                        match proxy_self.rx.try_recv() {
                            Ok(Ok(afasdaf)) => {
                                let danene = afasdaf.lock().unwrap();
                                proxy_self.czy_to_juz_koniec = 2;
                                proxy_self.przetworzone_pliki = danene[0];
                                proxy_self.timestamp_sekundy = danene[1] as u64;
                                proxy_self.timestamp_milisekundy = danene[2] as u32;
                                play_finish_sound(proxy_self.ustawienia_glosnosc);
                            }
                            Ok(Err(e)) => {
                                proxy_self.czy_to_juz_koniec = 3;
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
                                proxy_self.czy_to_juz_koniec = 4;
                                proxy_self.szyfrowanie_err_3 = "Disconected".to_string();
                            }
                        }
                        ui.add_space( proxy_self.formatowanie_spacja_srednia);
                        let t_p_d= match proxy_self.czy_to_juz_koniec{
                            0 => RichText::new(tekst_przycisku_kompresji.to_string()),
                            1 => RichText::new(proxy_self.sz_loading_anim.to_string()).monospace().color(Color32::BLACK),
                            2 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_koniec.to_string()),
                            3 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_3.to_string()),
                            4 => RichText::new(&proxy_self.current_language.szyfrowanie_przycisk_4.to_string()),
                            _ => RichText::new("".to_string())
                        };


                        // ui.put(Rect::from_center_size(Pos2{x:1.,y:500.},Vec2{x:1.,y:2.}), egui::Label::new("gadfgs"));
                        if ui.put(Rect::from_center_size(Pos2{x:(proxy_self.szerokosc_okna / 4.),y:proxy_self.wysokosc_btn_egzekucyjny},Vec2{x:250.,y:40.}),egui::Button::new(t_p_d
                            .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.wybor_czcionki)))
                        // ui.vertical_centered(|ui|{
                        //     if ui.add(egui::Button::new(t_p_d
                            // .size( proxy_self.formatowanie_rozmiar_czcionki_duza))
                            .min_size(egui::vec2(250.0, 40.0))
                            .corner_radius(10.)
                            .fill(szyfr_butt_col))
                            .clicked() {



                                if blblblblblblbl{
                                    proxy_self.czy_to_juz_koniec = 0;
                                    let rozszerzenie_plikku = if proxy_self.toggle_encryption == 0 {"jrz"} else if proxy_self.toggle_encryption == 1 {"jrzs"} else{"bin"};
                                    let output_file = format!("{}/{}.{}" , proxy_self.output_folder_path, proxy_self.output_name,rozszerzenie_plikku);
                                    let index_file = format!("{}/{}.idx", proxy_self.output_folder_path, proxy_self.output_name);

                                    let arc_z_u8 = Arc::new(Mutex::new(vec![proxy_self.toggle_encryption,proxy_self.toggle_compression,proxy_self.poziom_kompresji]));
                                    let arc_z_bool = Arc::new(Mutex::new(vec![proxy_self.debug_create_lua_file]));
                                    let arc_z_str = Arc::new(Mutex::new(vec![proxy_self.imput_folder_path.clone().to_string(),output_file.clone().to_string(),index_file.clone().to_string(),proxy_self.template.clone().to_string(),proxy_self.password.clone().to_string()]));
                                        
                                        

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
                                    
                                    proxy_self.czy_to_juz_koniec = 1;

                                }}

                                            

                            // });
}