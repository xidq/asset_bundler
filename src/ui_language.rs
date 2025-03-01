

// ██╗      █████╗ ███╗   ██╗ ██████╗ ██╗   ██╗ █████╗  ██████╗ ███████╗
// ██║     ██╔══██╗████╗  ██║██╔════╝ ██║   ██║██╔══██╗██╔════╝ ██╔════╝
// ██║     ███████║██╔██╗ ██║██║  ███╗██║   ██║███████║██║  ███╗█████╗  
// ██║     ██╔══██║██║╚██╗██║██║   ██║██║   ██║██╔══██║██║   ██║██╔══╝  
// ███████╗██║  ██║██║ ╚████║╚██████╔╝╚██████╔╝██║  ██║╚██████╔╝███████╗
// ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝



pub struct Language {

    pub szyfrowanie_naglowek: &'static str,
    pub szyfrowanie_nazwa_tytul: &'static str,
    pub szyfrowanie_nazwa: &'static str,
    pub szyfrowanie_kompresja_wybor: &'static str,
    pub szyfrowanie_kompresja: &'static str,
    pub szyfrowanie_haslo_tytul: &'static str,
    pub szyfrowanie_haslo: &'static str,
    pub szyfrowanie_haslo_wylaczone: &'static str,
    pub szyfrowanie_wybierz_ustawienia_prekonfigurowane: &'static str,
    pub szyfrowanie_przycisk_ok: &'static str,
    pub szyfrowanie_przycisk_nie_ok: &'static str,
    pub szyfrowanie_przycisk_koniec: &'static str,

    pub szyfrowanie_przysick_2: &'static str,
    pub szyfrowanie_przycisk_3: &'static str,
    pub szyfrowanie_przycisk_4: &'static str,

    pub deszyfrowanie_naglowek: &'static str,
    pub deszyfrowanie_plik_dat: &'static str,
    pub deszyfrowanie_plik_idx: &'static str,
    pub deszyfrowanie_przycisk_ok: &'static str,


    pub wybierz_folder_wejsciowy: &'static str,
    pub wybierz_folder_wyjsciowy: &'static str,
    pub wybierz_folder: &'static str,

    pub debug_deszyfracja_idx: &'static str,
    
    pub h_wybierz_folder: &'static str,
    pub ukryte_sktory: &'static str,
    pub create_lua_file: &'static str,



    //monit
    pub problem_egzystencjonalny: &'static str,

    //USTAWIENIA!!!!!!!
    pub ustawienia_tytul:&'static str,
    pub ustawienia_skrot:&'static str,

}


impl Language {

    pub fn polish() -> Self {

        Language {

            szyfrowanie_naglowek: "Szyfrowanko",
            szyfrowanie_nazwa_tytul: "Nazwa pliku",
            szyfrowanie_nazwa: "Podaj nazwę",
            szyfrowanie_kompresja_wybor: "Wybór kompresji",
            szyfrowanie_kompresja: "Zacność kompresji",
            szyfrowanie_haslo_tytul: "Hasło",
            szyfrowanie_haslo: "Hasło Zgasło, wpisz inne!",
            szyfrowanie_haslo_wylaczone: "Hasło wyłączone...",
            szyfrowanie_wybierz_ustawienia_prekonfigurowane: "<-- Predefiniowana Konfiguracja",
            szyfrowanie_przycisk_ok: "Szyfrufruj!",
            szyfrowanie_przycisk_nie_ok: "Czegoś ci jeszcze brakuje UwU",
            szyfrowanie_przycisk_koniec: "Yay, koniec!",

            szyfrowanie_przysick_2: "Yay, koniec!",
            szyfrowanie_przycisk_3: "erroru",
            szyfrowanie_przycisk_4: "erroru",

            deszyfrowanie_naglowek: "Deszyfrowanko",
            deszyfrowanie_plik_dat: "Plik .jrz/.jrzs",
            deszyfrowanie_plik_idx: "Plik .idx",
            deszyfrowanie_przycisk_ok: "Deszyfrufruj!",

            wybierz_folder_wejsciowy: "Folder Wejściowy",
            wybierz_folder_wyjsciowy: "Folder Docelowy",
            wybierz_folder: "Wybierz Folder",

            debug_deszyfracja_idx: "Deszyfracja zaszyfrowanego idx",

            h_wybierz_folder: "Wybierz folder",

            ukryte_sktory: "CTRL+R : przełącz szyfrowanie\nCTRL+B : brainroot (nie zaimplementowano)",

            create_lua_file: "Utwórz dodatkowy plik lua podczas szyfrowania",
            

            //monit
            problem_egzystencjonalny: "Ma problemy egzystencjonalne...",


            //USTAWIENIA
            ustawienia_tytul: "Ustawienia",
            ustawienia_skrot:"Ctrl+S zamknij to okno",

        }

    }


    pub fn english() -> Self {
        Language {

            szyfrowanie_naglowek: "Encoding",
            szyfrowanie_nazwa_tytul: "File name",
            szyfrowanie_nazwa: "Name your file",
            szyfrowanie_kompresja_wybor: "Compression method",
            szyfrowanie_kompresja: "Compression strength",
            szyfrowanie_haslo_tytul: "Password",
            szyfrowanie_haslo: "Write smth...",
            szyfrowanie_haslo_wylaczone: "Password is disabled.. Ya know?",
            szyfrowanie_wybierz_ustawienia_prekonfigurowane: "<-- Asset",
            szyfrowanie_przycisk_ok: "Encode!",
            szyfrowanie_przycisk_nie_ok: "Seems u forgor somting UwU",
            szyfrowanie_przycisk_koniec: "Process ended",

            szyfrowanie_przysick_2: "Process ended",
            szyfrowanie_przycisk_3: "erroru",
            szyfrowanie_przycisk_4: "erroru",

            deszyfrowanie_naglowek: "Decoding",
            deszyfrowanie_plik_dat: "File .jrz/.jrzs",
            deszyfrowanie_plik_idx: "File .idx",
            deszyfrowanie_przycisk_ok: "Decode!!!!!",


            wybierz_folder_wejsciowy: "Input Folder",
            wybierz_folder_wyjsciowy: "Output Folder",
            wybierz_folder: "Choose directory",


            debug_deszyfracja_idx: "Decrypt .idx file only",

            h_wybierz_folder: "Choose your desired folder",

            ukryte_sktory:"CTRL+R : toogle encription\nCTRL+B for brainroot",

            create_lua_file: "Create additional lua file while encryption",

            //monit
            problem_egzystencjonalny: "Have existencial crysis rn",

            // USTAWIENIA
            ustawienia_tytul: "Settings",
            ustawienia_skrot:"Ctrl+S close that window",
        }

    }

}