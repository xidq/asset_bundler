use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
// use std::thread::spawn;
use std::time::{/*Duration,*/Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::ErrorKind;
use crate::utils::arc_mutex_strip::{
    get_locked_data_bool,
    get_locked_data_u8,
    get_locked_data_string
};

// use egui::mutex::Mutex;
use walkdir::WalkDir;
use zstd::*;
use chrono::offset::Local;
// use rand::*;
use rand::Rng;
use rand::distr::Alphanumeric;
use std::collections::HashMap;
// use tokio::io::AsyncWriteExt;
// use std::time::Instant;
// use std::thread;
// use tokio::*;
// use pbkdf2::pbkdf2;
// use sha2::Sha256;
// use xor_utils::*;
// use aes::Aes256; // Użyjemy AES-256
// use chacha20poly1305::*;
// use chacha20poly1305::aead::generic_array::sequence::GenericSequence;
// use rand::rngs::OsRng;

use crate::moi_chacha20;
use crate::encrypt_filetype;
// use crate::xor;
use crate::encrypt_asset_setting;
// use crate::zstd;
// use crate::chacha20;



//  █████  ██████  ██████  ██ ████████ ██  ██████  ███    ██  █████  ██      
// ██   ██ ██   ██ ██   ██ ██    ██    ██ ██    ██ ████   ██ ██   ██ ██      
// ███████ ██   ██ ██   ██ ██    ██    ██ ██    ██ ██ ██  ██ ███████ ██      
// ██   ██ ██   ██ ██   ██ ██    ██    ██ ██    ██ ██  ██ ██ ██   ██ ██      
// ██   ██ ██████  ██████  ██    ██    ██  ██████  ██   ████ ██   ██ ███████ 
                                                                          
                                                                          
// ███████ ██    ██ ███    ██  ██████ ████████ ██  ██████  ███    ██ ███████ 
// ██      ██    ██ ████   ██ ██         ██    ██ ██    ██ ████   ██ ██      
// █████   ██    ██ ██ ██  ██ ██         ██    ██ ██    ██ ██ ██  ██ ███████ 
// ██      ██    ██ ██  ██ ██ ██         ██    ██ ██    ██ ██  ██ ██      ██ 
// ██       ██████  ██   ████  ██████    ██    ██  ██████  ██   ████ ███████ 

// as u can see, function generating unique id for each file,
// can be helpful when u have multiple similar named files or something, idk.
fn generate_unique_id() -> String {
    let mut rng = rand::rng();
    // Generowanie losowego identyfikatora 12-znakowego
    (0..12)
        .map(|_| rng.sample(Alphanumeric))
        .map(char::from)
        .collect()
}

// here is function as u can see
pub fn sanitize_filename(filename: &str) -> String {
    let mut sanitized = filename
        .replace([' ', '?', '%'], "-");
        let parts: Vec<&str> = sanitized.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        sanitized = parts[1].replace('.', "-") + "." + parts[0]; // Zmieniamy kropki przed rozszerzeniem
    }
    
    sanitized
}

// function template check, templates are in encrypt_asset_settings.rs.
// that funtion takes path and template selected from ui to perform actions.
// Long story short here we can see assigning type in it's natural habitat or smth.
fn matches_template(path: &Path, template: &str) -> bool {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let allowed_extensions = encrypt_asset_setting::get_template_extensions(template);
    
    allowed_extensions.is_empty() || allowed_extensions.contains(&ext.as_str())
}



// here's another function, taking fisrts thing b4 '_' in file name
fn get_type_id(path: &Path) -> String {
    let base = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or("");

    let san_base = sanitize_filename(base);
    san_base.split('_')
        .next()
        .unwrap_or("")
        .to_string()
}

// takin' second thing, that one between '_'
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

// and here we go, another one, 3rd to be exact
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

// here's main encoding function, ui is freezing while function is runnning, but it works! (kind of...)
// Encryption isn't working for now tho,
// after decompressing compressed files i can see few additional bytes at the end or something in text files.
pub fn encrypt_folder(
    arc_z_str: Arc<Mutex<Vec<String>>>,
    arc_z_u8: Arc<Mutex<Vec<u8>>>,
    arc_z_bool:Arc<Mutex<Vec<bool>>>
) -> Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, io::Error > {
    // let (tx, rx) = mpsc::channel();
    // for statistics
    let time_start = Instant::now();
    let mut processed_files_count = 0;
    // let mut info_file: Option<File> = None;

        // Klonujemy `Arc`, aby móc używać ich w różnych wątkach
    let arc_z_u8_clone = Arc::clone(&arc_z_u8);
    let arc_z_bool_clone = Arc::clone(&arc_z_bool);
    let arc_z_str_clone = Arc::clone(&arc_z_str);

    let nowy_watek = thread::spawn(move||-> Result<usize, io::Error>{
        let strip_arc_z_u8_clone = get_locked_data_u8(&arc_z_u8_clone)?;
        let toggle_encryption = strip_arc_z_u8_clone[0];
        let toggle_compression = strip_arc_z_u8_clone[1];
        let poziom_kompresji = strip_arc_z_u8_clone[2];

        // Odblokowujemy Mutex w `arc_z_bool_clone`
        let strip_arc_z_bool_clone = get_locked_data_bool(&arc_z_bool_clone)?;
        let debug_create_lua_file = strip_arc_z_bool_clone[0];

        // Odblokowujemy Mutex w `arc_z_str_clone`
        let strip_arc_z_str_clone = get_locked_data_string(&arc_z_str_clone)?;
        let input_folder = &strip_arc_z_str_clone[0];
        let output_file = &strip_arc_z_str_clone[1];
        let index_file = &strip_arc_z_str_clone[2];
        let template = &strip_arc_z_str_clone[3];
        let password = &strip_arc_z_str_clone[4];





    // for creating binary file
        let mut output = File::create(output_file)
            .map_err(|e| {
                eprintln!("[Szyfrowanie/encrypt_folder/output :: LocalTime:{}]\n---> Błąd przy tworzeniu pliku wyjściowego {}: {}\n", 
                    Local::now().format("%H:%M:%S"), output_file, e);
                e

        })?;
        //for creating index file, where u can find index, offsets, lenghts etc
        let mut index = File::create(index_file)  
                .map_err(|e| {
            eprintln!("[Szyfrowanie/encrypt_folder/index :: LocalTime:{}]\n---> Błąd przy tworzeniu pliku indeksu {}: {}\n", 
                Local::now().format("%H:%M:%S"), index_file, e);
            e

        })?;

        // optional lua file just bcoz i can
        let mut info_file: Option<File> = None;
        if debug_create_lua_file {
            info_file = Some(File::create(format!("{}{}", output_file, ".lua"))
                .map_err(|e| {
                    eprintln!("[Szyfrowanie/encrypt_folder/info_file] :: Błąd przy tworzeniu pliku info: {}", e);
                    e
                })?);
        }
        
        let mut file_entries = Vec::new();

        // here function is taking files according to template, ya know, sometimes that can be somewhat picky
        for entry in WalkDir::new(input_folder).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && matches_template(path, template) {
                file_entries.push(path.to_path_buf());
            }
        }
        // ya, that's me
        let signature = "created by Patryk Jerzak, ALL RIGHTS RESERVED";
        let signature_bytes = signature.as_bytes();

        output.write_all(&(file_entries.len() as u32).to_le_bytes())?; // putting something at the start of binary file
        output.write_all(signature_bytes)?; // and here putting signature

        let mut file_infos = Vec::new();
        let header_size = 4 + signature_bytes.len() as u64; // <-- Dynamic header lenght
        let mut current_offset = header_size; 



        //  ██████  ██████  ██      ██      ███████  ██████ ████████ 
        // ██      ██    ██ ██      ██      ██      ██         ██    
        // ██      ██    ██ ██      ██      █████   ██         ██    
        // ██      ██    ██ ██      ██      ██      ██         ██    
        //  ██████  ██████  ███████ ███████ ███████  ██████    ██    
                                                                
                                                                
        // ██████   █████  ████████  █████                           
        // ██   ██ ██   ██    ██    ██   ██                          
        // ██   ██ ███████    ██    ███████                          
        // ██   ██ ██   ██    ██    ██   ██                          
        // ██████  ██   ██    ██    ██   ██ 

        // for loop for for in for for for getting data of each file
        for path in &file_entries {

            let file_path = match path.strip_prefix(input_folder) {
                Ok(p) => match p.to_str() {
                    Some(s) => s.replace("\\", "/"),
                    None => return Err(io::Error::new(ErrorKind::InvalidData, "Nie udało się przekształcić ścieżki na tekst")),
                },
                Err(_) => return Err(io::Error::new(ErrorKind::InvalidInput, "Ścieżka nie jest prefiksem")),
            };
            

            let unique_id = generate_unique_id(); 
            let type_name = encrypt_filetype::get_type(path);
            let type_name_id = get_type_id(path);
            let type_name_variant = get_type_variant(path);
            let type_name_variant_size = get_type_variant_size(path);

            // so, here we can see filling blank spaces with 0-s, just for organisation purposes
            let type_name_id = if type_name_id.is_empty() { "0".to_string() } else { type_name_id };
            let type_name_variant = if type_name_variant.is_empty() { "0".to_string() } else { type_name_variant };
            let type_name_variant_size = if type_name_variant_size.is_empty() { "0".to_string() } else { type_name_variant_size };


            let metadata = fs::metadata(path)?;
            let original_file_size = metadata.len();

            file_infos.push((type_name.clone(), type_name_id.clone(), type_name_variant.clone(), type_name_variant_size.clone(), unique_id.clone(), file_path.clone(), original_file_size, current_offset));
            processed_files_count += 1;
            current_offset += original_file_size;
        }

        let mut current_offset = header_size;



        // ███████ ██ ██      ███████                                                      
        // ██      ██ ██      ██                                                           
        // █████   ██ ██      █████                                                        
        // ██      ██ ██      ██                                                           
        // ██      ██ ███████ ███████                                                      
                                                                                        
                                                                                        
        //  ██████  ██████  ███████ ██████   █████  ████████ ██  ██████  ███    ██ ███████ 
        // ██    ██ ██   ██ ██      ██   ██ ██   ██    ██    ██ ██    ██ ████   ██ ██      
        // ██    ██ ██████  █████   ██████  ███████    ██    ██ ██    ██ ██ ██  ██ ███████ 
        // ██    ██ ██      ██      ██   ██ ██   ██    ██    ██ ██    ██ ██  ██ ██      ██ 
        //  ██████  ██      ███████ ██   ██ ██   ██    ██    ██  ██████  ██   ████ ███████

        // and here u can see beating those files to desintegrate into binary format
        for xoxo in &mut file_infos {

            let (file_path, original_file_size, _) = (xoxo.5.clone(),xoxo.6, xoxo.7);
            // {
            //     let entry = &file_infos[i];
            //     (entry.5.clone(), entry.6, entry.7)
            // };
        
            let full_path = format!("{}/{}", &input_folder, file_path);
            let mut file = File::open(&full_path)?;
        
            // reading original size of file b4 compression or smth
            let mut buffer = vec![0; original_file_size as usize];
            file.read_exact(&mut buffer)?;


            println!("[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> dat po, Odczytane dane (pierwsze 50 bajtów): \n---> {:?}\n", 
                Local::now().format("%H:%M:%S"), &buffer[0..buffer.len().min(50)]);

                // let processed_data = Vec::new();
                // let przeprocesowane = Vec::new();

                let processed_data =if toggle_compression == 1 {
                let mut compressed_data = Vec::new();{

                    let mut encoder = Encoder::new(&mut compressed_data, poziom_kompresji as i32)?;
                    encoder.multithread(4)?;
                    encoder.include_magicbytes(true)?;
                    encoder.write_all(&buffer)?;
                    encoder.write_all(&(file_entries.len() as u32).to_le_bytes())?;
                    encoder.finish()?;

                }

                compressed_data
            
            }else{

                buffer.clone()
            
            };

            let przeprocesowane = if toggle_encryption == 1 {

                moi_chacha20::encrypt_chacha(&processed_data, password)

            } else {
                processed_data.clone()
            };

            xoxo.6 = przeprocesowane.len() as u64; // new size
            xoxo.7 = current_offset; // new offset

            current_offset += przeprocesowane.len() as u64;
            output.write_all(&przeprocesowane)?
        
        } // end of: for i in 0..file_infos.len()



        // ██ ███    ██ ██████  ███████ ██   ██ 
        // ██ ████   ██ ██   ██ ██       ██ ██  
        // ██ ██ ██  ██ ██   ██ █████     ███   
        // ██ ██  ██ ██ ██   ██ ██       ██ ██  
        // ██ ██   ████ ██████  ███████ ██   ██ 
                                            
                                            
        // ███████ ██ ██      ███████           
        // ██      ██ ██      ██                
        // █████   ██ ██      █████             
        // ██      ██ ██      ██                
        // ██      ██ ███████ ███████           

        // putting things into idx(index) file
        let mut idx_data = Vec::new();
        for (

            type_name, 
            type_name_id, 
            type_name_variant, 
            type_name_variant_size, 
            unique_id, 
            new_file_path, 
            file_size, 
            offset
            
        ) in &file_infos {

            // let sanitized_file_path = sanitize_filename(new_file_path);
            println!("[Szyfrowanie/encrypt_folder/idx_data :: LocalTime:{}]\n---> Plik: {},\n---> Rozmiar: {},\n---> Offset: {}\n", 
                Local::now().format("%H:%M:%S"), new_file_path,file_size, offset);

            let entry = format!(

                "{} {} {} {} {} {} {} {}", 
                type_name, type_name_id, type_name_variant, type_name_variant_size, unique_id, new_file_path, file_size, offset

            );

            idx_data.push(entry);
        }
        

        let idx_bytes = idx_data.join("\n").into_bytes();

        println!("[Szyfrowanie/encrypt_folder/idx_bytes :: LocalTime:{}]\n---> Index przed, Odczytane dane (pierwsze 5 bajtów):\n---> {:?}\n", 
            Local::now().format("%H:%M:%S"), &idx_bytes[0..idx_bytes.len().min(50)]);

        // xor not used, previous tests failed in files other than text files, so, big nono for me for now

        // if  toggle_encryption==1{
        //     xor::xor_encrypt_decrypt(&mut idx_bytes, password);
        //     index.write_all(&idx_bytes)?;
        // }else{    index.write_all(&idx_bytes)?;}

        // saving idx
        index.write_all(&idx_bytes)?;

        println!("[Szyfrowanie/encrypt_folder/idx_bytes :: LocalTime:{}]\n---> Index po, Odczytane dane: {:?}\n", 
            Local::now().format("%H:%M:%S"), &idx_bytes[0..idx_bytes.len().min(50)]);



        // ██      ██    ██  █████    
        // ██      ██    ██ ██   ██   
        // ██      ██    ██ ███████   
        // ██      ██    ██ ██   ██   
        // ███████  ██████  ██   ██   
                                    
                                    
        // ███████ ██ ██      ███████ 
        // ██      ██ ██      ██      
        // █████   ██ ██      █████   
        // ██      ██ ██      ██      
        // ██      ██ ███████ ███████         

        //lua file
        if let Some(ref mut info_file) = info_file {

            writeln!(info_file, "-- Plik informacyjny dla {}\n\n\n\n\n\n\n\n\n\n",output_file)?;
            writeln!(info_file, "  ██████╗███████╗██╗   ██╗████████╗ █████╗ ███╗   ██╗██╗███████╗    ██╗    ██╗███████╗██████╗ ██████╗  ██████╗ ███╗   ██╗██╗ ██████╗ ███╗   ██╗███████╗")?;
            writeln!(info_file, " ██╔════╝╚══███╔╝╚██╗ ██╔╝╚══██╔══╝██╔══██╗████╗  ██║██║██╔════╝    ██║    ██║╚══███╔╝██╔══██╗██╔══██╗██╔═══██╗████╗  ██║██║██╔═══██╗████╗  ██║██╔════╝")?;
            writeln!(info_file, " ██║       ███╔╝  ╚████╔╝    ██║   ███████║██╔██╗ ██║██║█████╗      ██║ █╗ ██║  ███╔╝ ██████╔╝██████╔╝██║   ██║██╔██╗ ██║██║██║   ██║██╔██╗ ██║█████╗")?;
            writeln!(info_file, " ██║      ███╔╝    ╚██╔╝     ██║   ██╔══██║██║╚██╗██║██║██╔══╝      ██║███╗██║ ███╔╝  ██╔══██╗██╔══██╗██║   ██║██║╚██╗██║██║██║   ██║██║╚██╗██║██╔══╝")?;
            writeln!(info_file, " ╚██████╗███████╗   ██║      ██║   ██║  ██║██║ ╚████║██║███████╗    ╚███╔███╔╝███████╗██████╔╝██║  ██║╚██████╔╝██║ ╚████║██║╚██████╔╝██║ ╚████║███████╗")?;
            writeln!(info_file, " ╚══════╝╚══════╝   ╚═╝      ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═╝╚══════╝     ╚═══╝╚═╝ ╚══════╝╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝")?;
            writeln!(info_file,"\n\n\n\n\n\n")?;

            if toggle_encryption==0{writeln!(info_file, "password = \"{}\"", password)?;} else {writeln!(info_file, "Nie zabezpieczone hasłem")?;}    

            writeln!(info_file, "files = {{")?;

            // Grupa plików według `type_name`
            let mut file_groups: HashMap<String, Vec<_>> = HashMap::new();
            
            // Grupowanie plików na podstawie `type_name`
            for (

                type_name, 
                type_name_id, 
                type_name_variant, 
                type_name_variant_size, 
                unique_id, 
                new_file_path, 
                file_size, 
                offset

            ) in &file_infos {

                file_groups.entry(type_name.clone()) // creatin' or modyfyin' group
                    .or_insert_with(Vec::new)
                    .push((type_name_id, type_name_variant, type_name_variant_size, unique_id, new_file_path, file_size, offset));

            }

            // Grouping things according to `type_name`, just my prefference
            for (type_name, group) in file_groups {
                writeln!(info_file, "    {}:", type_name)?; // Nagłówek dla grupy
                for (type_name_id, type_name_variant, type_name_variant_size, unique_id, new_file_path, file_size, offset) in group {
                    
                    let _sanitized_file_path = sanitize_filename(new_file_path);
                    writeln!(
                        info_file,
                        "        {} {{ \n\t\t\t type name variant = \"{}\",\n\t\t\t\t type name size variant = \"{}\",\n\t\t\t\t\t unique id = \"{}\",\n\t\t\t\t\t\t size = {},\n\t\t\t\t\t\t\t offset = {} }},",
                        type_name_id, type_name_variant, type_name_variant_size, unique_id, file_size, offset
                    )?;

                }
            }





        }
        println!("[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> Pliki spakowane do: {}\n---> i zapisane indeksy w {}\n", Local::now().format("%H:%M:%S"),
        &output_file, &index_file);
        // tx.send(processed_files_count).unwrap();
        Ok(processed_files_count)
    });
    
    let przeprocesowane_pliki = nowy_watek.join().map_err(|_| io::Error::new(io::ErrorKind::Other,"Wątek zakończył się błędem"))??;



    println!("[Szyfrowanie/encrypt_folder :: LocalTime:{}]\n---> Dodatkowe info w pliku LUA\n",
        Local::now().format("%H:%M:%S"));

    let czas_trwania = time_start.elapsed();
    let czas_trwania_sekundy: usize = czas_trwania.as_secs() as usize;
    let czas_trwania_milisekundy:usize = czas_trwania.subsec_millis() as usize;

    Ok(Arc::new(Mutex::new(vec![przeprocesowane_pliki,czas_trwania_sekundy, czas_trwania_milisekundy])))

}









// ████████ ███████ ███████ ████████ ███████ ██████  
//    ██    ██      ██         ██    ██           ██ 
//    ██    █████   ███████    ██    ███████   ▄███  
//    ██    ██           ██    ██         ██   ▀▀    
//    ██    ███████ ███████    ██    ███████   ██





// #[cfg(test)]
// mod tests_skompresowany {
//     use super::*;
//     use std::fs;
//     const TEST_FOLDER: &str = "test_files/source_files";
//     const OUTPUT_FILE: &str = "test_files/after_compression/compressed_data.dat";
//     const INDEX_FILE: &str = "test_files/after_compression/compressed_data.idx";
//     const PASSWORD: &str = "CyCkI"; // Przykładowe hasło



//     #[test]
//     fn test_encrypt_folder() {
//         // Przygotowanie ścieżek
//         let input_folder = TEST_FOLDER;
//         let output_file = OUTPUT_FILE;
//         let index_file = INDEX_FILE;

//         // Wyczyść poprzednie dane, jeśli istnieją
//         if Path::new(output_file).exists() {
//             fs::remove_file(output_file).unwrap();
//         }
//         if Path::new(index_file).exists() {
//             fs::remove_file(index_file).unwrap();
//         }

//         // Wywołanie funkcji encrypt_folder
//         let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 0, 1, 3,false);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }
//         // Sprawdzenie, czy operacja zakończyła się sukcesem
//         assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

//         // Sprawdzenie, czy plik wyjściowy został utworzony
//         assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

//         // Sprawdzenie, czy plik indeksu został utworzony
//         assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

//         // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
//         let output_metadata = fs::metadata(output_file).unwrap();
//         assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

//         let index_metadata = fs::metadata(index_file).unwrap();
//         assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
//     }
// }
// #[cfg(test)]
// mod tests_nieskompresowane {
//     use super::*;
//     use std::fs;
//     const TEST_FOLDER: &str = "test_files/source_files";
//     const OUTPUT_FILE_U: &str = "test_files/after_compression/uncompressed_data.dat";
//     const INDEX_FILE_U: &str = "test_files/after_compression/uncompressed_data.idx";

//     const PASSWORD: &str = "CyCkI"; // Przykładowe hasło


//     #[test]
//     fn test_encrypt_folder() {
//         // Przygotowanie ścieżek
//         let input_folder = TEST_FOLDER;
//         let output_file = OUTPUT_FILE_U;
//         let index_file = INDEX_FILE_U;

//         // Wyczyść poprzednie dane, jeśli istnieją
//         if Path::new(output_file).exists() {
//             fs::remove_file(output_file).unwrap();
//         }
//         if Path::new(index_file).exists() {
//             fs::remove_file(index_file).unwrap();
//         }

//         // Wywołanie funkcji encrypt_folder
//         let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 0, 0,3,false);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }
//         // Sprawdzenie, czy operacja zakończyła się sukcesem
//         assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

//         // Sprawdzenie, czy plik wyjściowy został utworzony
//         assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

//         // Sprawdzenie, czy plik indeksu został utworzony
//         assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

//         // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
//         let output_metadata = fs::metadata(output_file).unwrap();
//         assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

//         let index_metadata = fs::metadata(index_file).unwrap();
//         assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
//     }
// }
// #[cfg(test)]
// mod tests_szyfrowany_skompresowany {
//     use super::*;
//     use std::fs;
//     const TEST_FOLDER: &str = "test_files/source_files";
//     const OUTPUT_FILE_SZ: &str = "test_files/after_compression/szyfrowana_data.dat";
//     const INDEX_FILE_SZ: &str = "test_files/after_compression/szyfrowana_data.idx";
//     const PASSWORD: &str = "CyCkI"; // Przykładowe hasło



//     #[test]
//     fn test_encrypt_folder() {
//         // Przygotowanie ścieżek
//         let input_folder = TEST_FOLDER;
//         let output_file = OUTPUT_FILE_SZ;
//         let index_file = INDEX_FILE_SZ;

//         // Wyczyść poprzednie dane, jeśli istnieją
//         if Path::new(output_file).exists() {
//             fs::remove_file(output_file).unwrap();
//         }
//         if Path::new(index_file).exists() {
//             fs::remove_file(index_file).unwrap();
//         }

//         // Wywołanie funkcji encrypt_folder
//         let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 1, 1,3,false);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }
//         // Sprawdzenie, czy operacja zakończyła się sukcesem
//         assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

//         // Sprawdzenie, czy plik wyjściowy został utworzony
//         assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

//         // Sprawdzenie, czy plik indeksu został utworzony
//         assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

//         // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
//         let output_metadata = fs::metadata(output_file).unwrap();
//         assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

//         let index_metadata = fs::metadata(index_file).unwrap();
//         assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
//     }
// }
// #[cfg(test)]
// mod tests_szyfrowany_nieskompresowane {
//     use super::*;
//     use std::fs;
//     const TEST_FOLDER: &str = "test_files/source_files";
//     const OUTPUT_FILE_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.dat";
//     const INDEX_FILE_SZ_U: &str = "test_files/after_compression/szyfrowana_uncompressed_data.idx";
//     const PASSWORD: &str = "CyCkI"; // Przykładowe hasło



//     #[test]
//     fn test_encrypt_folder() {
//         // Przygotowanie ścieżek
//         let input_folder = TEST_FOLDER;
//         let output_file = OUTPUT_FILE_SZ_U;
//         let index_file = INDEX_FILE_SZ_U;

//         // Wyczyść poprzednie dane, jeśli istnieją
//         if Path::new(output_file).exists() {
//             fs::remove_file(output_file).unwrap();
//         }
//         if Path::new(index_file).exists() {
//             fs::remove_file(index_file).unwrap();
//         }

//         // Wywołanie funkcji encrypt_folder
//         let result = encrypt_folder(input_folder, output_file, index_file, "None", PASSWORD, 1, 0,3,false);
//         match &result {
//             Ok(_) => println!("Szyfrowanie zakończone sukcesem"),
//             Err(e) => eprintln!("Błąd podczas szyfrowania: {}", e),
//         }
//         // Sprawdzenie, czy operacja zakończyła się sukcesem
//         assert!(result.is_ok(), "Funkcja encrypt_folder nie zakończyła się pomyślnie");

//         // Sprawdzenie, czy plik wyjściowy został utworzony
//         assert!(Path::new(output_file).exists(), "Plik wyjściowy data.dat nie został utworzony");

//         // Sprawdzenie, czy plik indeksu został utworzony
//         assert!(Path::new(index_file).exists(), "Plik indeksu data.idx nie został utworzony");

//         // Można dodatkowo sprawdzić rozmiar pliku, by upewnić się, że dane zostały zapisane
//         let output_metadata = fs::metadata(output_file).unwrap();
//         assert!(output_metadata.len() > 0, "Plik wyjściowy data.dat jest pusty");

//         let index_metadata = fs::metadata(index_file).unwrap();
//         assert!(index_metadata.len() > 0, "Plik indeksu data.idx jest pusty");
//     }
// }
