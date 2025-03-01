use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zstd::*;
use chrono::offset::Local;
use rand::*;
use rand::distr::Alphanumeric;
// use tokio::*;
// use pbkdf2::pbkdf2;
// use sha2::Sha256;
// use xor_utils::*;
// use aes::Aes256; // Użyjemy AES-256
use chacha20poly1305::*;
// use chacha20poly1305::aead::generic_array::sequence::GenericSequence;
use rand::rngs::OsRng;


use crate::xor;
// use crate::chacha20;






fn generate_unique_id() -> String {
    let mut rng = rand::rng();
    // Generowanie losowego identyfikatora 12-znakowego
    (0..12)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect()
}


fn sanitize_filename(filename: &str) -> String {
    let mut sanitized = filename
        .replace(' ', "-")    // Zamiana spacji na %20
        .replace('?', "-")       // Zamiana '?' na '-'
        .replace('%', "-");       // Zamiana '%' na '-'
            // Zmiana kropek (oprócz tej przed rozszerzeniem)
    let parts: Vec<&str> = sanitized.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        sanitized = parts[1].replace('.', "-") + "." + parts[0]; // Zmieniamy kropki przed rozszerzeniem
    }
    
    sanitized
}


/// Typy plików dla różnych template'ów
fn get_template_extensions(template: &str) -> Vec<&'static str> {
    match template {
        "assets" => vec!["glb", "jpg", "png", "ktx2", "exr", "tga", "dds", "ico", "tiff", "bmp", "ogg", "mp3", "wav", "flac", "oga", "spx", "cfg","meta"],
        _ => vec![], // Domyślnie bierzemy wszystkie pliki
    }
}

/// Sprawdza, czy plik pasuje do template
fn matches_template(path: &Path, template: &str) -> bool {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let allowed_extensions = get_template_extensions(template);
    
    allowed_extensions.is_empty() || allowed_extensions.contains(&ext.as_str())
}

/// Zwraca nazwę pliku z prefiksem w zależności od rozszerzenia
fn get_type(path: &Path) -> String {
    let ext = path.extension()
                  .and_then(|s| s.to_str())
                  .unwrap_or("")
                  .to_lowercase();
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");
    match ext.as_str() {
        "glb" | "obj" | "3mf" | "fbx" => format!("Model"),
        "jpg" | "png" | "exr" | "tga" | "dds" | "ico" | "tiff" | "bmp"  => format!("Image"),
        "ktx2" | "hdr" => format!("EnvTex"),
        "ogg" | "mp3" | "wav" | "flac" | "oga" | "spx" => format!("Audio"),
        "epub" => format!("E-book"),
        "pdf" => format!("PDF"),
        "xls" | "xlsx" => format!("Spreadsheet"),
        "cfg" => format!("Configuration"),
        "meta" => format!("Meta"),
        _ => sanitize_filename(base).to_string(),
    }
}

fn get_type_id(path: &Path) -> String {
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");
    // Główny prefix przed pierwszym "_" lub "-"
    let san_base = sanitize_filename(base);
    san_base.split('_')
        .next()
        .unwrap_or("")
        .to_string()
}

fn get_type_variant(path: &Path) -> String {
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");
                let san_base = sanitize_filename(base);
    let parts: Vec<&str> = san_base.split('_').collect();
    if parts.len() > 1{
        parts[1].to_string()
    }else{
        String::new()
    }
}

fn get_type_variant_size(path: &Path) -> String {
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");
                let san_base = sanitize_filename(base);
    let parts: Vec<&str> = san_base.split('_').collect();
    if parts.len() > 2{
        parts[2].to_string()
    }else{
        String::new()
    }
}

// To jest główna funkcja szyfrująca tutaj, przyjmuje argumenty z ui,
// powinna ogarniać wszystkie dane, a jak jest zobaczymy ;)

pub fn encrypt_folder(
    input_folder: &str, 
    output_file: &str, 
    index_file: &str, 
    template: &str, 
    password: &str, 
    toggle_encryption: u8, 
    toggle_compression: u8, 
    poziom_kompresji: u8, 
    debug_create_lua_file: bool,) -> tokio::io::Result<()> {


    let mut info_file: Option<File> = None; // Początkowo ustawiony na None
    // let mut existing_ids: ;


    let mut output = File::create(output_file)
        .map_err(|e| {
            eprintln!("[Szyfrowanie/encrypt_folder/output :: LocalTime:{}]\n---> Błąd przy tworzeniu pliku wyjściowego {}: {}\n", Local::now().format("%H:%M:%S"), output_file, e);
            e

    })?;

    let mut index = File::create(index_file)    
            .map_err(|e| {
        eprintln!("[Szyfrowanie/encrypt_folder/index :: LocalTime:{}]\n---> Błąd przy tworzeniu pliku indeksu {}: {}\n", Local::now().format("%H:%M:%S"), index_file, e);
        e

    })?;

    
    if debug_create_lua_file {
        info_file = Some(File::create(format!("{}{}", output_file, ".lua"))
            .map_err(|e| {
                eprintln!("[Szyfrowanie/encrypt_folder/info_file :: LocalTime:{}]\n---> Błąd przy tworzeniu pliku info: {}\n", Local::now().format("%H:%M:%S"), e);
                e
            })?);
    }
    
    let mut file_entries = Vec::new();

    // Zbierane pliki zgodne z template
    for entry in WalkDir::new(input_folder).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && matches_template(path, template) {
            file_entries.push(path.to_path_buf());
        }
    }
    let signature = "created by Patryk Jerzak, ALL RIGHTS RESERVED";
    let signature_bytes = signature.as_bytes();

    output.write_all(&(file_entries.len() as u32).to_le_bytes())?;
    output.write_all(signature_bytes)?;

    let  offset_handler: u64 = if toggle_compression>=1{58}else{49};
    let mut file_infos = Vec::new();
    let mut data_start_pos: u64 = offset_handler; // Po liczbie plików
    // let mut current_offset = data_start_pos; // Początkowy offset po nagłówku
    let header_size = 4 + signature_bytes.len() as u64; // Dynamiczny rozmiar nagłówka
    let mut current_offset = header_size; 



    // Przetwarzamy pliki
    for path in &file_entries {

        let file_path = path.strip_prefix(input_folder)
            .unwrap()
            .to_str()
            .unwrap()
            .replace("\\", "/");

        let unique_id = generate_unique_id();  // Generowanie ID
        let type_name = get_type(path);
        let type_name_id = get_type_id(path);
        let type_name_variant = get_type_variant(path);
        let type_name_variant_size = get_type_variant_size(path);

        let type_name_id = if type_name_id.is_empty() { "0".to_string() } else { type_name_id };
        let type_name_variant = if type_name_variant.is_empty() { "0".to_string() } else { type_name_variant };
        let type_name_variant_size = if type_name_variant_size.is_empty() { "0".to_string() } else { type_name_variant_size };


        let metadata = fs::metadata(path)?;
        let original_file_size = metadata.len();

        file_infos.push((type_name.clone(), type_name_id.clone(), type_name_variant.clone(), type_name_variant_size.clone(), unique_id.clone(), file_path.clone(), original_file_size, current_offset));

        current_offset += original_file_size;
    }

    let mut current_offset = header_size;

    for i in 0..file_infos.len() {
        // Wydobądź potrzebne dane bez mutowalnego pożyczenia
        let (file_path, original_file_size, _) = {
            let entry = &file_infos[i];
            (entry.5.clone(), entry.6, entry.7)
        };
    
        let full_path = format!("{}/{}", input_folder, file_path);
        let mut file = File::open(&full_path)?;
    
        // Odczytaj plik używając ORYGINALNEGO rozmiaru (przed kompresją)
        let mut buffer = vec![0; original_file_size as usize];
        file.read_exact(&mut buffer)?;


        println!("[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> dat po, Odczytane dane (pierwsze 50 bajtów): \n---> {:?}\n", Local::now().format("%H:%M:%S"), &buffer[0..buffer.len().min(50)]);
        let mut processed_data = Vec::new();
        if toggle_compression == 1 {
            let mut compressed_data = Vec::new();{
            use std::io::Seek; // Wymagane do użycia stream_position()
        
            let mut encoder = Encoder::new(&mut compressed_data, poziom_kompresji as i32)?;
            encoder.multithread(2)?;
            encoder.include_magicbytes(true)?;
            encoder.write_all(&buffer)?;
            encoder.write_all(&(file_entries.len() as u32).to_le_bytes())?;
            encoder.finish()?;
            }
            processed_data = compressed_data;
        }else{
            processed_data = buffer.clone();
        }

        if toggle_encryption == 1 {
            xor::xor_encrypt_decrypt(&mut processed_data, password);
        }
        //na chacha20
        // let mut data = vec![0u8; 1024]; // Bufor musi pomieścić dane + 16+24+16 bajtów
        // let plaintext = b"Tajne dane!";
        // data[..plaintext.len()].copy_from_slice(plaintext);
        
        // encrypt(&mut data, "moje_mocne_hasło");
        
        // decrypt(&mut data, "moje_mocne_hasło");
        
        // assert_eq!(&data[..plaintext.len()], plaintext);


    // Aktualizuj rozmiar i offset w file_infos
    file_infos[i].6 = processed_data.len() as u64; // Nowy rozmiar
    file_infos[i].7 = current_offset; // Nowy offset

    current_offset += processed_data.len() as u64;
    output.write_all(&processed_data)?;
    
    }

    // Zapis do pliku indeksu – dodajemy zarówno display_name jak i file_path
    let mut idx_data = Vec::new();
    for (type_name, type_name_id, type_name_variant, type_name_variant_size, unique_id, new_file_path, file_size, offset) in &file_infos {
        let sanitized_file_path = sanitize_filename(new_file_path);
        println!("[Szyfrowanie/encrypt_folder/idx_data :: LocalTime:{}]\n---> Plik: {},\n---> Rozmiar: {},\n---> Offset: {}\n", Local::now().format("%H:%M:%S"), new_file_path,file_size, offset);
        let entry = format!(
            "{} {} {} {} {} {} {} {}", 
            type_name, type_name_id, type_name_variant, type_name_variant_size, unique_id, new_file_path, file_size, offset
        );
        idx_data.push(entry);
    }
    
    let mut idx_bytes = idx_data.join("\n").into_bytes();
    println!("[Szyfrowanie/encrypt_folder/idx_bytes :: LocalTime:{}]\n---> Index przed, Odczytane dane (pierwsze 5 bajtów):\n---> {:?}\n", Local::now().format("%H:%M:%S"), &idx_bytes[0..idx_bytes.len().min(50)]);

    if  toggle_encryption==1{
        xor::xor_encrypt_decrypt(&mut idx_bytes, password);
        index.write_all(&idx_bytes)?;
    }else{    index.write_all(&idx_bytes)?;}

    println!("[Szyfrowanie/encrypt_folder/idx_bytes :: LocalTime:{}]\n---> Index po, Odczytane dane: {:?}\n", Local::now().format("%H:%M:%S"), &idx_bytes[0..idx_bytes.len().min(50)]);

    if let Some(ref mut info_file) = info_file {
    // Zapis do pliku informacyjnego (Lua)
    writeln!(info_file, "-- Plik informacyjny dla data.dat \n\n\n\n\n\n\n\n\n\n")?;
    writeln!(info_file, "--  ██████╗███████╗██╗   ██╗████████╗ █████╗ ███╗   ██╗██╗███████╗    ██╗    ██╗███████╗██████╗ ██████╗  ██████╗ ███╗   ██╗██╗ ██████╗ ███╗   ██╗███████╗")?;
    writeln!(info_file, "-- ██╔════╝╚══███╔╝╚██╗ ██╔╝╚══██╔══╝██╔══██╗████╗  ██║██║██╔════╝    ██║    ██║╚══███╔╝██╔══██╗██╔══██╗██╔═══██╗████╗  ██║██║██╔═══██╗████╗  ██║██╔════╝")?;
    writeln!(info_file, "-- ██║       ███╔╝  ╚████╔╝    ██║   ███████║██╔██╗ ██║██║█████╗      ██║ █╗ ██║  ███╔╝ ██████╔╝██████╔╝██║   ██║██╔██╗ ██║██║██║   ██║██╔██╗ ██║█████╗")?;
    writeln!(info_file, "-- ██║      ███╔╝    ╚██╔╝     ██║   ██╔══██║██║╚██╗██║██║██╔══╝      ██║███╗██║ ███╔╝  ██╔══██╗██╔══██╗██║   ██║██║╚██╗██║██║██║   ██║██║╚██╗██║██╔══╝")?;
    writeln!(info_file, "-- ╚██████╗███████╗   ██║      ██║   ██║  ██║██║ ╚████║██║███████╗    ╚███╔███╔╝███████╗██████╔╝██║  ██║╚██████╔╝██║ ╚████║██║╚██████╔╝██║ ╚████║███████╗")?;
    writeln!(info_file, "-- ╚══════╝╚══════╝   ╚═╝      ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═╝╚══════╝     ╚═══╝╚═╝ ╚══════╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝")?;
    writeln!(info_file,"\n\n\n\n\n\n")?;
    if toggle_encryption==0{writeln!(info_file, "password = \"{}\"", password)?;} else {writeln!(info_file, "Nie zabezpieczone hasłem")?;}    
    writeln!(info_file, "files = {{")?;
    for (type_name, type_name_id, type_name_variant, type_name_variant_size,unique_id, new_file_path, file_size, offset) in &file_infos {
        let sanitized_file_path = sanitize_filename(new_file_path);
        writeln!(
            info_file,
            "    {{ type name = \"{}\",\n\t\t type name id = \"{}\",\n\t\t\t type name variant = \"{}\",\n\t\t\t\t type name size variant = \"{}\",\n\t\t\t\t\t unique id = \"{}\",\n\t\t\t\t\t\t size = {},\n\t\t\t\t\t\t\t offset = {} }},",
            type_name, type_name_id, type_name_variant, type_name_variant_size, unique_id, file_size, offset
        )?;
    }
    writeln!(info_file, "}}")?;
}

    println!(
        "[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> Pliki spakowane do: {}\n---> i zapisane indeksy w {}\n", Local::now().format("%H:%M:%S"),output_file, index_file);
    println!("[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> Dodatkowe info w pliku LUA\n", Local::now().format("%H:%M:%S"));

    Ok(())

}













const TEST_FOLDER: &str = "test_files/source_files";
const OUTPUT_FILE: &str = "test_files/after_compression/compressed_data.dat";
const INDEX_FILE: &str = "test_files/after_compression/compressed_data.idx";
const OUTPUT_FILE_U: &str = "test_files/after_compression/uncompressed_data.dat";
const INDEX_FILE_U: &str = "test_files/after_compression/uncompressed_data.idx";
const OUTPUT_FILE_SZ: &str = "test_files/after_compression/szyfrowana_data.dat";
const INDEX_FILE_SZ: &str = "test_files/after_compression/szyfrowana_data.idx";
const OUTPUT_FILE_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.dat";
const INDEX_FILE_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.idx";
const PASSWORD: &str = "CyCkI"; // Przykładowe hasło

#[cfg(test)]
mod tests_skompresowany {
    use super::*;
    use std::fs;



    #[test]
    fn test_encrypt_folder() {
        // Przygotowanie ścieżek
        let input_folder = TEST_FOLDER;
        let output_file = OUTPUT_FILE;
        let index_file = INDEX_FILE;

        // Wyczyść poprzednie dane, jeśli istnieją
        if Path::new(output_file).exists() {
            fs::remove_file(output_file).unwrap();
        }
        if Path::new(index_file).exists() {
            fs::remove_file(index_file).unwrap();
        }

        // Wywołanie funkcji encrypt_folder
        let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 0, 1, 3,false);
        match &result {
            Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
            Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
        }
        // Sprawdzenie, czy operacja zakończyła się sukcesem
        assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

        // Sprawdzenie, czy plik wyjściowy został utworzony
        assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

        // Sprawdzenie, czy plik indeksu został utworzony
        assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

        // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
        let output_metadata = fs::metadata(output_file).unwrap();
        assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

        let index_metadata = fs::metadata(index_file).unwrap();
        assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
    }
}
mod tests_nieskompresowane {
    use super::*;
    use std::fs;



    #[test]
    fn test_encrypt_folder() {
        // Przygotowanie ścieżek
        let input_folder = TEST_FOLDER;
        let output_file = OUTPUT_FILE_U;
        let index_file = INDEX_FILE_U;

        // Wyczyść poprzednie dane, jeśli istnieją
        if Path::new(output_file).exists() {
            fs::remove_file(output_file).unwrap();
        }
        if Path::new(index_file).exists() {
            fs::remove_file(index_file).unwrap();
        }

        // Wywołanie funkcji encrypt_folder
        let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 0, 0,3,false);
        match &result {
            Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
            Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
        }
        // Sprawdzenie, czy operacja zakończyła się sukcesem
        assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

        // Sprawdzenie, czy plik wyjściowy został utworzony
        assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

        // Sprawdzenie, czy plik indeksu został utworzony
        assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

        // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
        let output_metadata = fs::metadata(output_file).unwrap();
        assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

        let index_metadata = fs::metadata(index_file).unwrap();
        assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
    }
}

mod tests_szyfrowany_skompresowany {
    use super::*;
    use std::fs;



    #[test]
    fn test_encrypt_folder() {
        // Przygotowanie ścieżek
        let input_folder = TEST_FOLDER;
        let output_file = OUTPUT_FILE_SZ;
        let index_file = INDEX_FILE_SZ;

        // Wyczyść poprzednie dane, jeśli istnieją
        if Path::new(output_file).exists() {
            fs::remove_file(output_file).unwrap();
        }
        if Path::new(index_file).exists() {
            fs::remove_file(index_file).unwrap();
        }

        // Wywołanie funkcji encrypt_folder
        let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 1, 1,3,false);
        match &result {
            Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
            Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
        }
        // Sprawdzenie, czy operacja zakończyła się sukcesem
        assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

        // Sprawdzenie, czy plik wyjściowy został utworzony
        assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

        // Sprawdzenie, czy plik indeksu został utworzony
        assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

        // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
        let output_metadata = fs::metadata(output_file).unwrap();
        assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

        let index_metadata = fs::metadata(index_file).unwrap();
        assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
    }
}

mod tests_szyfrowany_nieskompresowane {
    use super::*;
    use std::fs;



    #[test]
    fn test_encrypt_folder() {
        // Przygotowanie ścieżek
        let input_folder = TEST_FOLDER;
        let output_file = OUTPUT_FILE_SZ_U;
        let index_file = INDEX_FILE_SZ_U;

        // Wyczyść poprzednie dane, jeśli istnieją
        if Path::new(output_file).exists() {
            fs::remove_file(output_file).unwrap();
        }
        if Path::new(index_file).exists() {
            fs::remove_file(index_file).unwrap();
        }

        // Wywołanie funkcji encrypt_folder
        let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 1, 0,3,false);
        match &result {
            Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
            Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
        }
        // Sprawdzenie, czy operacja zakończyła się sukcesem
        assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

        // Sprawdzenie, czy plik wyjściowy został utworzony
        assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

        // Sprawdzenie, czy plik indeksu został utworzony
        assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

        // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
        let output_metadata = fs::metadata(output_file).unwrap();
        assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

        let index_metadata = fs::metadata(index_file).unwrap();
        assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
    }
}
