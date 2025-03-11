use egui::{
    Color32,  
    Context, RichText, 
};
#[cfg(debug_assertions)]
use std::time::{Instant,
                // Duration
};
use crate::{dodaj_średni_label/*, dodaj_średni_richtext*/};
use crate::ui::ui_defaults::Appencja;
use crate::ui::ui_change_font::wybrana_aktualna_czcionka;



pub fn right_panel_info_stats(proxy_self: &mut Appencja,_ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32,_zolty_ciemny:Color32,_szarawy_ciemny:Color32){


    let tekst_plików=     match &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 10 {
        1 if &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
        2..=4 if (&proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
        _ =>"files"
    };  
    let de_tekst_plików=
    match &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 10 {
        1 if &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
        2..=4 if (&proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
        _ => "files",
    };  
    let ui_konwersja_utworzone_pliki=
    match &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 10 {
        1 if &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
        2..=4 if (&proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 < 10 || &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
        _ => "files",
    };  
    let ui_konwersja_przetworzone_pliki=
    match &proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 10 {
        1 if &proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
        1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
        2..=4 if (&proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
        _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
        _ => "files",
    };  
    // let tekst_plików=     match &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 10 {
    //     1 if &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0  => "plik", // wyjątek dla 11
    //     1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
    //     2..=4 if (&proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
    //     _ => "files",
    // };  
    // let de_tekst_plików=
    // match &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 10 {
    //     1 if &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
    //     2..=4 if (&proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
    //     _ => "files",
    // };  
    // let ui_konwersja_utworzone_pliki=
    // match &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 10 {
    //     1 if &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
    //     2..=4 if (&proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 < 10 || &proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
    //     _ => "files",
    // };  
    // let ui_konwersja_przetworzone_pliki=
    // match &proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 10 {
    //     1 if &proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 100 != 11 && proxy_self.general_ui_przelacz_jezyk == 0 => "plik", // wyjątek dla 11
    //     1 if proxy_self.general_ui_przelacz_jezyk == 1 => "file",
    //     2..=4 if (&proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki % 100 < 10 || &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki % 100 > 20) && proxy_self.general_ui_przelacz_jezyk == 0 => "pliki", // wyjątek dla 11-14, 21-24, itd.
    //     _ if proxy_self.general_ui_przelacz_jezyk == 0 => "plików",
    //     _ => "files",
    // };  
    // let tekst_plikow_en = match &proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki{
    //     1 => "file",
    //     _ => "files"
    // };
    // let tekst_plikow_de_en = match &proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki{
    //     1 => "file",
    //     _ => "files"
    // };
    // let jezyk_cos_tekst = match proxy_self.general_ui_przelacz_jezyk{
    //     0 => tekst_plików,
    //     1 => tekst_plikow_en,
    //     _ => "overflow"
    // };
    // let jezyk_cos_tekst_de = match proxy_self.general_ui_przelacz_jezyk{
    //     0 => de_tekst_plików,
    //     1 => tekst_plikow_de_en,
    //     _ => "overflow"
    // };
                            ui.separator();
                            ui.add_space( proxy_self.formatowanie_spacja_średnia);
                            ui.add(dodaj_średni_label!(format!(
                                    "{} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_spakowano_tytul,
                                    proxy_self.ui_pack_specyfic_statystyki_przetworzone_pliki,

                                    tekst_plików, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.ui_pack_specyfic_statystyki_czas_sekundy, 
                                    &proxy_self.ui_pack_specyfic_statystyki_czas_milisekundy))
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_średnia);
                            ui.add(dodaj_średni_label!(format!("{} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_rozpakowano_tytul,
                                    proxy_self.ui_unpack_specyfic_statystyki_przetworzone_pliki,
                                    de_tekst_plików, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.ui_unpack_specyfic_statystyki_czas_sekundy, 
                                    &proxy_self.ui_unpack_specyfic_statystyki_czas_milisekundy))
                                );

                            ui.add_space( proxy_self.formatowanie_spacja_średnia);
                            ui.add(dodaj_średni_label!(
                                    format!("{} {} {} {} {} {} {}: {}.{}s",
                                    proxy_self.current_language.general_ui_przetworzono_tytul,
                                    proxy_self.ui_konwersja_specyfic_statystyki_utworzone_pliki,
                                    ui_konwersja_utworzone_pliki,
                                    proxy_self.current_language.general_ui_z_oryginalnych_tytul,
                                    proxy_self.ui_konwersja_specyfic_statystyki_przetworzone_pliki,
                                    ui_konwersja_przetworzone_pliki, 
                                    proxy_self.current_language.general_ui_w_czasie_tytul,
                                    proxy_self.ui_konwersja_specyfic_statystyki_czas_sekundy, 
                                    &proxy_self.ui_konwersja_specyfic_statystyki_czas_sekundy))
                                );

                                #[cfg(debug_assertions)]
                                if proxy_self.general_ui_przelacz_tryb_debug{



                                    static mut LAST_FRAME: Option<Instant> = None;
                                    unsafe {
                                        let now = Instant::now();
                                        // Jeżeli to nie jest pierwsza klatka, oblicz czas od poprzedniego renderowania
                                        if let Some(last_frame) = LAST_FRAME {

                                            let frame_duration = now.duration_since(last_frame);
                                            match proxy_self.general_ui_licznik_czasu_debug{
                                                0=>{proxy_self.general_ui_statystyki_klatki.push((frame_duration.as_millis(),frame_duration.as_nanos()));},
                                                _=>{}
                                            };

                                            if proxy_self.general_ui_statystyki_klatki.len() > 600 {
                                                proxy_self.general_ui_statystyki_klatki.remove(0);
                                            }
                                            fn dostan_średnia(xoxo:&Vec<(u128,u128)>)->(u128,u128){
                                                let suma_milisekund: u128 = xoxo.iter().map(|(milli, _)| milli).sum();
                                                let suma_nanosekund: u128 = xoxo.iter().map(|(_, nano)| nano).sum();
                                    
                                                // Zwracamy średnią w milisekundach i nanosekundach
                                                let średnia_milisekund = if suma_milisekund>0{suma_milisekund / xoxo.len() as u128}else{000 as u128};
                                                let średnia_nanosekund = if suma_nanosekund>0{suma_nanosekund / xoxo.len() as u128}else{000 as u128};
                                                let ddddddd = średnia_nanosekund / 10000;
                                    
                                                (średnia_milisekund, ddddddd)
                                            }
                                            let fdgdgf:u128= 1;
                                            let sfddfgff:&u128 = if proxy_self.general_ui_statystyki_klatki.len()>1{proxy_self.general_ui_statystyki_klatki.iter().map(|(milli, _)| milli).min().unwrap()}else{&fdgdgf};
                                            let sfddfgfdf:&u128 = if proxy_self.general_ui_statystyki_klatki.len()>1{proxy_self.general_ui_statystyki_klatki.iter().map(|(_, nano)| nano).min().unwrap()}else{&fdgdgf};
                                            let aaaa:&u128 = if proxy_self.general_ui_statystyki_klatki.len()>1{proxy_self.general_ui_statystyki_klatki.iter().map(|(milli, _)| milli).max().unwrap()}else{&fdgdgf};
                                            let aaaab:&u128 = if proxy_self.general_ui_statystyki_klatki.len()>1{proxy_self.general_ui_statystyki_klatki.iter().map(|(_, nano)| nano).max().unwrap()}else{&fdgdgf};
                                            let (czas_mili,czas_nano)=dostan_średnia(&proxy_self.general_ui_statystyki_klatki);
                                            ui.label(format!("minimalny czas {}.{}ms",sfddfgff,sfddfgfdf));
                                            ui.label(format!("średni czas {}.{}ms",czas_mili,czas_nano));
                                            ui.label(format!("maksymalny czas {}.{}ms",aaaa,aaaab));

                                        }
                                
                                        // Zaktualizuj LAST_FRAME dla następnej klatki
                                        LAST_FRAME = Some(now);
                                    }
                                

                                    if proxy_self.general_ui_przelacz_binarny_zdjecia && proxy_self.general_ui_przelacz_zdjecia_opcje==0{
                                            ui.label(format!("arc_z_rozdzielczość: 16k {}, 8k {}, 4k {}, 2k {}, 1k {}, 512 {}, 256 {}, 128 {}, 64 {}, 32 {}",
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_16k,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_8k,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_4k,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_2k,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_1k,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_512,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_256,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_128,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_64,
                                            proxy_self.ui_konwersja_specyfic_dane_rozdzielczość_32
                                            ));
                                
                                
                                        let fdghdkh=[
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_jpg,"jpg "),
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_png,"png "),
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_png_16,"png_16 "),
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossless,"webp "),
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_webp_lossy,"webp_lossy "),
                                            (proxy_self.ui_konwersja_specyfic_dane_bool_tga,"tga"),
                                        ];
                                        let mut wybrane_rozszerzenia_tekst = String::new();
                                        for (xxx,xxy) in fdghdkh{
                                
                                            if xxx{
                                                wybrane_rozszerzenia_tekst.push_str(xxy);
                                                wybrane_rozszerzenia_tekst.push(',');
                                            }
                                
                                        }
                                
                                        ui.label(format!("arc_z_rozszerzenie: {}",
                                            wybrane_rozszerzenia_tekst
                                        ));
                                        ui.label(format!("arc_z_jakość: jpg {}, png {}, png_16 {}, webp {}, webp_lossy {}, tga {}",
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_jpg,
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_png,
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_png,
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossless,
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_webp_lossy,
                                            proxy_self.ui_konwersja_specyfic_dane_jakosc_tga
                                        ));
                                        ui.label(format!("arc_z_danymi: filter {}, alpha {}, alpha kolor {}, png_filter {}",
                                            proxy_self.ui_konwersja_specyfic_dane_filter,
                                            proxy_self.ui_konwersja_specyfic_dane_alpha,
                                            proxy_self.ui_konwersja_specyfic_dane_alpha_kolor,
                                            proxy_self.ui_konwersja_specyfic_dane_filter_png,
                                        ));

                                
                                    }
                                }

                            
                        
}