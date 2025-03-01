use std::fs::{self, File};
use std::io::{Read, Write};
use std::usize;
use std::path::Path;
use std::time::Instant;
use std::sync::{Arc,mpsc, Mutex};
use std::path::PathBuf;
// use rodio::decoder;
// use zstd::*;
use std::thread;
use chrono::offset::Local;
use egui::output;
use std::time::Duration;

use crate::moi_chacha20;
// use crate::xor;
// use xor_utils::*;



// ██████  ███████  █████  ██████      
// ██   ██ ██      ██   ██ ██   ██     
// ██████  █████   ███████ ██   ██     
// ██   ██ ██      ██   ██ ██   ██     
// ██   ██ ███████ ██   ██ ██████      
                                    
                                    
// ███████ ██████   ██████  ███    ███ 
// ██      ██   ██ ██    ██ ████  ████ 
// █████   ██████  ██    ██ ██ ████ ██ 
// ██      ██   ██ ██    ██ ██  ██  ██ 
// ██      ██   ██  ██████  ██      ██ 
                                    
                                    
// ██ ██████  ██   ██                  
// ██ ██   ██  ██ ██                   
// ██ ██   ██   ███                    
// ██ ██   ██  ██ ██                   
// ██ ██████  ██   ██   

/// reading files data from idx file
pub fn read_index_file(idx_path: &Path, _de_password: &str, _toggle_encryption: u8) -> std::io::Result<Vec<(String, u64, u64)>> {

    println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n---> Odczytywanie pliku indeksu: {}\n", 
        Local::now().format("%H:%M:%S"), idx_path.display());

    let mut file = File::open(idx_path)?;

    if !idx_path.exists() {

        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, 
        "\n##################################\n##### Plik .idx nie istnieje #####\n##################################\n"));

    }

    let mut idx_bytes = Vec::new();
    file.read_to_end(&mut idx_bytes)?;

    println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n---> Odczytano {} bajtów z pliku indeksu\n", 
        Local::now().format("%H:%M:%S"), idx_bytes.len());

    // just NAH
    // if toggle_encryption ==1 {
    //     // let key = de_password.as_bytes();
    //     println!("[Deszyfrowanie/read_index_file/toggle_encryption :: LocalTime:{}]\n---> Deszyfrowanie indeksów za pomocą podanego hasła...\n", Local::now().format("%H:%M:%S"));
    //     xor::xor_encrypt_decrypt(&mut idx_bytes, de_password);
    // } else {
    //     println!("[Deszyfrowanie/read_index_file/toggle_encryption :: LocalTime:{}]\n---> Deszyfrowanie pominięte, dane są już w stanie niezaszyfrowanym.\n", Local::now().format("%H:%M:%S"));
    // }

    // after reading etc
    println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n---> Dane po deszyfrowaniu indeksu: \n--> {:?}\n", Local::now().format("%H:%M:%S"), 
        &idx_bytes[0..idx_bytes.len().min(100)]);

    let idx_data = String::from_utf8_lossy(&idx_bytes);
    let mut file_infos = Vec::new();

    //getting data
    for line in idx_data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 7 {
            let _type_name = parts[0].to_string();
            let _type_name_id = parts[1].to_string();
            let _type_name_variant = parts[2].to_string();
            let _type_name_variant_size = parts[3].parse::<u64>().unwrap_or(0);
            let _file_unique_id = parts[4].to_string();
            let file_path = parts[5].to_string();
            let file_size = parts[6].parse::<u64>().unwrap_or(0);
            let offset = parts[7].parse::<u64>().unwrap_or(0);

            println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n---> Załadowany plik z indeksu: {}, rozmiar: {}, offset: {}\n", 
                Local::now().format("%H:%M:%S"), file_path, file_size, offset); // Logowanie

            file_infos.push((file_path, file_size, offset)); // adding necesarry data to vector

        } else {
            println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n\n##### ---> Błąd w danych indeksu: {} <---#####\n\n", Local::now().format("%H:%M:%S"), line); // Logowanie błędnych wierszy
        }
    }

    println!("[Deszyfrowanie/read_index_file :: LocalTime:{}]\n---> Załadowano informacje o {} pliku/plikach z indeksu\n", Local::now().format("%H:%M:%S"), file_infos.len());
    Ok(file_infos)
}

// Function that was used for debugging problems when i was using xor - depriciated!
// not used, but still have it own place in ui tho, so let it be here.
pub fn decrypt_idx_to_text_file(
    debug_idx_file: &Path, 
    debug_output_folder: &Path, 
    _debug_idx_password: &str,
    _toggle_encryption: u8
) -> std::io::Result<()> {
    println!("Ścieżka do której dąży:{:?}",debug_output_folder);
    println!("[Deszyfrowanie/decrypt_idx_to_text_file :: LocalTime:{}]\n---> Rozpoczęcie deszyfrowania pliku idx: {}\n", Local::now().format("%H:%M:%S"), debug_idx_file.display());

    // Odczytujemy zawartość pliku .idx
    let mut file = File::open(debug_idx_file)?;
    let mut idx_bytes = Vec::new();
    file.read_to_end(&mut idx_bytes)?;

    // if toggle_encryption == 1{
    //     // let key = debug_idx_password.as_bytes();
    //     println!("[Deszyfrowanie/decrypt_idx_to_text_file/toggle_encription :: LocalTime:{}]\n---> Deszyfrowanie indeksu za pomocą podanego hasła\n", Local::now().format("%H:%M:%S"));
    //     xor::xor_encrypt_decrypt(&mut idx_bytes, debug_idx_password);
    // } else {
    //     println!("[Deszyfrowanie/decrypt_idx_to_text_file/toggle_encription :: LocalTime:{}]\n---> Deszyfrowanie pominięte, dane są nie zaszyfrowane.\n", Local::now().format("%H:%M:%S"));
    // }

    // Debugowanie zawartości indeksu po deszyfrowaniu
    println!("[Deszyfrowanie/decrypt_idx_to_text_file :: LocalTime:{}]\n---> Dane po deszyfrowaniu indeksu (pierwsze 50): {:?}", Local::now().format("%H:%M:%S"), &idx_bytes[0..idx_bytes.len().min(50)]);

    // Zamiana danych na tekst
    let decrypted_idx_data = String::from_utf8_lossy(&idx_bytes);

    // Przygotowanie ścieżki do pliku wynikowego
    let output_file = format!("{}/decrypted_idx.txt", debug_output_folder.display());
    let mut output_file = File::create(output_file)?;

    // Zapisanie zawartości deszyfrowanego pliku .idx do pliku tekstowego
    output_file.write_all(decrypted_idx_data.as_bytes())?;
    println!("[Deszyfrowanie/decrypt_idx_to_text_file :: LocalTime:{}]\n---> Plik z deszyfrowanymi danymi zapisany w: {}\n\n", Local::now().format("%H:%M:%S"), debug_output_folder.display());

    Ok(())
}



// ███    ███  █████  ██ ███    ██                                   
// ████  ████ ██   ██ ██ ████   ██                                   
// ██ ████ ██ ███████ ██ ██ ██  ██                                   
// ██  ██  ██ ██   ██ ██ ██  ██ ██                                   
// ██      ██ ██   ██ ██ ██   ████                                   
                                                                  
                                                                  
// ███████ ██    ██ ███    ██  ██████ ████████ ██  ██████  ███    ██ 
// ██      ██    ██ ████   ██ ██         ██    ██ ██    ██ ████   ██ 
// █████   ██    ██ ██ ██  ██ ██         ██    ██ ██    ██ ██ ██  ██ 
// ██      ██    ██ ██  ██ ██ ██         ██    ██ ██    ██ ██  ██ ██ 
// ██       ██████  ██   ████  ██████    ██    ██  ██████  ██   ████ 
                                                                  
                                                                  
// ██   ██ ███████ ██████  ███████                                   
// ██   ██ ██      ██   ██ ██                                        
// ███████ █████   ██████  █████                                     
// ██   ██ ██      ██   ██ ██                                        
// ██   ██ ███████ ██   ██ ███████  

/// Funkcja deszyfrująca pliki
pub fn decrypt_files(
    arc_z_path:Arc<Mutex<Vec<PathBuf>>>,
    arc_z_str: Arc<Mutex<Vec<String>>>,
    arc_z_u8: Arc<Mutex<Vec<u8>>>
) -> Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error > {

    let time_start = Instant::now();
    let mut processed_files_count = 0; // licznik przetworzonych plików
    let arc_z_u8_clone = Arc::clone(&arc_z_u8);
    let arc_z_path_clone = Arc::clone(&arc_z_path);
    let arc_z_str_clone = Arc::clone(&arc_z_str);

    println!("[Deszyfrowanie/decrypt_files] Rozpoczęcie procesu...");
    let nowy_watek = thread::spawn(move||-> Result<usize, std::io::Error>{
        let strip_arc_z_u8_clone = arc_z_u8_clone.lock().unwrap();
        let toggle_encryption= strip_arc_z_u8_clone[0];

        let strip_arc_z_path_clone = arc_z_path_clone.lock().unwrap();
        let dat_path = &strip_arc_z_path_clone[0];
        let idx_path = &strip_arc_z_path_clone[1];
        let aoutput_folder = &strip_arc_z_path_clone[2];
        let output_folder: &str = Option::expect(aoutput_folder.to_str(), "&str" );

        let strip_arc_z_str_clone = arc_z_str_clone.lock().unwrap();
        let de_password = &strip_arc_z_str_clone[0];

        // Odczytaj informacje o plikach z indeksu
        let file_infos = read_index_file(&idx_path, de_password, toggle_encryption)?;

        // read whole binary file
        let mut dat_file = File::open(dat_path)?;
        let mut dat_bytes = Vec::new();
        dat_file.read_to_end(&mut dat_bytes)?;

        // // Deszyfruj CAŁY plik .dat jeśli potrzebne
        if toggle_encryption == 1 {
            // xor::xor_encrypt_decrypt(&mut dat_bytes, de_password);
            moi_chacha20::decrypt(&mut dat_bytes, de_password);
        }

        for (file_path, file_size, offset) in file_infos {
            let start = offset as usize;
            let end = start + file_size as usize;
            processed_files_count += 1;
            // Wyodrębnij fragment danych
            let mut chunk = dat_bytes[start..end].to_vec();
            let toggle_compression:u8 =1;
            // Obsługa magic bytes i kompresji
            if toggle_compression == 1 {
                match try_decompress_zstd(&chunk) {
                    Ok(Some(decompressed)) => {
                        chunk = decompressed;
                    },
                    Ok(None) => {
                        println!("[WARNING] Plik {} nie jest skompresowany", file_path);
                    },
                    Err(e) => {
                        eprintln!("[ERROR] Błąd dekompresji {}: {}", file_path, e);
                        continue;
                    }
                }
            }

            // Zapisz plik
            let output_path = format!("{}/{}", output_folder, &file_path);
            fs::create_dir_all(Path::new(&output_path).parent().unwrap())?;
            fs::write(output_path, &chunk)?;
        }
        Ok(processed_files_count)
    });

    let przeprocesowane_pliki = nowy_watek.join().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other,"Wątek zakończył się błędem"))??;
    let czas_trwania = time_start.elapsed();
    let czas_trwania_sekundy: usize = czas_trwania.as_secs() as usize;
    let czas_trwania_milisekundy:usize = czas_trwania.subsec_millis() as usize;
    
    Ok(Arc::new(Mutex::new(vec![przeprocesowane_pliki,czas_trwania_sekundy, czas_trwania_milisekundy])))
}


// Old, just in case

// fn try_decompress_zstd(data: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
//     if data.starts_with(b"Zstd") { // Zakładając, że plik ma nagłówek Zstd
//         let mut decoder = Decoder::new(data)?;
//         let mut decompressed_data = Vec::new();
//         decoder.read_to_end(&mut decompressed_data)?;
//         return Ok(Some(decompressed_data));
//     }
//     Ok(None) // Zwracamy None, jeśli plik nie jest skompresowany Zstd
// }



// ██████  ███████  ██████  ██████  ███    ███ ██████  ██████  ███████ ███████ ███████ 
// ██   ██ ██      ██      ██    ██ ████  ████ ██   ██ ██   ██ ██      ██      ██      
// ██   ██ █████   ██      ██    ██ ██ ████ ██ ██████  ██████  █████   ███████ ███████ 
// ██   ██ ██      ██      ██    ██ ██  ██  ██ ██      ██   ██ ██           ██      ██ 
// ██████  ███████  ██████  ██████  ██      ██ ██      ██   ██ ███████ ███████ ███████ 
                                                                                    
                                                                                    
// ███████ ██    ██ ███    ██  ██████ ████████ ██  ██████  ███    ██                   
// ██      ██    ██ ████   ██ ██         ██    ██ ██    ██ ████   ██                   
// █████   ██    ██ ██ ██  ██ ██         ██    ██ ██    ██ ██ ██  ██                   
// ██      ██    ██ ██  ██ ██ ██         ██    ██ ██    ██ ██  ██ ██                   
// ██       ██████  ██   ████  ██████    ██    ██  ██████  ██   ████  

//propably here is something off, magicbytes are used in encoding,
// problem is that in text files i can see some additional bytes at the end, photos etc are working tho

fn try_decompress_zstd(data: &[u8]) -> std::io::Result<Option<Vec<u8>>> {
    const ZSTD_MAGIC: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];
    
    
    // Sprawdź czy dane zawierają magic bytes Zstd
    if data.len() < 4 || data[0..4] != ZSTD_MAGIC {
        return Ok(None);
    }

    // Stwórz dekoder z automatyczną detekcją magic bytes
    let mut decoder = match zstd::Decoder::new(data) {

        Ok(d) => d,
        Err(e) => {
            println!("[ERROR] Błąd dekodera Zstd: {}", e);
            return Ok(None);
        }
    };

    // Dekompresuj
    let mut decompressed = Vec::new();
    match decoder.read_to_end(&mut decompressed) {
        Ok(_) => Ok(Some(decompressed)),
        Err(e) => {
            println!("[ERROR] Błąd dekompresji: {}", e);
            Ok(None)
        }
    }
}





// ████████ ███████ ███████ ████████ ███████ ██████  
//    ██    ██      ██         ██    ██           ██ 
//    ██    █████   ███████    ██    ███████   ▄███  
//    ██    ██           ██    ██         ██   ▀▀    
//    ██    ███████ ███████    ██    ███████   ██

// const SRC_FOLDER: &str = "test_files/source_files";
// const COMPRESSED_IDX: &str = "test_files/after_compression/compressed_data.idx";
// const COMPRESSED_DAT: &str = "test_files/after_compression/compressed_data.dat";
// const RESULT_FOLDER: &str = "test_files/result_files";
// const PASSWORD: &str = "CyCkI"; // Przykładowe hasło
// const COMPRESSED_IDX_U: &str = "test_files/after_compression/uncompressed_data.idx";
// const COMPRESSED_DAT_U: &str = "test_files/after_compression/uncompressed_data.dat";
// const RESULT_FOLDER_U: &str = "test_files/result_files_uncompressed";
// const COMPRESSED_IDX_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.idx";
// const COMPRESSED_DAT_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.dat";
// const RESULT_FOLDER_SZ_U: &str = "test_files/result_files_szyfrowana_uncompressed";
// const COMPRESSED_IDX_SZ: &str = "test_files/after_compression/szyfrowana_data.idx";
// const COMPRESSED_DAT_SZ: &str = "test_files/after_compression/szyfrowana_data.dat";
// const RESULT_FOLDER_SZ: &str = "test_files/result_files_szyfrowana";


// #[cfg(test)]
// mod test_skompresowany {
//     use super::*;
//     use std::fs;
//     use std::path::{Path, PathBuf};
//     use std::io::{self, Read};
//     use std::fs::File;



//     fn compare_files(src_file: &Path, result_file: &Path) -> io::Result<()> {
//         let mut src_data = Vec::new();
//         let mut result_data = Vec::new();

//         // Odczytanie danych z pliku źródłowego
//         let mut src = File::open(src_file)?;
//         src.read_to_end(&mut src_data)?;

//         // Odczytanie danych z pliku wynikowego
//         let mut result = File::open(result_file)?;
//         result.read_to_end(&mut result_data)?;

//         // Porównanie danych
//         if src_data != result_data {
//             return Err(io::Error::new(io::ErrorKind::Other, "Dane plików się różnią"));
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_decrypt_files() {
//         let dat_file = Path::new(COMPRESSED_DAT);
//         let idx_file = Path::new(COMPRESSED_IDX);
//         let outputt_folder = RESULT_FOLDER;

//         // Dekompresja plików
//         let result = decrypt_files(dat_file, idx_file, outputt_folder, PASSWORD, 0);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }

//         // Pobranie listy plików w folderze źródłowym i wynikowym
//         let src_files = fs::read_dir(SRC_FOLDER).expect("Nie udało się odczytać folderu źródłowego");
//         let result_files = fs::read_dir(outputt_folder).expect("Nie udało się odczytać folderu wynikowego");

//         // Iteracja po plikach źródłowych
//         for src_entry in src_files {
//             let src_entry = src_entry.expect("Nie udało się odczytać pliku w folderze źródłowym");
//             let src_file = src_entry.path();

//             // Sprawdzenie, czy odpowiadający plik znajduje się w folderze wynikowym
//             if let Some(file_name) = src_file.file_name() {
//                 let result_file = Path::new(outputt_folder).join(file_name);
//                 if result_file.exists() {
//                     // Porównanie plików
//                     match compare_files(&src_file, &result_file) {
//                         Ok(_) => println!("Pliki są identyczne: {}", file_name.to_string_lossy()),
//                         Err(e) => eprintln!("Pliki różnią się: {}: {}", file_name.to_string_lossy(), e),
//                     }
//                 } else {
//                     eprintln!("Brak pliku w folderze wynikowym: {}", file_name.to_string_lossy());
//                 }
//             }
//         }
//     }
// }

// mod test_nieskompresowane {
//     use super::*;
//     use std::fs;
//     use std::path::{Path, PathBuf};
//     use std::io::{self, Read};
//     use std::fs::File;



//     fn compare_files(src_file: &Path, result_file: &Path) -> io::Result<()> {
//         let mut src_data = Vec::new();
//         let mut result_data = Vec::new();

//         // Odczytanie danych z pliku źródłowego
//         let mut src = File::open(src_file)?;
//         src.read_to_end(&mut src_data)?;

//         // Odczytanie danych z pliku wynikowego
//         let mut result = File::open(result_file)?;
//         result.read_to_end(&mut result_data)?;

//         // Porównanie danych
//         if src_data != result_data {
//             return Err(io::Error::new(io::ErrorKind::Other, "Dane plików się różnią"));
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_decrypt_files() {
//         let dat_file = Path::new(COMPRESSED_DAT_U);
//         let idx_file = Path::new(COMPRESSED_IDX_U);
//         let outputt_folder = RESULT_FOLDER_U;

//         // Dekompresja plików
//         let result = decrypt_files(dat_file, idx_file, outputt_folder, PASSWORD, 0);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }

//         // Pobranie listy plików w folderze źródłowym i wynikowym
//         let src_files = fs::read_dir(SRC_FOLDER).expect("Nie udało się odczytać folderu źródłowego");
//         let result_files = fs::read_dir(outputt_folder).expect("Nie udało się odczytać folderu wynikowego");

//         // Iteracja po plikach źródłowych
//         for src_entry in src_files {
//             let src_entry = src_entry.expect("Nie udało się odczytać pliku w folderze źródłowym");
//             let src_file = src_entry.path();

//             // Sprawdzenie, czy odpowiadający plik znajduje się w folderze wynikowym
//             if let Some(file_name) = src_file.file_name() {
//                 let result_file = Path::new(outputt_folder).join(file_name);
//                 if result_file.exists() {
//                     // Porównanie plików
//                     match compare_files(&src_file, &result_file) {
//                         Ok(_) => println!("Pliki są identyczne: {}", file_name.to_string_lossy()),
//                         Err(e) => eprintln!("Pliki różnią się: {}: {}", file_name.to_string_lossy(), e),
//                     }
//                 } else {
//                     eprintln!("Brak pliku w folderze wynikowym: {}", file_name.to_string_lossy());
//                 }
//             }
//         }
//     }
// }

// mod tests_szyfrowany_nieskompresowane {
//     use super::*;
//     use std::fs;
//     use std::path::{Path, PathBuf};
//     use std::io::{self, Read};
//     use std::fs::File;



//     fn compare_files(src_file: &Path, result_file: &Path) -> io::Result<()> {
//         let mut src_data = Vec::new();
//         let mut result_data = Vec::new();

//         // Odczytanie danych z pliku źródłowego
//         let mut src = File::open(src_file)?;
//         src.read_to_end(&mut src_data)?;

//         // Odczytanie danych z pliku wynikowego
//         let mut result = File::open(result_file)?;
//         result.read_to_end(&mut result_data)?;

//         // Porównanie danych
//         if src_data != result_data {
//             return Err(io::Error::new(io::ErrorKind::Other, "Dane plików się różnią"));
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_decrypt_files() {
//         let dat_file = Path::new(COMPRESSED_DAT_SZ_U);
//         let idx_file = Path::new(COMPRESSED_IDX_SZ_U);
//         let outputt_folder = RESULT_FOLDER_SZ_U;

//         // Dekompresja plików
//         let result = decrypt_files(dat_file, idx_file, outputt_folder, PASSWORD, 1);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }

//         // Pobranie listy plików w folderze źródłowym i wynikowym
//         let src_files = fs::read_dir(SRC_FOLDER).expect("Nie udało się odczytać folderu źródłowego");
//         let result_files = fs::read_dir(outputt_folder).expect("Nie udało się odczytać folderu wynikowego");

//         // Iteracja po plikach źródłowych
//         for src_entry in src_files {
//             let src_entry = src_entry.expect("Nie udało się odczytać pliku w folderze źródłowym");
//             let src_file = src_entry.path();

//             // Sprawdzenie, czy odpowiadający plik znajduje się w folderze wynikowym
//             if let Some(file_name) = src_file.file_name() {
//                 let result_file = Path::new(outputt_folder).join(file_name);
//                 if result_file.exists() {
//                     // Porównanie plików
//                     match compare_files(&src_file, &result_file) {
//                         Ok(_) => println!("Pliki są identyczne: {}", file_name.to_string_lossy()),
//                         Err(e) => eprintln!("Pliki różnią się: {}: {}", file_name.to_string_lossy(), e),
//                     }
//                 } else {
//                     eprintln!("Brak pliku w folderze wynikowym: {}", file_name.to_string_lossy());
//                 }
//             }
//         }
//     }
// }

// mod tests_szyfrowany_skompresowane {
//     use super::*;
//     use std::fs;
//     use std::path::{Path, PathBuf};
//     use std::io::{self, Read};
//     use std::fs::File;



//     fn compare_files(src_file: &Path, result_file: &Path) -> io::Result<()> {
//         let mut src_data = Vec::new();
//         let mut result_data = Vec::new();

//         // Odczytanie danych z pliku źródłowego
//         let mut src = File::open(src_file)?;
//         src.read_to_end(&mut src_data)?;

//         // Odczytanie danych z pliku wynikowego
//         let mut result = File::open(result_file)?;
//         result.read_to_end(&mut result_data)?;

//         // Porównanie danych
//         if src_data != result_data {
//             return Err(io::Error::new(io::ErrorKind::Other, "Dane plików się różnią"));
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_decrypt_files() {
//         let dat_file = Path::new(COMPRESSED_DAT_SZ);
//         let idx_file = Path::new(COMPRESSED_IDX_SZ);
//         let outputt_folder = RESULT_FOLDER_SZ;

//         // Dekompresja plików
//         let result = decrypt_files(dat_file, idx_file, outputt_folder, PASSWORD, 1);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }

//         // Pobranie listy plików w folderze źródłowym i wynikowym
//         let src_files = fs::read_dir(SRC_FOLDER).expect("Nie udało się odczytać folderu źródłowego");
//         let result_files = fs::read_dir(outputt_folder).expect("Nie udało się odczytać folderu wynikowego");

//         // Iteracja po plikach źródłowych
//         for src_entry in src_files {
//             let src_entry = src_entry.expect("Nie udało się odczytać pliku w folderze źródłowym");
//             let src_file = src_entry.path();

//             // Sprawdzenie, czy odpowiadający plik znajduje się w folderze wynikowym
//             if let Some(file_name) = src_file.file_name() {
//                 let result_file = Path::new(outputt_folder).join(file_name);
//                 if result_file.exists() {
//                     // Porównanie plików
//                     match compare_files(&src_file, &result_file) {
//                         Ok(_) => println!("Pliki są identyczne: {}", file_name.to_string_lossy()),
//                         Err(e) => eprintln!("Pliki różnią się: {}: {}", file_name.to_string_lossy(), e),
//                     }
//                 } else {
//                     eprintln!("Brak pliku w folderze wynikowym: {}", file_name.to_string_lossy());
//                 }
//             }
//         }
//     }
// }
