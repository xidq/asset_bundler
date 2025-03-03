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
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka
};


pub fn ui_right_panel_settings_0(proxy_self: &mut Appencja,ctx: &Context,ui: &mut egui::Ui, fiolet_ciemny:Color32,zolty_ciemny:Color32,szarawy_ciemny:Color32){
    ui.add_space(10.);
    ui.vertical_centered_justified(|ui|{
        ui.checkbox(&mut proxy_self.debug_create_lua_file, RichText::new(&proxy_self.current_language.create_lua_file.to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki)));
    });

    ui.add_space( proxy_self.formatowanie_spacja_srednia);

    ui.columns(2,|columns|{
        columns[0].vertical_centered_justified(|ui|{
            ui.add(egui::Label::new(RichText::new("Głośność").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
        });
        columns[1].vertical_centered_justified(|ui|{
            ui.add(egui::Slider::new(&mut proxy_self.ustawienia_glosnosc, 0f32..=1.).text(""));
        });
    });
    ui.columns(2,|columns|{
        columns[0].vertical_centered_justified(|ui|{
            ui.add(egui::Label::new(RichText::new("mala czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
        });
        columns[1].vertical_centered_justified(|ui|{
            ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_mala,4f32..=12f32));
        });
    });
    ui.columns(2,|columns|{
        columns[0].vertical_centered_justified(|ui|{
            ui.add(egui::Label::new(RichText::new("srednia czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
        });
        columns[1].vertical_centered_justified(|ui|{
            ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_srednia,6f32..=18f32));
        });
    });
    ui.columns(2,|columns|{
        columns[0].vertical_centered_justified(|ui|{
            ui.add(egui::Label::new(RichText::new("duza czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
        });
        columns[1].vertical_centered_justified(|ui|{
            ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_duza,8f32..=24f32));
        });
    });

}
// ui.add(egui::Label::new(RichText::new("mala czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
// ui.add(egui::Label::new(RichText::new("srednia czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));
// ui.add(egui::Label::new(RichText::new("duza czcionka").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.wybor_czcionki))).selectable(false));


// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_mala,4f32..=12f32));
// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_srednia,6f32..=18f32));
// ui.add(egui::Slider::new(&mut proxy_self.formatowanie_rozmiar_czcionki_duza,8f32..=24f32));