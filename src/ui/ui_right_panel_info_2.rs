use egui::{
    Color32, 
    Pos2, 
    Response, 
    RichText,
    Rect,
    Vec2,
    Context
};
use rfd::FileDialog;
use std::sync::{
    Arc, 
    Mutex
};
use crate::ui::{
    ui_defaults::Appencja,
    ui_play_sound::play_finish_sound
};

pub fn ui_right_panel_info_0_2(
    proxy_self: &mut Appencja,
    ctx: &Context,ui: &mut egui::Ui,
    zolty_ciemny:Color32,
    szarawy_ciemny:Color32
){                            ui.add(egui::Label::new("Lorem Ipsum, myślisz, że robisz wrażenie,
Wszystko to bełkot, zero treści, to tylko pozorne istnienie.
Kopiujesz, wklejasz, ale gdzie jest sens?
Puste słowa jak piosenka, której nie chcesz słyszeć więcej.

Nie ma tu nic oryginalnego, tylko ślepa repetetycja,
Cały twój przekaz to sztuczna projekcja.
Więc nie bądź dumny, bo w świecie tekstów masz już dość,
Jak pusty filler – tylko niepotrzebny hałas wśród treści, która ma moc!”").selectable(false));}