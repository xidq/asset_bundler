// use egui::WidgetText::RichText;
use crate::ui::ui_change_font::wybrana_aktualna_czcionka;
use crate::ui::ui_defaults::Appencja;
use egui::RichText;

pub fn dodaj_duży_label(tekst:&str) -> egui::Label {
    let proxy_self = &mut Appencja::default() ;
    egui::Label::new(RichText::new(tekst).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza, proxy_self.formatowanie_wybor_czcionki))).selectable(false)
}
pub fn dodaj_średni_label(tekst:&str) -> egui::Label {
    let proxy_self = &mut Appencja::default() ;
    egui::Label::new(RichText::new(tekst).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia, proxy_self.formatowanie_wybor_czcionki))).selectable(false)
}