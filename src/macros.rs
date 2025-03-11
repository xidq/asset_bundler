// use egui::WidgetText::RichText;
// use crate::ui::ui_defaults::Appencja;


// just some macros to fill places in ui.

// definitely I need more macros!!!!!!!!!!!!!!!!!!!!!!!!!


//macro for big text in disabled label
#[macro_export]
macro_rules! dodaj_duży_label { //aka add big label
    ($tekst:expr) => {{
        let proxy_self = Appencja::default();

        egui::Label::new(
            RichText::new($tekst)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_duża,
                    proxy_self.formatowanie_wybór_czcionki,
                )),
        )
        .selectable(false)
    }};
}


// macro with medium text for disabled label
#[macro_export]
macro_rules! dodaj_średni_label { // aka add_medium_label
    ($tekst:expr) => {{
        let proxy_self = Appencja::default();

        egui::Label::new(
            RichText::new($tekst)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_średnia,
                    proxy_self.formatowanie_wybór_czcionki,
                )),
        )
        .selectable(false)
    }};
}


// nah, maybe later
#[macro_export]
macro_rules! dodaj_selectable_val_tekst { 
    ($target:expr, $value:expr, $tekst:expr) => {{

        $target = $value;

        egui::Label::new(
            RichText::new($tekst)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_średnia,
                    proxy_self.formatowanie_wybór_czcionki,
                )),
        )
        .selectable(false)
    }};
}


// macro for medium size text
#[macro_export]
macro_rules! dodaj_średni_richtext { //aka add_medium_RichText
    ($tekst:expr) => {{
        let proxy_self = Appencja::default();


            RichText::new($tekst)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_średnia,
                    proxy_self.formatowanie_wybór_czcionki,
                ))

    }}
}


// macro for big size text
#[macro_export]
macro_rules! dodaj_duży_richtext {  //aka add_big_RichText
    ($tekst:expr) => {{
        let proxy_self = Appencja::default();


            RichText::new($tekst)
                .font(wybrana_aktualna_czcionka(
                    proxy_self.formatowanie_rozmiar_czcionki_duża,
                    proxy_self.formatowanie_wybór_czcionki,
                ))

    }}
}