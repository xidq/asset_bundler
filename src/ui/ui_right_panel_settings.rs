use egui::{
    Color32,  
    RichText, 
    Context, 
};
use crate::{dodaj_średni_label, dodaj_średni_richtext};
use crate::ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka
};


pub fn ui_right_panel_settings_0(
    proxy_self: &mut Appencja,
    _ctx: &Context,ui:
    &mut egui::Ui,
    _fiolet_ciemny:Color32,
    _zolty_ciemny:Color32,
    _szarawy_ciemny:Color32
){
    ui.add_space(10.);
    ui.vertical_centered_justified(|ui|{
        ui.checkbox(&mut proxy_self.ui_ustawienia_tworzenie_pliku_lua,
                    dodaj_średni_richtext!(proxy_self.current_language.create_lua_file.to_string()));
    });

    ui.add_space( proxy_self.formatowanie_spacja_średnia);
    #[cfg(not(feature = "raw"))]
    ui.columns(2,|columns|{
        columns[0].vertical_centered_justified(|ui|{
            ui.add(dodaj_średni_label!(proxy_self.current_language.general_ui_glosnosc));
        });
        columns[1].vertical_centered_justified(|ui|{
            ui.add(egui::Slider::new(&mut proxy_self.ui_ustawienia_glosnosc, 0f32..=1.));
        });
    });


}
// ui.add(dodaj__label!("mala czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_średnia,proxy_self.formatowanie_wybór_czcionki))).selectable(false));
// ui.add(dodaj__label!("średnia czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_średnia,proxy_self.formatowanie_wybór_czcionki))).selectable(false));
// ui.add(dodaj__label!("duża czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_średnia,proxy_self.formatowanie_wybór_czcionki))).selectable(false));


// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_mala,4f32..=12f32));
// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_średnia,6f32..=18f32));
// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_duża,8f32..=24f32));