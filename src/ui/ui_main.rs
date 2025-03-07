use egui::{Color32/*, FontId, Pos2*/, RichText, Stroke/*, Window,CornerRadius*/};
use egui::{CentralPanel, /*FontFamily,*/FontDefinitions,FontData};
// use rand::random_range;
use ecolor::Hsva;
// use rfd::*;
// use crate::utils::*;

use std::sync::Arc;
// use crate::{image_actions, ui_play_sound::*};
// use crate::encrypt;
// use crate::encrypt_bez_async_i_bez_chacha20;
// use crate::decrypt_copy;
use crate::ui::*;
use crate::ui::ui_defaults::Appencja;
// use core::random;
// use std::f32::consts::PI;
// use std::thread;
// use std::path::Path;
// use std::time::Duration;
// use rodio::*;
// use std::sync::Arc;
// use chrono::offset::Local;
// use crate::ui::ui_play_sound::play_finish_sound;
use crate::ui::{
    ui_left_panel_1_kompresja::ui_left_panel_encrypt,
    ui_left_panel_1_dekompresja::ui_left_panel_decrypt,
    ui_left_panel_2_foto_przetwarzanie::ui_left_panel_foty_przetwarzanie,
    ui_right_panel_info_0::right_panel_info_main,
    ui_right_panel_info_1::right_panel_info_stats,
    ui_right_panel_info_2::ui_right_panel_info_0_2,
    ui_right_panel_info_3::ui_right_panel_info_0_3,
    ui_right_panel_info_4::ui_right_panel_info_0_4,
    ui_right_panel_settings::ui_right_panel_settings_0,
    ui_right_panel_debug_0::ui_right_panel_debug_0,
    ui_change_font::wybrana_aktualna_czcionka,
    ui_left_panel_2_foto_laczenie::ui_left_panel_foty_laczenie,
};
// use rand::seq::IteratorRandom;
// use toml::Value;
// use tokio::*;
// use crate::data_checker;
// use std::vec::*;
// use encrypt::StatusPakExport;












// -------------------------------------------------------------------------------------------------------------------------------------------
// ████████ ██ ████████ ██      ███████ 
//    ██    ██    ██    ██      ██      
//    ██    ██    ██    ██      █████   
//    ██    ██    ██    ██      ██      
//    ██    ██    ██    ███████ ███████

impl Appencja {

    pub fn name() -> &'static str {
        concat!("Asset Bundler v", env!("CARGO_PKG_VERSION"))
    }

}

// -------------------------------------------------------------------------------------------------------------------------------------------

impl eframe::App for Appencja {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        self.current_language = match self.toggle_language{
            2 =>ui_language::Language::debug(),
            1 =>ui_language::Language::english(),
            // 3 =>ui_language::Language::japanese(),
            _ =>ui_language::Language::polish()
        };
        // if self.toggle_language {

        //     self.current_language = ui_language::Language::english();

        // } else {

        //     self.current_language = ui_language::Language::polish();

        // }

        // -------------------------------------------------------------------------------------------------------------------------------------------
        //  ██████  ██    ██ ███████ ██████  ██████  ██ ██████  ███████ 
        // ██    ██ ██    ██ ██      ██   ██ ██   ██ ██ ██   ██ ██      
        // ██    ██ ██    ██ █████   ██████  ██████  ██ ██   ██ █████   
        // ██    ██  ██  ██  ██      ██   ██ ██   ██ ██ ██   ██ ██      
        //  ██████    ████   ███████ ██   ██ ██   ██ ██ ██████  ███████ 
                                                                     
                                                                     
        // ███████  ██████  ███    ██ ████████                          
        // ██      ██    ██ ████   ██    ██                             
        // █████   ██    ██ ██ ██  ██    ██                             
        // ██      ██    ██ ██  ██ ██    ██                             
        // ██       ██████  ██   ████    ██   
                // Konfiguracja czcionki japońskiej
        let mut font_definitions = FontDefinitions::default();

        // // Wczytanie czcionki bezpośrednio do pliku wykonywalnego
        let font_data = FontData::from_static(include_bytes!("../../assets/Forum-Regular.ttf"));

        font_definitions.font_data.insert("XanhMono".to_owned(), Arc::new(font_data));
        font_definitions.families.insert(

            egui::FontFamily::Proportional, // Use default proportional to override, u can add new like in next example
            vec!["XanhMono".to_owned()],

        );
        // let font_data = FontData::from_static(include_bytes!("../../assets/NotoSerifJP-VariableFont_wght.ttf"));
        // font_definitions.font_data.insert("japanese_font".to_owned(), Arc::new(font_data));
        // font_definitions.families.insert(
        //         egui::FontFamily::Name("japanese_font".to_owned().into()), // Używamy dedykowanej rodziny
        //         vec!["japanese_font".to_owned()],
        //     );

        //confirm
        ctx.set_fonts(font_definitions);



                // // Konfiguracja czcionki japońskiej
                // let mut font_definitions = FontDefinitions::default();
        
                // // Wczytanie czcionki bezpośrednio do pliku wykonywalnego
                // let font_data = FontData::from_static(include_bytes!("../assets/NotoSerifJP-VariableFont_wght.ttf"));

        
                // font_definitions.font_data.insert("japanese_font".to_owned(), Arc::new(font_data));
                // font_definitions.families.insert(
                //     egui::FontFamily::Name("japanese_font".to_owned().into()), // Używamy dedykowanej rodziny
                //     vec!["japanese_font".to_owned()],
                // );

        
                // // Tutaj konfigurujesz czcionki w kontekście, ale bez globalnej zmiany
                // ctx.set_fonts(font_definitions);
        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██   ██ ███████ ██    ██                                   
        // ██  ██  ██       ██  ██                                    
        // █████   █████     ████                                     
        // ██  ██  ██         ██                                      
        // ██   ██ ███████    ██                                      
                                                                
                                                                
        // ██████  ██ ███    ██ ██████  ██ ███    ██  ██████  ███████ 
        // ██   ██ ██ ████   ██ ██   ██ ██ ████   ██ ██       ██      
        // ██████  ██ ██ ██  ██ ██   ██ ██ ██ ██  ██ ██   ███ ███████ 
        // ██   ██ ██ ██  ██ ██ ██   ██ ██ ██  ██ ██ ██    ██      ██ 
        // ██████  ██ ██   ████ ██████  ██ ██   ████  ██████  ███████ 
        // naciskanie klawiszy i tego konsekwencje
        if ctx.input(|i| i.key_pressed(egui::Key::E) && i.modifiers.ctrl) {
            self.show_debug_labels = !self.show_debug_labels; 
        }
        if ctx.input(|i| i.key_pressed(egui::Key::B) && i.modifiers.ctrl) {
            self.brainroot = !self.brainroot; 
        }
        // if ctx.input(|i| i.key_pressed(egui::Key::R) && i.modifiers.ctrl) {
        //     self.toggle_encryption = !self.toggle_encryption; 
        // }
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.settings_toggle = !self.settings_toggle; 
        }

        


        // -------------------------------------------------------------------------------------------------------------------------------------------
        //  ██████  ██████  ██       ██████  ██████  ███████ 
        // ██      ██    ██ ██      ██    ██ ██   ██ ██      
        // ██      ██    ██ ██      ██    ██ ██████  ███████ 
        // ██      ██    ██ ██      ██    ██ ██   ██      ██ 
        //  ██████  ██████  ███████  ██████  ██   ██ ███████

        //kolory niestandardowe:
        let szarawy_ciemny_hsva = Hsva {
            h: 0.5,    // 0-1
            s: 0.2,    // 0-1
            v: 0.05,    // 0-1
            a: 1.0,    // 0-1
        };

        let szarawy_ciemny: Color32 = szarawy_ciemny_hsva.into();

        let fiolet_ciemny_hsva = Hsva {
            h: 0.9,    // Hue: 0 degrees (czerwony kolor)
            s: 0.7,    // Saturation: pełna saturacja (intensywny kolor)
            v: 0.2,    // Value: pełna jasność
            a: 1.0,    // Alpha: pełna przezroczystość
        };

        let fiolet_ciemny: Color32 = fiolet_ciemny_hsva.into();

        let zolty_ciemny_hsva = Hsva {
            h: 0.091,    // Hue: 0 degrees (czerwony kolor)
            s: 0.57,    // Saturation: pełna saturacja (intensywny kolor)
            v: 0.4,    // Value: pełna jasność
            a: 1.0,    // Alpha: pełna przezroczystość
        };

        let zolty_ciemny: Color32 = zolty_ciemny_hsva.into();



        //another checker
        // let sprawdzacz_plikow_dekompresja: bool = 
        //     Path::new(&self.imput_de_dat_folder_path)
        //         .extension()
        //         .map_or(false,
        //             |ext| ext == "jrz" || ext == "jrzs") &&
        //     Path::new(&self.imput_de_idx_folder_path)
        //         .extension()
        //         .map_or(false, 
        //             |ext| ext == "idx") &&
        //     !self.output_de_folder_path.is_empty();

        //override some things bcoz i want to
        let visuals = ctx.style().visuals.clone();
        let mut visuals = visuals;
        visuals.selection.bg_fill = fiolet_ciemny; // Zmiana tła na zielony przy selekcji
        visuals.selection.stroke = Stroke{width:2.,color:Color32::WHITE};
        ctx.set_visuals(visuals);

        
        // pub fn wybrana_aktualna_czcionka(rozmiar: f32,wybor_czcionki: u8) -> FontId{
        //     let aktualna_czcionka_aaaa = match wybor_czcionki {
        //         1 =>egui::FontFamily::Name("japanese_font".to_owned().into()),
        //         _ =>egui::FontFamily::Proportional
        //     };
        //     FontId{size:rozmiar,family:aktualna_czcionka_aaaa}

        // }


        match self.toggle_language{
            3 => self.wybor_czcionki=1,
            _ => self.wybor_czcionki=0

        }


        // 1 =>FontId{size:16.,family:egui::FontFamily::Name("japaneese".to_owned().into())},
        // _ =>FontId{size:16.,family:egui::FontFamily::Proportional}
        // FontId{size:16.,family:egui::FontFamily::Proportional}

        egui_extras::install_image_loaders(ctx);

        ctx.request_repaint();
        // let centralny_panel = egui::containers::Frame {

        //     // fill: Color32::DARK_BLUE,
        //     corner_radius:CornerRadius{nw:200,ne:200,sw:2,se:2},
        //     ..Default::default()
        // };

        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██    ██ ██ 
        // ██    ██ ██ 
        // ██    ██ ██ 
        // ██    ██ ██ 
        //  ██████  ██ 

        CentralPanel::default()
        .show(ctx, |_ui| { //.frame(centralny_panel)


        // -------------------------------------------------------------------------------------------------------------------------------------------
        // ██       ███████ ███████ ████████          
        // ██       ██      ██         ██             
        // ██       █████   █████      ██             
        // ██       ██      ██         ██             
        // ████████ ███████ ██         ██             
                                                
                                                
        // ██████   █████  ███    ██ ███████ ██      
        // ██   ██ ██   ██ ████   ██ ██      ██      
        // ██████  ███████ ██ ██  ██ █████   ██      
        // ██      ██   ██ ██  ██ ██ ██      ██      
        // ██      ██   ██ ██   ████ ███████ ██████ 

            egui::SidePanel::left("lewy_panel")
            .exact_width(self.szerokosc_okna / 2.)
            .resizable(false)
            .show(ctx, |ui|{

                ui.horizontal(|ui|{
                    // ui.add(egui::Label::new(encrypt::encrypt_folder::Result);

                    ui.add_space(if self.show_debug_labels{(self.szerokosc_okna / 2.) - 160.} else {(self.szerokosc_okna / 2.) - 90.});
                    ui.selectable_value(&mut self.toggle_language, 0, RichText::new("PL").size(self.formatowanie_rozmiar_czcionki_srednia));
                    ui.add(egui::Label::new("||").selectable(false));
                    ui.selectable_value(&mut self.toggle_language, 1, RichText::new("EN").size(self.formatowanie_rozmiar_czcionki_srednia));
                    // ui.add(egui::Label::new("||").selectable(false));
                    // ui.selectable_value(&mut self.toggle_language, 3, RichText::new("JP").size(self.formatowanie_rozmiar_czcionki_srednia));
                    if self.show_debug_labels{
                        ui.add(egui::Label::new("||").selectable(false));
                        ui.selectable_value(&mut self.toggle_language, 2, RichText::new("Debug").size(self.formatowanie_rozmiar_czcionki_srednia));
                    }

                });
                ui.add_space(self.formatowanie_spacja_srednia);
                ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                    ui.selectable_value(&mut self.ustawienie_lewy_panel_foty_szyfrowanie, false, RichText::new(self.current_language.general_ui_label_binary).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienie_lewy_panel_foty_szyfrowanie, true, RichText::new(self.current_language.general_ui_label_images).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                    });
                });
                ui.add_space(self.formatowanie_spacja_mala);
                ui.separator();
                ui.add_space(self.formatowanie_spacja_mala);
                if self.ustawienie_lewy_panel_foty_szyfrowanie{
                    ui.columns(2, |columns|{
                    columns[0].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienie_lewy_panel_foty_podopcje, 0, RichText::new(self.current_language.przetwarzanie_naglowek).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                        });
                        columns[1].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.ustawienie_lewy_panel_foty_podopcje, 1, RichText::new(self.current_language.general_ui_kompilacja_rgb_tytul).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                        });
                    });
                } else {
                    ui.columns(2, |columns|{
                        columns[0].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.ustawienie_lewy_panel_szyfrowanie_podopcje, 0, RichText::new(self.current_language.szyfrowanie_naglowek).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                            });
                            columns[1].vertical_centered_justified(|ui|{
                                ui.selectable_value(&mut self.ustawienie_lewy_panel_szyfrowanie_podopcje, 1, RichText::new(self.current_language.deszyfrowanie_naglowek).font(wybrana_aktualna_czcionka(self.formatowanie_rozmiar_czcionki_srednia,self.wybor_czcionki)));
                            });
                        });
                }
                ui.add_space(self.formatowanie_spacja_srednia);

                egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    // if !self.ustawienie_lewy_panel_foty_szyfrowanie{


                        match (
                            self.ustawienie_lewy_panel_foty_szyfrowanie,
                            self.ustawienie_lewy_panel_szyfrowanie_podopcje,
                            self.ustawienie_lewy_panel_foty_podopcje
                        ){
                            (false,0,_) =>{ui_left_panel_encrypt(self,ctx,ui,fiolet_ciemny,zolty_ciemny,szarawy_ciemny);},
                            (false,1,_) =>{ui_left_panel_decrypt(self,ctx,ui,fiolet_ciemny,zolty_ciemny,szarawy_ciemny);},
                            (true,_,0) =>{ui_left_panel_foty_przetwarzanie(self,ctx,ui,zolty_ciemny,szarawy_ciemny);},
                            (true,_,1) =>{ui_left_panel_foty_laczenie(self,ctx,ui,zolty_ciemny,szarawy_ciemny);},
                            _ => {ui.label("coś sie skiepściło");}
                        }
                        
                                                                                        

            
                }); // zamknięcie scroll area
                
            }); //zamkniecie left panel


            // ----------------------------------------------------------------------------------------------------------------------------------------
            // ██████  ██  ██████  ██   ██ ████████      
            // ██   ██ ██ ██       ██   ██    ██         
            // ██████  ██ ██   ███ ███████    ██         
            // ██   ██ ██ ██    ██ ██   ██    ██         
            // ██   ██ ██  ██████  ██   ██    ██         
                                                      
                                                      
            // ██████   █████  ███    ██ ███████ ██      
            // ██   ██ ██   ██ ████   ██ ██      ██      
            // ██████  ███████ ██ ██  ██ █████   ██      
            // ██      ██   ██ ██  ██ ██ ██      ██      
            // ██      ██   ██ ██   ████ ███████ ███████

            egui::SidePanel::right("halp_menu")
            .exact_width(self.szerokosc_okna / 2.)
            .resizable(false)
            .show(ctx,|ui|{
                ui.add_space( self.formatowanie_spacja_srednia);
                ui.columns(if self.show_debug_labels{3}else{2},|columns|{


                    columns[0].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienia_menu, 0, self.current_language.general_ui_info);
                    });
                    columns[1].vertical_centered_justified(|ui|{
                        ui.selectable_value(&mut self.ustawienia_menu, 1, self.current_language.general_ui_ustawienia);
                    });
                    if self.show_debug_labels{
                        columns[2].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.ustawienia_menu, 2, "Debug");
                        });
                    }

                });

                ui.add_space(5.);

                ui.separator();
                if self.ustawienia_menu == 0{
                    ui.columns(2,|columns|{
                        columns[0].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 0, self.current_language.general_ui_info);
                        });
                        columns[1].vertical_centered_justified(|ui|{
                            ui.selectable_value(&mut self.toggle_halp, 1, self.current_language.general_ui_statystyki);
                        });
                        // columns[2].vertical_centered_justified(|ui|{
                        //     ui.selectable_value(&mut self.toggle_halp, 2, "pakowanie");
                        // });
                        // columns[3].vertical_centered_justified(|ui|{
                        //     ui.selectable_value(&mut self.toggle_halp, 3, "rozpakowywanie");
                        // });
                        // columns[4].vertical_centered_justified(|ui|{
                        //     ui.selectable_value(&mut self.toggle_halp, 4, "filtrowanie");
                        // });
                    });
                }

                egui::ScrollArea::vertical()
                .show(ui, |ui| {

                    match (self.ustawienia_menu,self.toggle_halp){

                        (0,0) => {right_panel_info_main(self,ctx,ui,fiolet_ciemny,zolty_ciemny,szarawy_ciemny);},
                        (0,1) => {right_panel_info_stats(self,ctx,ui,fiolet_ciemny,zolty_ciemny,szarawy_ciemny);},
                        (0,2) => {ui_right_panel_info_0_2(self,ctx,ui,zolty_ciemny,szarawy_ciemny);},
                        (0,3) => {ui_right_panel_info_0_3(self,ctx,ui,zolty_ciemny,szarawy_ciemny);},
                        (0,4) => {ui_right_panel_info_0_4(self,ctx,ui,zolty_ciemny,szarawy_ciemny);},
                        (1,_) => {ui_right_panel_settings_0(self,ctx,ui,fiolet_ciemny,zolty_ciemny,szarawy_ciemny);},
                        (2,_) => {ui_right_panel_debug_0(self,ctx,ui,fiolet_ciemny);},
                        _ => {ui.label("coś poszło nie tak");}
                        
                    }

                })
            });//zamkniecie right panel


        }); // zamknięcie central_panel
        if self.settings_toggle{
            egui::TopBottomPanel::top("settings").show(ctx, |ui| {
                ui.vertical_centered(|ui|{
                    ui.add(egui::Label::new(RichText::new(self.current_language.ustawienia_tytul.to_string()).size( self.formatowanie_rozmiar_czcionki_duza)).selectable(false));
            });
                ui.add(egui::Label::new(self.current_language.ustawienia_skrot.to_string()).selectable(false));
            });
        }    




    } // zamknięcie fn update

} // zamknięcie Appencja




