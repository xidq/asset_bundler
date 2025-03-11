#[cfg(debug_assertions)]
use egui::{
    Color32,
    Context
};
#[cfg(debug_assertions)]
use crate::ui::ui_defaults::Appencja;
#[cfg(debug_assertions)]
pub fn ui_right_panel_info_0_4(
    proxy_self: &mut Appencja,
    _ctx: &Context,ui: &mut egui::Ui,
    _zolty_ciemny:Color32,
    _szarawy_ciemny:Color32
){ ui.add(egui::Label::new("Brak:").selectable(false));
ui.add(egui::Label::new("wszystkie pliki są pakowane").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_średnia);
ui.add(egui::Label::new("Assets:").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("rodzaj pliku, nazwa, wariant, podwariant, ID, ścieżka, file_size, offset").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("rodzaj pliku - generowane na podstawie rozszerzenia").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("nazwa - prefiks w nazwie pliku, określa ogólną nazwę assetu").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("wariant - wartość pomiędzy _X_ po nazwie, określa wariant").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("podwariant - wartość pomiędzy _X_ po wariancie, określa podwariant").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("ID - unikalne id generowane dla każdego pliku").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("ścieżka - oryginalna lokalna ścieżka do pliku wraz z nazwą").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("file_size - rozmiar danego pliku").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("offset - przesunięcie początku pliku").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("aktualne rodzaje plików:").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("Model_3D").selectable(false));
ui.add(egui::Label::new("glb | obj | 3mf | fbx | stl | dae | ply | x3d | 3ds | max | usd | usdz | c4d | ac | vmd | lwo | abc").selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("Image").selectable(false));
ui.add(egui::Label::new("jpg | png  | exr  | tga  | dds | ico | tiff | bmp  | jpeg | gif | tif | webp | heif | heic | raw" ).selectable(false));
ui.add_space( proxy_self.formatowanie_spacja_mala);
ui.add(egui::Label::new("DOKOŃCZYĆ!").selectable(false));

}