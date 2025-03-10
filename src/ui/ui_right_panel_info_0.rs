use egui::{
    Color32,  
    RichText, 
    Context, 
};
use crate::ui::{
    ui_defaults::Appencja,
    ui_change_font::wybrana_aktualna_czcionka
};
use webbrowser::open;



pub fn right_panel_info_main(proxy_self: &mut Appencja,_ctx: &Context,ui: &mut egui::Ui, _fiolet_ciemny:Color32,_zolty_ciemny:Color32,_szarawy_ciemny:Color32){
    ui.separator();
    #[cfg(feature = "statystyki")]
    ui.add(egui::Label::new(RichText::new("CTRL + E for aditional menu").font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))).selectable(false));
    #[cfg(feature = "statystyki")]
    ui.add(egui::Label::new(RichText::new("wiadomość TODO!".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))));
    ui.add(egui::Label::new(RichText::new("do konwersji zdjęć obsługiwane na ten moment:\n jpg | jpeg | png | webp | tga ".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))));
    ui.add(egui::Label::new(RichText::new("może działać ale nie musi:\n inne rozszerzenia zdjęć".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))));
    ui.add(egui::Label::new(RichText::new("Konwersja zdjęć działa wielowątkowo".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))));
    ui.add(egui::Label::new(RichText::new("Tworzone są w podfolderach jak drzewno oryginalnego folderu".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_srednia,proxy_self.formatowanie_wybor_czcionki))));
    ui.add(egui::Label::new(RichText::new("Rozdzielczoś jest ustawiana dla dłuższej krawędzi".to_string()).font(wybrana_aktualna_czcionka(proxy_self.formatowanie_rozmiar_czcionki_duza,proxy_self.formatowanie_wybor_czcionki))));
 
    // // Wyświetlanie tekstu z aktualnym kolorem
    // ui.add(egui::Label::new(if proxy_self.nie_krzycz_iter<5{
    //     RichText::new("(już nie) ZACINA PROGRAM(u) PODCZAS WYKONYWANIA FUNKCJI, \nTAK MA BYĆ! \nBĘDZIE ZMIANA JAK BĘDZIE POTRZEBA, \nNA RAZIE DZIAŁA, \nA JAK DZIAŁA TO NIE TYKAĆ!!!!!")
    // }else{
    //     RichText::new("Программа зависает во время выполнения функции, \nТак и должно быть! \nБудет изменение, если потребуется, \nПока работает, \nА если работает, не трогать!!!!!")
    //     // RichText::new("プログラムは関数の実行中にフリーズします、\nそれで問題ありません！\n必要な場合に変更がありますが、\n現時点では動作しています、\n動作しているなら触らないでください！！！").font(egui::FontId::new(16.0, egui::FontFamily::Name("japanese_font".to_owned().into())))
    // }
    //     .size(cd * proxy_self.nie_krzycz_zmniejszacz)
    //     .color(kolor_komunikatu))
    //     .selectable(false));
    
    // // Przycisk, który zmienia kolor po kliknięciu
    // if ui.button("Rozumim, nie krzyczaj").clicked() {
    //     // Zmiana koloru po kliknięciu przycisku
    //     proxy_self.toggle_halp_komunikat = true;
    //     proxy_self.nie_krzycz_zmniejszacz = proxy_self.nie_krzycz_zmniejszacz * 0.9;
    //     proxy_self.nie_krzycz_iter -= 1
    // }
    // if ui.button("Nie rozumim, krzyczaj!").clicked() {
    //     // Zmiana koloru po kliknięciu przycisku
    //     proxy_self.toggle_halp_komunikat = false;
    //     proxy_self.nie_krzycz_zmniejszacz = proxy_self.nie_krzycz_zmniejszacz * 1.1;
    //     proxy_self.nie_krzycz_iter += 1
    // }
    #[cfg(not(feature = "raw"))]
    ui.add(egui::Image::new(egui::include_image!("../br/ezgif-762e334d60199c.gif"))
    .sense(egui::Sense::click())
    .max_height(330.0)
    .max_width(300.0)
    .maintain_aspect_ratio(true)
    .fit_to_original_size(1.)
    // .rounding(10.0));
    
    );

    ui.add_space(100.);
    ui.vertical_centered_justified(|ui|{
        if ui.link(RichText::new("GitHub").size(30.)).clicked() {
            if let Err(e) = open("https://github.com/xidq/asset_bundler") {
                eprintln!("Błąd przy otwieraniu linku: {}", e);
            }
        }
    });
}