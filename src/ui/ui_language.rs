

// ██╗      █████╗ ███╗   ██╗ ██████╗ ██╗   ██╗ █████╗  ██████╗ ███████╗
// ██║     ██╔══██╗████╗  ██║██╔════╝ ██║   ██║██╔══██╗██╔════╝ ██╔════╝
// ██║     ███████║██╔██╗ ██║██║  ███╗██║   ██║███████║██║  ███╗█████╗  
// ██║     ██╔══██║██║╚██╗██║██║   ██║██║   ██║██╔══██║██║   ██║██╔══╝  
// ███████╗██║  ██║██║ ╚████║╚██████╔╝╚██████╔╝██║  ██║╚██████╔╝███████╗
// ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝



pub struct Language {
    //ui general
    pub general_ui_label_binary: &'static str,
    pub general_ui_label_images: &'static str,
    pub general_ui_label_brak: &'static str,

    //szablony
    pub szablony_wybór_0: &'static str,
    pub szablony_wybór_1: &'static str,
    pub szablony_wybór_2: &'static str,
    pub szablony_wybór_3: &'static str,
    pub szablony_wybór_4: &'static str,
    pub szablony_wybór_5: &'static str,
    pub szablony_wybór_6: &'static str,





    //ui szyfrowanie
    pub szyfrowanie_naglowek: &'static str,
    pub general_ui_nazwa_tytul: &'static str,
    pub general_ui_nazwa: &'static str,
    pub general_ui_kompresja_tekst: &'static str,
    pub general_ui_kompresja_tytul: &'static str,
    pub szyfrowanie_kompresja: &'static str,
    pub general_ui_haslo_tytul: &'static str,
    pub general_ui_haslo: &'static str,
    pub general_ui_haslo_wylaczone: &'static str,
    pub szyfrowanie_szyfrowanie_tytul: &'static str,
    pub general_ui_szablony_tytul: &'static str,
    pub general_ui_info: &'static str,
    pub general_ui_ustawienia: &'static str,
    pub general_ui_statystyki: &'static str,
    pub general_ui_glosnosc: &'static str,

    pub szyfrowanie_przycisk_ok: &'static str,
    pub szyfrowanie_przycisk_nie_ok: &'static str,
    pub szyfrowanie_przycisk_koniec: &'static str,

    // pub szyfrowanie_przysick_2: &'static str,
    pub szyfrowanie_przycisk_3: &'static str,
    pub szyfrowanie_przycisk_4: &'static str,

    pub deszyfrowanie_naglowek: &'static str,
    pub general_ui_wybierz_plik_dat: &'static str,
    pub general_ui_wybierz_plik_idx: &'static str,
    pub deszyfrowanie_przycisk_ok: &'static str,


    pub general_ui_wybierz_folder_wejściowy: &'static str,
    pub general_ui_wybierz_folder_wyjściowy: &'static str,
    pub general_ui_wybierz_folder: &'static str,
    pub general_ui_wybierz_plik_general: &'static str,

    // pub debug_deszyfracja_idx: &'static str,
    pub przetwarzanie_bez_alpha: &'static str,
    pub przetwarzanie_z_alpha: &'static str,
    
    // pub h_general_ui_wybierz_folder: &'static str,
    pub ukryte_sktory: &'static str,
    pub create_lua_file: &'static str,

    pub przetwarzanie_naglowek: &'static str,
    pub general_ui_alpha_tytul: &'static str,
    pub general_ui_filter_png_tytul: &'static str,
    pub general_ui_warianty_tytul: &'static str,
    pub general_ui_rozdzielczość_tytul: &'static str,
    pub general_ui_rozszerzenie_tytul: &'static str,
    pub general_ui_rozpakowano_tytul: &'static str,
    pub general_ui_spakowano_tytul: &'static str,
    pub general_ui_przetworzono_tytul: &'static str,
    // pub general_ui_z_ekspresja: &'static str,
    pub general_ui_w_czasie_tytul: &'static str,
    pub general_ui_z_oryginalnych_tytul:&'static str,
    pub general_ui_kompilacja_rgb_tytul:&'static str,

    pub łączenie_statyczny_id: &'static str,


    pub err_value_overflow: &'static str,
    pub err_aspect_ratio: &'static str,

    //monit
    // pub problem_egzystencjonalny: &'static str,

    //USTAWIENIA!!!!!!!
    pub ustawienia_tytul:&'static str,
    pub ustawienia_skrot:&'static str,

    pub general_colors_white: &'static str,
    pub general_colors_black: &'static str,
    pub general_colors_red: &'static str,
    pub general_colors_green: &'static str,
    pub general_colors_blue: &'static str,


    pub png_specyfic_filter_png_none:&'static str,
    pub png_specyfic_filter_png_sub:&'static str,
    pub png_specyfic_filter_png_up:&'static str,
    pub png_specyfic_filter_png_avg:&'static str,
    pub png_specyfic_filter_png_paeth:&'static str,
    pub png_specyfic_filter_png_adaptive:&'static str,

    pub image_specyfic_filter_nearest:&'static str,
    pub image_specyfic_filter_triangle:&'static str,
    pub image_specyfic_filter_catmullrom:&'static str,
    pub image_specyfic_filter_gaussian:&'static str,
    pub image_specyfic_filter_lanczos3:&'static str,


    pub png_specyfic_compression_none:&'static str,
    pub png_specyfic_compression_default:&'static str,
    pub png_specyfic_compression_best:&'static str,




}


impl Language {

    pub fn polish() -> Self {

        Language {

            //ui general
            general_ui_label_binary: "Pliki Binarne",
            general_ui_label_images: "Obrazy",
            general_ui_label_brak:"Brak",

                //szablony
            szablony_wybór_0: "Wszystkie pliki",
            szablony_wybór_1: "Assets",
            szablony_wybór_2: "Zdjęcia",
            szablony_wybór_3: "Dźwięki",
            szablony_wybór_4: "Modele 3d",
            szablony_wybór_5: "Dokumenty",
            szablony_wybór_6: "Raw",

            szyfrowanie_naglowek: "Pakowanie",
            general_ui_nazwa_tytul: "Nazwa pliku",
            general_ui_nazwa: "Podaj nazwę",
            general_ui_kompresja_tytul: "Wybór kompresji",
            szyfrowanie_kompresja: "Zacność kompresji",
            general_ui_kompresja_tekst:"Kompresja",
            general_ui_haslo_tytul: "Hasło",
            general_ui_haslo: "Hasło Zgasło, wpisz inne!",
            general_ui_haslo_wylaczone: "Hasło wyłączone...",
            szyfrowanie_szyfrowanie_tytul:"Szyfrowanie",
            general_ui_szablony_tytul:"Szablony",
            general_ui_info: "Info",
            general_ui_ustawienia: "Ustawienia",
            general_ui_statystyki: "Statystyki",
            general_ui_glosnosc: "Głośność",
        

            szyfrowanie_przycisk_ok: "Szyfrufruj!",
            szyfrowanie_przycisk_nie_ok: "Czegoś ci jeszcze brakuje UwU",
            szyfrowanie_przycisk_koniec: "Yay, koniec!",

            // szyfrowanie_przysick_2: "Yay, koniec!",
            szyfrowanie_przycisk_3: "erroru",
            szyfrowanie_przycisk_4: "erroru",

            deszyfrowanie_naglowek: "Rozpakowywanie",
            general_ui_wybierz_plik_dat: "Plik .jrz/.jrzs",
            general_ui_wybierz_plik_idx: "Plik .idx",
            deszyfrowanie_przycisk_ok: "Deszyfrufruj!",

            general_ui_wybierz_folder_wejściowy: "Folder Wejściowy",
            general_ui_wybierz_folder_wyjściowy: "Folder Docelowy",
            general_ui_wybierz_folder: "Wybierz Folder",
            general_ui_wybierz_plik_general: "Wybierz Plik",

            // debug_deszyfracja_idx: "Deszyfracja zaszyfrowanego idx",

            // h_general_ui_wybierz_folder: "Wybierz folder",

            łączenie_statyczny_id:"Włącz statyczne ID",

            ukryte_sktory: "CTRL+R : przełącz szyfrowanie\nCTRL+B : brainroot (nie zaimplementowano)",

            create_lua_file: "Utwórz dodatkowy plik lua podczas szyfrowania",


            przetwarzanie_naglowek:"Przetwarzanie",
            general_ui_alpha_tytul: "Alpha",
            general_ui_filter_png_tytul: "Filtr",
            general_ui_warianty_tytul: "Warianty",
            general_ui_rozdzielczość_tytul: "Rozdzielczość",
            general_ui_rozszerzenie_tytul: "Rozszerzenie",

            general_ui_rozpakowano_tytul: "Rozpakowano",
            general_ui_spakowano_tytul: "Spakowano",
            general_ui_przetworzono_tytul: "Przetworzono",
            // general_ui_z_ekspresja: "z",
            general_ui_w_czasie_tytul: "w czasie",
            general_ui_z_oryginalnych_tytul: "z oryginalnych",
            general_ui_kompilacja_rgb_tytul: "Kompilacja RGB",





            err_value_overflow:"Poza limitem",
            err_aspect_ratio:"Błąd: aspect ratio: ",
            

            //monit
            // problem_egzystencjonalny: "Ma problemy egzystencjonalne...",


            //USTAWIENIA
            ustawienia_tytul: "Ustawienia",
            ustawienia_skrot:"Ctrl+S zamknij to okno",

            general_colors_white: "Biały",
            general_colors_black: "Czarny",
            general_colors_red: "Czerwony",
            general_colors_green: "Zielony",
            general_colors_blue: "Niebieski",

            przetwarzanie_bez_alpha:"bez Alpha",
            przetwarzanie_z_alpha:"z Alpha",

            png_specyfic_filter_png_none:"Żaden",
            png_specyfic_filter_png_sub:"Sub",
            png_specyfic_filter_png_up:"Up",
            png_specyfic_filter_png_avg:"Średni",
            png_specyfic_filter_png_paeth:"Paeth",
            png_specyfic_filter_png_adaptive:"Adaptacyjny",

            image_specyfic_filter_nearest:"Najbliższe",
            image_specyfic_filter_triangle:"Triangle",
            image_specyfic_filter_catmullrom:"CatmullRom",
            image_specyfic_filter_gaussian:"Gaussian",
            image_specyfic_filter_lanczos3:"Lanczos3",

            png_specyfic_compression_none:"Brak",
            png_specyfic_compression_default:"Domyślna",
            png_specyfic_compression_best:"Najlepsza",

        }

    }


    pub fn english() -> Self {
        Language {
            general_ui_label_binary: "Binary Files",
            general_ui_label_images: "Images",
            general_ui_label_brak:"None",

                //szablony
            szablony_wybór_0: "All files",
            szablony_wybór_1: "Assets",
            szablony_wybór_2: "Images",
            szablony_wybór_3: "Sounds",
            szablony_wybór_4: "3d models",
            szablony_wybór_5: "Documents",
            szablony_wybór_6: "Raw photos",

            szyfrowanie_naglowek: "Packing",
            general_ui_nazwa_tytul: "File name",
            general_ui_nazwa: "Name your file",
            general_ui_kompresja_tytul: "Compression method",
            szyfrowanie_kompresja: "Compression strength",
            general_ui_kompresja_tekst:"Compression",
            general_ui_haslo_tytul: "Password",
            general_ui_haslo: "Write smth...",
            general_ui_haslo_wylaczone: "Password is disabled.. Ya know?",

            szyfrowanie_szyfrowanie_tytul:"Encoding",
            general_ui_szablony_tytul:"Templates",

            szyfrowanie_przycisk_ok: "Pack!",
            szyfrowanie_przycisk_nie_ok: "Seems u forgor somting UwU",
            szyfrowanie_przycisk_koniec: "Process ended",

            // szyfrowanie_przysick_2: "Process ended",
            szyfrowanie_przycisk_3: "erroru",
            szyfrowanie_przycisk_4: "erroru",

            deszyfrowanie_naglowek: "Unpacking",
            general_ui_wybierz_plik_dat: "File .jrz/.jrzs",
            general_ui_wybierz_plik_idx: "File .idx",
            deszyfrowanie_przycisk_ok: "Decode!!!!!",


            general_ui_wybierz_folder_wejściowy: "Input Folder",
            general_ui_wybierz_folder_wyjściowy: "Output Folder",
            general_ui_wybierz_folder: "Choose directory",

            general_ui_wybierz_plik_general: "Choose file",


            łączenie_statyczny_id:"Enable static ID",
            // debug_deszyfracja_idx: "Decrypt .idx file only",

            // h_general_ui_wybierz_folder: "Choose your desired folder",

            ukryte_sktory:"CTRL+R : toogle encription\nCTRL+B for brainroot",

            create_lua_file: "Create additional lua file while encryption",

            przetwarzanie_naglowek:"Conversion",
            general_ui_alpha_tytul: "Alpha",
            general_ui_filter_png_tytul: "Filter",
            general_ui_warianty_tytul: "Variants",
            general_ui_rozdzielczość_tytul: "Resolution",
            general_ui_rozszerzenie_tytul: "Extension",
            general_ui_info: "Info",
            general_ui_ustawienia: "Settings",
            general_ui_statystyki: "Stats",
            general_ui_glosnosc: "Volume",
            general_ui_rozpakowano_tytul: "Unpacked",
            general_ui_spakowano_tytul: "Packed",
            general_ui_przetworzono_tytul: "Processed",
            general_ui_kompilacja_rgb_tytul:"RGB stacking",

            // general_ui_z_ekspresja: "from",
            general_ui_w_czasie_tytul: "in time",
            general_ui_z_oryginalnych_tytul: "from oryginal",

            //monit
            // problem_egzystencjonalny: "Have existencial crysis rn",

            // USTAWIENIA
            ustawienia_tytul: "Settings",
            ustawienia_skrot:"Ctrl+S close that window",

            general_colors_white: "White",
            general_colors_black: "Black",
            general_colors_red: "Red",
            general_colors_green: "Green",
            general_colors_blue: "Blue",
            przetwarzanie_bez_alpha:"Without Alpha channel",
            przetwarzanie_z_alpha:"With Alpha channel",

            err_value_overflow:"Value overflow",
            err_aspect_ratio:"Error: aspect ratio: ",



            png_specyfic_filter_png_none:"None",
            png_specyfic_filter_png_sub:"Sub",
            png_specyfic_filter_png_up:"Up",
            png_specyfic_filter_png_avg:"Average",
            png_specyfic_filter_png_paeth:"Paeth",
            png_specyfic_filter_png_adaptive:"Adaptive",

            image_specyfic_filter_nearest:"Nearest",
            image_specyfic_filter_triangle:"Triangle",
            image_specyfic_filter_catmullrom:"CatmullRom",
            image_specyfic_filter_gaussian:"Gaussian",
            image_specyfic_filter_lanczos3:"Lanczos3",

            png_specyfic_compression_none:"None",
            png_specyfic_compression_default:"Default",
            png_specyfic_compression_best:"Best",
        }

    }

    pub fn debug() -> Self {
        Language {
            general_ui_label_binary: "general_ui_label_binary",
            general_ui_label_images: "general_ui_label_images",
            general_ui_label_brak:"general_ui_label_brak",

            //szablony
            szablony_wybór_0: "szablony_wybór_0",
            szablony_wybór_1: "szablony_wybór_1",
            szablony_wybór_2: "szablony_wybór_2",
            szablony_wybór_3: "szablony_wybór_3",
            szablony_wybór_4: "szablony_wybór_4",
            szablony_wybór_5: "szablony_wybór_5",
            szablony_wybór_6: "szablony_wybór_6",

            szyfrowanie_naglowek: "szyfrowanie_naglowek",
            general_ui_nazwa_tytul: "general_ui_nazwa_tytul",
            general_ui_nazwa: "general_ui_nazwa",
            general_ui_kompresja_tytul: "general_ui_kompresja_tytul",
            szyfrowanie_kompresja: "szyfrowanie_kompresja",
            general_ui_kompresja_tekst:"general_ui_kompresja_tekst",
            general_ui_haslo_tytul: "general_ui_haslo_tytul",
            general_ui_haslo: "general_ui_haslo",
            general_ui_haslo_wylaczone: "general_ui_haslo_wylaczone",
            szyfrowanie_szyfrowanie_tytul:"szyfrowanie_szyfrowanie_tytul",
            general_ui_szablony_tytul: "general_ui_szablony_tytul",

            szyfrowanie_przycisk_ok: "szyfrowanie_przycisk_ok",
            szyfrowanie_przycisk_nie_ok: "szyfrowanie_przycisk_nie_ok",
            szyfrowanie_przycisk_koniec: "szyfrowanie_przycisk_koniec",

            // szyfrowanie_przysick_2: "szyfrowanie_przysick_2",
            szyfrowanie_przycisk_3: "szyfrowanie_przycisk_3",
            szyfrowanie_przycisk_4: "szyfrowanie_przycisk_4",

            deszyfrowanie_naglowek: "deszyfrowanie_naglowek",
            general_ui_wybierz_plik_dat: "general_ui_wybierz_plik_dat",
            general_ui_wybierz_plik_idx: "general_ui_wybierz_plik_idx",
            deszyfrowanie_przycisk_ok: "deszyfrowanie_przycisk_ok",


            general_ui_wybierz_folder_wejściowy: "general_ui_wybierz_folder_wejściowy",
            general_ui_wybierz_folder_wyjściowy: "general_ui_wybierz_folder_wyjściowy",
            general_ui_wybierz_folder: "general_ui_wybierz_folder",

            general_ui_wybierz_plik_general: "general_ui_wybierz_plik_general",
            general_ui_rozpakowano_tytul: "general_ui_rozpakowano_tytul",
            general_ui_spakowano_tytul: "general_ui_rozpakowano_tytul",
            general_ui_przetworzono_tytul: "general_ui_rozpakowano_tytul",

            // general_ui_z_ekspresja: "general_ui_z_ekspresja",
            general_ui_w_czasie_tytul: "general_ui_w_czasie_tytul",
            general_ui_z_oryginalnych_tytul: "general_ui_z_oryginalnych_tytul",
            łączenie_statyczny_id: "łączenie_statyczny_id",

            // debug_deszyfracja_idx: "debug_deszyfracja_idx",

            // h_general_ui_wybierz_folder: "h_general_ui_wybierz_folder",

            ukryte_sktory:"ukryte_sktory",

            create_lua_file: "create_lua_file",

            przetwarzanie_naglowek:"przetwarzanie_naglowek",
            przetwarzanie_bez_alpha:"przetwarzanie_bez_alpha",
            przetwarzanie_z_alpha:"przetwarzanie_z_alpha",
            general_ui_alpha_tytul: "general_ui_alpha_tytul",
            general_ui_filter_png_tytul: "general_ui_filter_png_tytul",
            general_ui_warianty_tytul: "general_ui_warianty_tytul",
            general_ui_rozdzielczość_tytul: "general_ui_rozdzielczość_tytul",
            general_ui_rozszerzenie_tytul: "general_ui_rozszerzenie_tytul",
            general_ui_info: "general_ui_info",
            general_ui_ustawienia: "general_ui_ustawienia",
            general_ui_statystyki: "general_ui_statystyki",
            general_ui_glosnosc: "general_ui_glosnosc",
            general_ui_kompilacja_rgb_tytul:"general_ui_kompilacja_rgb_tytul",




            err_value_overflow:"err_value_overflow",
            err_aspect_ratio:"err_aspect_ratio",


            //monit
            // problem_egzystencjonalny: "problem_egzystencjonalny",

            // USTAWIENIA
            ustawienia_tytul: "ustawienia_tytul",
            ustawienia_skrot:"ustawienia_skrot",

            general_colors_white: "general_colors_white",
            general_colors_black: "general_colors_black",
            general_colors_red: "general_colors_red",
            general_colors_green: "general_colors_green",
            general_colors_blue: "general_colors_blue",


            png_specyfic_filter_png_none:"png_specyfic_filter_png_none",
            png_specyfic_filter_png_sub:"png_specyfic_filter_png_sub",
            png_specyfic_filter_png_up:"png_specyfic_filter_png_up",
            png_specyfic_filter_png_avg:"png_specyfic_filter_png_avg",
            png_specyfic_filter_png_paeth:"png_specyfic_filter_png_paeth",
            png_specyfic_filter_png_adaptive:"png_specyfic_filter_png_adaptive",

            image_specyfic_filter_nearest:"image_specyfic_filter_nearest",
            image_specyfic_filter_triangle:"image_specyfic_filter_triangle",
            image_specyfic_filter_catmullrom:"image_specyfic_filter_catmullrom",
            image_specyfic_filter_gaussian:"image_specyfic_filter_gaussian",
            image_specyfic_filter_lanczos3:"image_specyfic_filter_lanczos3",

            png_specyfic_compression_none:"png_specyfic_compression_none",
            png_specyfic_compression_default:"png_specyfic_compression_default",
            png_specyfic_compression_best:"png_specyfic_compression_best",

            
        }

    }
    // pub fn japanese() -> Self {
    //     Language {
    //         general_ui_label_binary: "バイナリファイル",
    //         general_ui_label_images: "画像",
    //         general_ui_label_brak: "なし",
    
    //         // szablony
    //         szablony_wybór_0: "すべてのファイル",
    //         szablony_wybór_1: "アセット",
    //         szablony_wybór_2: "画像",
    //         szablony_wybór_3: "音声",
    //         szablony_wybór_4: "3D モデル",
    //         szablony_wybór_5: "文書",
    //         szablony_wybór_6: "生写真",
    
    //         szyfrowanie_naglowek: "圧縮",
    //         general_ui_nazwa_tytul: "ファイル名",
    //         general_ui_nazwa: "ファイルに名前を付けてください",
    //         general_ui_kompresja_tytul: "圧縮方法",
    //         szyfrowanie_kompresja: "圧縮強度",
    //         general_ui_haslo_tytul: "パスワード",
    //         general_ui_haslo: "何かを書いてください...",
    //         general_ui_haslo_wylaczone: "パスワードは無効です.. わかりますか？",
    
    //         szyfrowanie_szyfrowanie_tytul: "エンコーディング",
    //         general_ui_szablony_tytul: "テンプレート",
    
    //         szyfrowanie_przycisk_ok: "圧縮開始！",
    //         szyfrowanie_przycisk_nie_ok: "何かを忘れたようです UwU",
    //         szyfrowanie_przycisk_koniec: "処理終了",
    
    //         // szyfrowanie_przysick_2: "処理終了",
    //         szyfrowanie_przycisk_3: "エラー",
    //         szyfrowanie_przycisk_4: "エラー",
    
    //         deszyfrowanie_naglowek: "解凍",
    //         general_ui_wybierz_plik_dat: "ファイル .jrz/.jrzs",
    //         general_ui_wybierz_plik_idx: "ファイル .idx",
    //         deszyfrowanie_przycisk_ok: "デコード開始！！！",
    
    //         general_ui_wybierz_folder_wejściowy: "入力フォルダ",
    //         general_ui_wybierz_folder_wyjściowy: "出力フォルダ",
    //         general_ui_wybierz_folder: "ディレクトリを選択",
    
    //         debug_deszyfracja_idx: ".idxファイルのみを復号化",
    
    //         // h_general_ui_wybierz_folder: "希望するフォルダを選択",
    
    //         ukryte_sktory: "CTRL+R : 暗号化を切り替える\nCTRL+B : brainrootを表示",
    
    //         create_lua_file: "暗号化中に追加のLuaファイルを作成",
    
    //         przetwarzanie_naglowek: "変換",
    //         general_ui_alpha_tytul: "アルファ",
    //         general_ui_filter_png_tytul: "フィルター",
    //         general_ui_warianty_tytul: "バリエーション",
    //         general_ui_rozdzielczość_tytul: "解像度",
    
    //         // monit
    //         // problem_egzystencjonalny: "今、存在的危機を感じています",
    
    //         // USTAWIENIA
    //         ustawienia_tytul: "設定",
    //         ustawienia_skrot: "Ctrl+S でウィンドウを閉じる",
    
    //         general_colors_white: "白",
    //         general_colors_black: "黒",
    //         general_colors_red: "赤",
    //         general_colors_green: "緑",
    //         general_colors_blue: "青",
    //         przetwarzanie_bez_alpha: "アルファチャンネルなし",
    //         przetwarzanie_z_alpha: "アルファチャンネルあり",
    //     }
    // }
    

}