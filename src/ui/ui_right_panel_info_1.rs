use egui::{
    Color32,  
    Context, 
};
use crate::ui::ui_defaults::Appencja;



pub fn right_panel_info_stats(proxy_self: &mut Appencja,_ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32,_zolty_ciemny:Color32,_szarawy_ciemny:Color32){


    let tekst_plików=
                                match &proxy_self.przetworzone_pliki % 10 {
                                    1 if &proxy_self.przetworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&proxy_self.przetworzone_pliki % 100 < 10) || (&proxy_self.przetworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                                let de_tekst_plików=
                                match &proxy_self.de_przetworzone_pliki % 10 {
                                    1 if &proxy_self.de_przetworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&proxy_self.de_przetworzone_pliki % 100 < 10) || (&proxy_self.przetworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                                let foto_tekst_plików_utworzone=
                                match &proxy_self.foto_utworzone_pliki % 10 {
                                    1 if &proxy_self.foto_utworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&proxy_self.foto_utworzone_pliki % 100 < 10) || (&proxy_self.foto_utworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                                let foto_tekst_plików_przetworzone=
                                match &proxy_self.foto_przetworzone_pliki % 10 {
                                    1 if &proxy_self.foto_przetworzone_pliki % 100 != 11 => "plik", // wyjątek dla 11
                                    2 | 3 | 4 if (&proxy_self.foto_przetworzone_pliki % 100 < 10) || (&proxy_self.przetworzone_pliki % 100 > 20) => "pliki", // wyjątek dla 11-14, 21-24, itd.
                                    _ => "plików",
                                };  
                            ui.separator();
                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                format!(
                                    "Spakowano {} {} w czasie: {}.{}s",
                                    proxy_self.przetworzone_pliki,tekst_plików, 
                                    proxy_self.timestamp_sekundy, 
                                    &proxy_self.timestamp_milisekundy))
                                    .selectable(false)
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                format!(
                                    "Rozpakowano {} {} w czasie: {}.{}s",
                                    proxy_self.de_przetworzone_pliki,de_tekst_plików, 
                                    proxy_self.de_timestamp_sekundy, 
                                    &proxy_self.de_timestamp_milisekundy))
                                    .selectable(false)
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                format!(
                                    "Utworzono {} {} z oryginalnych {} {} w czasie: {}.{}s",
                                    proxy_self.foto_utworzone_pliki,foto_tekst_plików_utworzone,
                                    proxy_self.foto_przetworzone_pliki,
                                    foto_tekst_plików_przetworzone, 
                                    proxy_self.foto_timestamp_sekundy, 
                                    &proxy_self.foto_timestamp_milisekundy))
                                    .selectable(false)
                                );

                            
                        
}