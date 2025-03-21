use egui::FontId;

pub fn wybrana_aktualna_czcionka(rozmiar: f32,wybór_czcionki: u8) -> FontId{
    let aktualna_czcionka_aaaa = match wybór_czcionki {
        1 => egui::FontFamily::Monospace,
        2 =>egui::FontFamily::Name("japanese_font".to_owned().into()),
        _ =>egui::FontFamily::Proportional
    };
    let rozmiarrr = match wybór_czcionki{
        1 => rozmiar - 3.5,
        _ => rozmiar
    };
    FontId{size:rozmiarrr,family:aktualna_czcionka_aaaa}
}