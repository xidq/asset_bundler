use std::{fs, io};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;
use crate::encrypt_bez_async_i_bez_chacha20::generate_unique_id;
use crate::utils::arc_mutex_strip::{
    get_locked_data_bool,
    get_locked_data_u8,
    get_locked_data_string,
    get_locked_data_pathbuf,
};

pub fn create_random_id(
    ścieżki:Arc<Mutex<Vec<PathBuf>>>,
) -> Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, io::Error > {

    let ścieżki_cloned = Arc::clone(&ścieżki);
    let ścieżki_cloned2 = get_locked_data_pathbuf(&ścieżki_cloned)?;

    match sprawdz_czy_istnieje_juz_taki_plik(&ścieżki_cloned2[0], "magic.numbers") {
        Ok(path) => {
            println!("Znaleziono plik: {:?}", &path);
            iteracja_po_każdym_pliku(&ścieżki_cloned2[0],&path)?;

        },
        Err(e) => {
            eprintln!("Błąd: {}", e);
            let sciezka_do_pliku_magic = &ścieżki_cloned2[0].join("magic.numbers");
            File::create(sciezka_do_pliku_magic)?;
            iteracja_po_każdym_pliku(&ścieżki_cloned2[0],sciezka_do_pliku_magic)?;
        },
    };

    Ok(Arc::new(Mutex::new(vec![0 as usize,1,3,2])))
}

fn sprawdz_czy_istnieje_juz_taki_plik(sciezka: &Path, plik: &str) -> Result<PathBuf, io::Error> {
    println!("jestem w sprawdz_czy_istnieje_juz_taki_plik");
    // Przechodzimy przez wszystkie pliki w katalogu
    for entry in fs::read_dir(sciezka)? {
        let entry = entry?;
        if let Some(file_name) = entry.file_name().to_str() {
            // Porównujemy nazwę pliku
            if file_name == plik {
                return Ok(entry.path());  // Zwracamy ścieżkę do pliku
            }
        }
    }

    // Jeśli plik nie został znaleziony, zwracamy błąd
    Err(io::Error::new(io::ErrorKind::NotFound, "Plik nie został znaleziony"))
}

pub fn iteracja_po_każdym_pliku(ścieżka: &PathBuf, ścieżka_pliku_magic: &PathBuf) -> io::Result<()> {
    #[cfg(feature = "statystyki")]
    println!("jestem w iteracja_po_każdym_pliku");
    // Otwórz plik magic.numbers do odczytu i zapisu
    let mut plik_magic = match OpenOptions::new()
        .read(true)  // Otwórz do odczytu
        .write(true) // Otwórz do zapisu
        .create(true) // Jeśli plik nie istnieje, utwórz go
        .open(ścieżka_pliku_magic){
        Ok(file) => {
            #[cfg(feature = "statystyki")]
            println!(" iteracja_po_kazdym_pliku plik_magic OK!");
            file },
        Err(_) => {
            #[cfg(feature = "statystyki")]
            println!(" iteracja_po_kazdym_pliku plik_magic ERROR!");
            File::create(ścieżka_pliku_magic)? },
    };

    // Wczytaj istniejące ID z pliku i zapisz je w HashSet
    let mut istniejące_id = HashSet::new();
    let mut istniejące_ścieżki = HashSet::new();  // Do przechowywania już zapisanych ścieżek
    let reader = io::BufReader::new(&plik_magic);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            istniejące_id.insert(parts[0].to_string());
            istniejące_ścieżki.insert(parts[1].to_string()); // Dodajemy pełną ścieżkę do tego zestawu
        }
    }

    let mut nowe_id = HashSet::new();
    #[cfg(feature = "statystyki")]
    println!("originalna zawartość pliku: {:?}", istniejące_id);

    // Iteruj po plikach w podanej ścieżce
    let entries = fs::read_dir(ścieżka)?;
    for entry in WalkDir::new(ścieżka).into_iter().filter_map(Result::ok){

        let path = entry.path();

        // Jeśli to plik, generuj unikalne ID i sprawdź, czy już istnieje
        if path.is_file() {
            let path_str = path.display().to_string();

            // Jeśli ścieżka już istnieje w pliku magic.numbers, pomijamy ten plik
            if istniejące_ścieżki.contains(&path_str) {
                #[cfg(feature = "statystyki")]
                println!("Plik {} już istnieje w magic.numbers, pomijam go.", path_str);
                continue;  // Przechodzimy do następnego pliku
            }

            let mut unikalne_id = generate_unique_id();
            #[cfg(feature = "statystyki")]
            println!("Wygenerowano ID: {}", unikalne_id);

            // Sprawdzaj, czy ID już istnieje, a jeśli tak, generuj nowe
            while istniejące_id.contains(&unikalne_id) || nowe_id.contains(&unikalne_id) {
                #[cfg(feature = "statystyki")]
                println!("ID {} już istnieje, generuję nowe...", unikalne_id);
                unikalne_id = generate_unique_id();
                #[cfg(feature = "statystyki")]
                println!("Wygenerowano nowe ID: {}", unikalne_id);
            }

            // Dodaj nowe ID do HashSet (będziemy je dodawać do pliku po zakończeniu iteracji)
            nowe_id.insert(unikalne_id.clone());

            // Zapisz faktyczną ścieżkę pliku i ID
            let zapis = format!("{} {}\n", unikalne_id, path.strip_prefix(ścieżka).unwrap().display());
            #[cfg(feature = "statystyki")]
            println!("Zapisuję do pliku: {}", zapis);
            plik_magic.write_all(zapis.as_bytes())?;
        }
    }


    Ok(())
}

pub fn generate_magic_number_file(){

}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_na_plikach(){
        let ścieżka:PathBuf= "assets/".parse().unwrap();
        match create_random_id(Arc::new(Mutex::new(vec![ścieżka]))){
            Ok(_) =>{println!("gotowe")},
            Err(e) =>{eprintln!("coś poszło nie tak {}",e)}
        }

    }
}