use egui::{
    Color32,  
    Context, RichText, 
};
use crate::ui::ui_defaults::Appencja;
use crate::ui::ui_change_font::wybrana_aktualna_czcionka;



pub fn right_panel_info_stats(proxy_self: &mut Appencja,_ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32,_zolty_ciemny:Color32,_szarawy_ciemny:Color32){


    let tekst_plików=     match &proxy_self.przetworzone_pliki % 10 {
        1 if &proxy_self.przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.toggle_language == 1 => "file",
        2..=4 if (&proxy_self.przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.toggle_language == 0 => "plików",
        _ =>"files"
    };  
    let de_tekst_plików=
    match &proxy_self.de_przetworzone_pliki % 10 {
        1 if &proxy_self.de_przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.toggle_language == 1 => "file",
        2..=4 if (&proxy_self.de_przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.toggle_language == 0 => "plików",
        _ => "files",
    };  
    let foto_tekst_plików_utworzone=
    match &proxy_self.foto_utworzone_pliki % 10 {
        1 if &proxy_self.foto_utworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.toggle_language == 1 => "file",
        2..=4 if (&proxy_self.foto_utworzone_pliki % 100 < 10 || &proxy_self.foto_utworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.toggle_language == 0 => "plików",
        _ => "files",
    };  
    let foto_tekst_plików_przetworzone=
    match &proxy_self.foto_przetworzone_pliki % 10 {
        1 if &proxy_self.foto_przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.toggle_language == 1 => "file",
        2..=4 if (&proxy_self.foto_przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.toggle_language == 0 => "plików",
        _ => "files",
    };  
    // let tekst_plików=     match &proxy_self.przetworzone_pliki % 10 {
    //     1 if &proxy_self.przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0  => "plik", // wyjątek dla 11
    //     1 if proxy_self.toggle_language == 1 => "file",
    //     2..=4 if (&proxy_self.przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.toggle_language == 0 => "plików",
    //     _ => "files",
    // };  
    // let de_tekst_plików=
    // match &proxy_self.de_przetworzone_pliki % 10 {
    //     1 if &proxy_self.de_przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.toggle_language == 1 => "file",
    //     2..=4 if (&proxy_self.de_przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.toggle_language == 0 => "plików",
    //     _ => "files",
    // };  
    // let foto_tekst_plików_utworzone=
    // match &proxy_self.foto_utworzone_pliki % 10 {
    //     1 if &proxy_self.foto_utworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.toggle_language == 1 => "file",
    //     2..=4 if (&proxy_self.foto_utworzone_pliki % 100 < 10 || &proxy_self.foto_utworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.toggle_language == 0 => "plików",
    //     _ => "files",
    // };  
    // let foto_tekst_plików_przetworzone=
    // match &proxy_self.foto_przetworzone_pliki % 10 {
    //     1 if &proxy_self.foto_przetworzone_pliki % 100 != 11 && proxy_self.toggle_language == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.toggle_language == 1 => "file",
    //     2..=4 if (&proxy_self.foto_przetworzone_pliki % 100 < 10 || &proxy_self.przetworzone_pliki % 100 > 20) && proxy_self.toggle_language == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.toggle_language == 0 => "plików",
    //     _ => "files",
    // };  
    // let tekst_plikow_en = match &proxy_self.przetworzone_pliki{
    //     1 => "file",
    //     _ => "files"
    // };
    // let tekst_plikow_de_en = match &proxy_self.de_przetworzone_pliki{
    //     1 => "file",
    //     _ => "files"
    // };
    // let jezyk_cos_tekst = match proxy_self.toggle_language{
    //     0 => tekst_plików,
    //     1 => tekst_plikow_en,
    //     _ => "overflow"
    // };
    // let jezyk_cos_tekst_de = match proxy_self.toggle_language{
    //     0 => de_tekst_plików,
    //     1 => tekst_plikow_de_en,
    //     _ => "overflow"
    // };
                            ui.separator();
                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                RichText::new(format!(
                                    "{} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_spakowano_tytul,
                                    proxy_self.przetworzone_pliki,

                                    tekst_plików, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.timestamp_sekundy, 
                                    &proxy_self.timestamp_milisekundy))
                                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                                    .selectable(false)
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                    RichText::new (format!("{} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_rozpakowano_tytul,
                                    proxy_self.de_przetworzone_pliki,
                                    de_tekst_plików, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.de_timestamp_sekundy, 
                                    &proxy_self.de_timestamp_milisekundy))
                                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                                    .selectable(false)
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_srednia);
                            ui.add(egui::Label::new(
                                RichText::new(
                                    format!("{} {} {} {} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_przetworzono_tytul,
                                    proxy_self.foto_utworzone_pliki,
                                    foto_tekst_plików_utworzone,
                                    proxy_self.current_language.general_ui_z_oryginalnych_tytul,
                                    proxy_self.foto_przetworzone_pliki,
                                    foto_tekst_plików_przetworzone, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.foto_timestamp_sekundy, 
                                    &proxy_self.foto_timestamp_milisekundy))
                                    .font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)))
                                    .selectable(false)
                                );

                            
                        
}