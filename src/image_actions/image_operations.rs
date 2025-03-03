use std::{
    fs::{create_dir_all, File}, 
    path::{Path, PathBuf}, 
    sync::{Arc, Mutex}, 
    thread, 
    time::Instant
};
use walkdir::WalkDir;

use crate::image_actions::*;


const TEMPLEJT:&str = "images_converter";



fn matches_template(path: &Path) -> bool {
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let allowed_extensions = crate::encrypt_asset_setting::get_template_extensions(TEMPLEJT);
    
    allowed_extensions.is_empty() || allowed_extensions.contains(&ext.as_str())
}

pub fn convert_images(
    arc_z_typ: Arc<Mutex<Vec<bool>>>,
    arc_z_typ_jakosc: Arc<Mutex<Vec<u8>>>,
    arc_z_rozdzielczosc:Arc<Mutex<Vec<bool>>>,
    arc_z_path:Arc<Mutex<Vec<String>>>,
    arc_z_dodatkowymi_ustawieniami: Arc<Mutex<Vec<u8>>>
)-> Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error >{

    let time_start = Instant::now();
    let mut processed_files_count = 0;
    let mut created_files_count = 0;

    let arc_z_typ_clone = Arc::clone(&arc_z_typ);
    let arc_z_typ_jakosc_clone = Arc::clone(&arc_z_typ_jakosc);
    let arc_z_rozdzielczosc_clone = Arc::clone(&arc_z_rozdzielczosc);
    let arc_z_path_clone = Arc::clone(&arc_z_path);
    let arc_z_dodatkowymi_ustawieniami_clone = Arc::clone(&arc_z_dodatkowymi_ustawieniami);

    let nowy_watek = thread::spawn(move||-> Result<(usize,usize), std::io::Error>{

        //potem zajmę się tym bo to z ui będzie szło.
        let strip_arc_z_typ_clone = arc_z_typ_clone.lock().unwrap();
        let czy_jpg= strip_arc_z_typ_clone[0];
        let czy_png= strip_arc_z_typ_clone[1];
        let czy_png_16= strip_arc_z_typ_clone[2];
        let czy_webp= strip_arc_z_typ_clone[3];
        let czy_webp_lossy= strip_arc_z_typ_clone[4];
        let czy_tga= strip_arc_z_typ_clone[5];

        let strip_arc_z_typ_jakosc_clone = arc_z_typ_jakosc_clone.lock().unwrap();
        let czy_jpg_jakosc = strip_arc_z_typ_jakosc_clone[0];
        let czy_png_jakosc = strip_arc_z_typ_jakosc_clone[1];
        let czy_png_16_jakosc = strip_arc_z_typ_jakosc_clone[2];
        let czy_webp_jakosc = strip_arc_z_typ_jakosc_clone[3];
        let czy_webp_lossy_jakosc = strip_arc_z_typ_jakosc_clone[4];
        let czy_tga_jakosc = strip_arc_z_typ_jakosc_clone[5];
        
        let strip_arc_z_rozdzielczosc_clone = arc_z_rozdzielczosc_clone.lock().unwrap();


        let strip_arc_z_path_clone = arc_z_path_clone.lock().unwrap();
        let folder_wejsciowy  = &strip_arc_z_path_clone[0];
        let folder_wyjsciowy = &strip_arc_z_path_clone[1];

        let strip_arc_z_dodatkowymi_ustawieniami_clone = arc_z_dodatkowymi_ustawieniami_clone.lock().unwrap();
        let filtrowanie  = strip_arc_z_dodatkowymi_ustawieniami_clone[0];        
        let czy_alpha  = strip_arc_z_dodatkowymi_ustawieniami_clone[1];        
        let czy_alpha_kolor  = strip_arc_z_dodatkowymi_ustawieniami_clone[2];
        let png_filter =  strip_arc_z_dodatkowymi_ustawieniami_clone[3];

        let mut file_entries = Vec::new();



    // w tym template są rozszerzenia plików które mają być brane pod uwagę jak coś.
        for entry in WalkDir::new(&folder_wejsciowy).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && matches_template(path) {
                file_entries.push(path.to_path_buf());
            }
        }
        create_dir_all(&folder_wyjsciowy)?;





        for gsfhsgf in &file_entries {
            processed_files_count += 1;
            

            let relative_path = gsfhsgf.strip_prefix(&folder_wejsciowy).unwrap_or(gsfhsgf);
            let output_path = std::path::Path::new(&folder_wyjsciowy).join(relative_path);

            // Tworzenie odpowiednich podfolderów
            if let Some(parent) = output_path.parent() {
                create_dir_all(parent)?;
            }
            let local_path = relative_path.parent().unwrap_or_else(|| Path::new(""));


            if czy_png{
                let dflkjsg = konwersja::konwersja_do_png(&strip_arc_z_rozdzielczosc_clone,&czy_png_jakosc,&filtrowanie,8,&czy_alpha,&czy_alpha_kolor, &gsfhsgf, &local_path,&folder_wyjsciowy,created_files_count,png_filter);
                println!("zribione pliki z fn konwersja_do_png 8bit {}",dflkjsg);
                created_files_count = dflkjsg
            }

            if czy_png_16{
                let dflkjsg = konwersja::konwersja_do_png(&strip_arc_z_rozdzielczosc_clone,&czy_png_16_jakosc,&filtrowanie, 16,&czy_alpha,&czy_alpha_kolor,&gsfhsgf,&local_path, &folder_wyjsciowy,created_files_count,png_filter);
                println!("zribione pliki z fn konwersja_do_png 16bit {}",dflkjsg);
                created_files_count = dflkjsg
            }
            
            if czy_jpg{
                let dflkjsg = konwersja::konwersja_do_jpg(&strip_arc_z_rozdzielczosc_clone,&czy_jpg_jakosc, &filtrowanie,8,&czy_alpha,&czy_alpha_kolor,&gsfhsgf,&local_path, &folder_wyjsciowy,created_files_count);
                println!("zribione pliki z fn konwersja_do_jpg {}",dflkjsg);
                created_files_count = dflkjsg
            }
            if czy_webp{
                let dflkjsg = konwersja::konwersja_do_webp(&strip_arc_z_rozdzielczosc_clone,&czy_webp_jakosc, &filtrowanie,8,&czy_alpha,&czy_alpha_kolor,&gsfhsgf,&local_path, &folder_wyjsciowy,created_files_count);
                println!("zribione pliki z fn konwersja_do_webp {}",dflkjsg);
                created_files_count = dflkjsg
            }
            if czy_webp_lossy{
                let dflkjsg = konwersja::konwersja_do_webp_lossy(&strip_arc_z_rozdzielczosc_clone,&czy_webp_lossy_jakosc, &filtrowanie,8,&czy_alpha,&czy_alpha_kolor,&gsfhsgf,&local_path, &folder_wyjsciowy,created_files_count);
                println!("zribione pliki z fn konwersja_do_webp lossy {}",dflkjsg);
                created_files_count = dflkjsg
            }
            if czy_tga{
                let dflkjsg = konwersja::konwersja_do_tga(&strip_arc_z_rozdzielczosc_clone,&czy_tga_jakosc, &filtrowanie,8,&czy_alpha,&czy_alpha_kolor,&gsfhsgf,&local_path, &folder_wyjsciowy,created_files_count);
                println!("zribione pliki z fn konwersja_do_tga {}",dflkjsg);
                created_files_count = dflkjsg
            }


        }



        Ok((processed_files_count,created_files_count as usize))
    });
    let przeprocesowane_pliki = nowy_watek.join().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other,"Wątek zakończył się błędem"))??;



    let czas_trwania = time_start.elapsed();
    let czas_trwania_sekundy: usize = czas_trwania.as_secs() as usize;
    let czas_trwania_milisekundy:usize = czas_trwania.subsec_millis() as usize;
    println!("Ogarnąłem {}plików zrobionych z {} plików w czasie {}.{}s",przeprocesowane_pliki.1,przeprocesowane_pliki.0,czas_trwania_sekundy,czas_trwania_milisekundy);
    Ok(Arc::new(Mutex::new(vec![
        przeprocesowane_pliki.1,
        przeprocesowane_pliki.0,
        czas_trwania_sekundy,
        czas_trwania_milisekundy
        ])))
}





#[cfg(test)]
mod tests_skompresowany {
    use super::*;

    #[test]
    fn test_image_conversion(){
        let input_folder:String = "test_files/test_image_src".to_string();
        let output_folder:String = "test_files/test_image_out".to_string();
        let wybor_jpg = true;
        let wybor_png = true;
        let wybor_png_16 = true;
        let wybor_webp = true;
        let wybor_webp_lossy = true;
        let wybor_tga = true;
        let wybor_jpg_jakosc: u8 = 63;
        let wybor_png_jakosc:u8 = 70;
        let wybor_png_16_jakosc:u8 = 70;
        let wybor_webp_jakosc:u8 = 80;
        let wybor_webp_lossy_jakosc:u8 = 80;
        let wybor_tga_jakosc:u8 = 80;
        let rozd_4k = false;
        let rozd_2k = true;
        let rozd_1k = true;
        let rozdzielczosc=vec![rozd_4k,rozd_2k,rozd_1k];
        let test_filtrowanie:u8 = 5;
        let test_alpha:u8 = 1;
        let test_alpha_kolor:u8 = 3;
        let test_arc_bool_rozdzielczosc = Arc::new(Mutex::new(vec![false,false,rozd_4k, rozd_2k,rozd_1k,false,false,false]));
        let test_arc_bool_typ = Arc::new(Mutex::new(vec![wybor_jpg, wybor_png, wybor_png_16,wybor_webp,wybor_webp_lossy,wybor_tga]));
        let test_arc_bool_typ_jakosc = Arc::new(Mutex::new(vec![wybor_jpg_jakosc, wybor_png_jakosc, wybor_png_16_jakosc,wybor_webp_jakosc,wybor_webp_lossy_jakosc,wybor_tga_jakosc]));
        let test_arc_paths= Arc::new(Mutex::new(vec![input_folder,output_folder]));
        let test_arc_dodatkowe_ustawienia= Arc::new(Mutex::new(vec![test_filtrowanie,test_alpha,test_alpha_kolor,3]));

        let afdhfsgh =convert_images(
            test_arc_bool_typ,
            test_arc_bool_typ_jakosc,
            test_arc_bool_rozdzielczosc,
            test_arc_paths,
            test_arc_dodatkowe_ustawienia
        );
        println!("{:?}", afdhfsgh);
    }
}