use std::{
    fs::create_dir_all, 
    path::Path, 
    sync::{Arc, Mutex}, 
    thread, 
    time::Instant
};
use chrono::Local;
use walkdir::WalkDir;
use rayon::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::utils::arc_mutex_strip::{
    get_locked_data_bool,
    get_locked_data_u8,
    get_locked_data_string
};
use crate::image_actions::konwersja::ogarniacz_fot;


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
    //universal fn setting!!!!!!!!!!!!!!!!!!!!!!!!!!1
    #[cfg(feature = "statystyki")]
    let func_id = "convert_images w image_actions::image_actions_main.rs" ;
    #[cfg(feature = "statystyki")]
    let current_time = Local::now();
    #[cfg(feature = "statystyki")]
    let formatted_time = current_time.format("%H:%M:%S").to_string();
    #[cfg(feature = "statystyki")]
    let formatted_time2 = formatted_time.clone();
    //    println!("[{func_id}]\n{formatted_time}   zaczynam konwersje {}",rozszerzenie_string);
    //***************************************************************
    #[cfg(feature = "statystyki")]
    println!("[{func_id}]\n{formatted_time}   rozpoczęcie funkcji konwersji");
    let time_start = Instant::now();
    // let processed_files_count = 0;
    // let created_files_count = 0;

    let arc_z_typ_clone = Arc::clone(&arc_z_typ);
    let arc_z_typ_jakosc_clone = Arc::clone(&arc_z_typ_jakosc);
    let arc_z_rozdzielczosc_clone = Arc::clone(&arc_z_rozdzielczosc);
    let arc_z_path_clone = Arc::clone(&arc_z_path);
    let arc_z_dodatkowymi_ustawieniami_clone = Arc::clone(&arc_z_dodatkowymi_ustawieniami);

    let nowy_watek = thread::spawn(move||-> Result<(usize,usize), std::io::Error>{
        #[cfg(feature = "statystyki")]
        println!("[{func_id}]\n{formatted_time}   rozpoczęcie nowego wątku");
        //potem zajmę się tym bo to z ui będzie szło.
        // let strip_arc_z_typ_clone = arc_z_typ_clone.lock().unwrap();

        
        // Teraz używasz funkcji i masz dane dostępne.
        let locked_data_z_typem = get_locked_data_bool(&arc_z_typ_clone)?;
        
        let czy_jpg = locked_data_z_typem[0];
        let czy_png = locked_data_z_typem[1];
        let czy_png_16 = locked_data_z_typem[2];
        let czy_webp = locked_data_z_typem[3];
        let czy_webp_lossy = locked_data_z_typem[4];
        let czy_tga = locked_data_z_typem[5];



        let locked_data_z_jakoscia = get_locked_data_u8(&arc_z_typ_jakosc_clone)?;

        let czy_jpg_jakosc = locked_data_z_jakoscia[0];
        let czy_png_jakosc = locked_data_z_jakoscia[1];
        let czy_png_16_jakosc = locked_data_z_jakoscia[2];
        let czy_webp_jakosc = locked_data_z_jakoscia[3];
        let czy_webp_lossy_jakosc = locked_data_z_jakoscia[4];
        let czy_tga_jakosc = locked_data_z_jakoscia[5];

        // let ilosc_bitow:u8 = match locked_data_z_typem.as_slice(){
        //     [true, ..] => 8,
        //     [false,true, ..] => 8,
        //     [false,false,true, ..] => 16,
        //     [false,false,false,true, ..] => 8,
        //     [false,false,false,false,true, ..] => 8,
        //     [false,false,false,false,false,true, ..] => 8,
        //     _=>8
        // };
        
        // let locked_data_z_rozmiarami = arc_z_rozdzielczosc_clone.lock().unwrap();
        let locked_data_z_rozmiarami = get_locked_data_bool(&arc_z_rozdzielczosc_clone)?;


        let locked_data_z_path = get_locked_data_string(&arc_z_path_clone)?;
        let folder_wejsciowy  = &locked_data_z_path[0];
        let folder_wyjsciowy = &locked_data_z_path[1];

        let strip_arc_z_dodatkowymi_ustawieniami_clone = get_locked_data_u8(&arc_z_dodatkowymi_ustawieniami_clone)?;
        let filtrowanie  = strip_arc_z_dodatkowymi_ustawieniami_clone[0];        
        let czy_alpha  = strip_arc_z_dodatkowymi_ustawieniami_clone[1];        
        let czy_alpha_kolor  = strip_arc_z_dodatkowymi_ustawieniami_clone[2];
        let png_filter =  strip_arc_z_dodatkowymi_ustawieniami_clone[3];

        let mut file_entries = Vec::new();




    // w tym template są rozszerzenia plików które mają być brane pod uwagę jak coś.
        for entry in WalkDir::new(folder_wejsciowy).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && matches_template(path) {
                file_entries.push(path.to_path_buf());
            }
        }

        create_dir_all(folder_wyjsciowy)?;





        let processed_results = file_entries.into_par_iter().map(|gsfhsgf| {
            #[cfg(feature = "statystyki")]
            println!("[{func_id}]\n{formatted_time}   rozpoczęcie iteracji wielowątkowej\n");
            let mut local_created_files_count = 0;
            

            let relative_path = gsfhsgf.strip_prefix(folder_wejsciowy).unwrap_or(&gsfhsgf);
            let output_path = std::path::Path::new(folder_wyjsciowy).join(relative_path);

            // Tworzenie odpowiednich podfolderów
            if let Some(parent) = output_path.parent() {
                create_dir_all(parent).unwrap();
            }
            let local_path = relative_path.parent().unwrap();
            

            if czy_png{
                let dflkjsg=ogarniacz_fot(vec![1,png_filter],&locked_data_z_rozmiarami,vec![&czy_png_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);
                // println!("zribione pliki z fn konwersja_do_png 8bit {}",dflkjsg);
                local_created_files_count = dflkjsg
            }

            if czy_png_16{
                let dflkjsg=ogarniacz_fot(vec![2,png_filter],&locked_data_z_rozmiarami,vec![&czy_png_16_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);

                // println!("zribione pliki z fn konwersja_do_png 16bit {}",dflkjsg);
                local_created_files_count = dflkjsg
            }
            
            if czy_jpg{
                let dflkjsg=ogarniacz_fot(vec![0,png_filter],&locked_data_z_rozmiarami,vec![&czy_jpg_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);

                // println!("zribione pliki z fn konwersja_do_jpg {}",dflkjsg);
                local_created_files_count = dflkjsg
            }
            if czy_webp{
                let dflkjsg=ogarniacz_fot(vec![3,png_filter],&locked_data_z_rozmiarami,vec![&czy_webp_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);

                // println!("zribione pliki z fn konwersja_do_webp {}",dflkjsg);
                local_created_files_count = dflkjsg
            }
            if czy_webp_lossy{
                let dflkjsg=ogarniacz_fot(vec![4,png_filter],&locked_data_z_rozmiarami,vec![&czy_webp_lossy_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);

                // println!("zribione pliki z fn konwersja_do_webp lossy {}",dflkjsg);
                local_created_files_count = dflkjsg
            }
            if czy_tga{
                let dflkjsg=ogarniacz_fot(vec![5,png_filter],&locked_data_z_rozmiarami,vec![&czy_tga_jakosc,&filtrowanie,&czy_alpha,&czy_alpha_kolor],&gsfhsgf,local_path,folder_wyjsciowy,local_created_files_count);

                // println!("zribione pliki z fn konwersja_do_tga {}",dflkjsg);
                local_created_files_count = dflkjsg
            }


            local_created_files_count
        }).collect::<Vec<_>>();

        let total_processed_files = processed_results.len();
        let total_created_files:i32 = processed_results.iter().sum();
        #[cfg(feature = "statystyki")]
        println!("[{func_id}]\n{formatted_time}   zakończenie iteracji wielowątkowej\n");

        Ok((total_processed_files, total_created_files as usize))
    });
    let przeprocesowane_pliki = nowy_watek.join().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other,"Wątek zakończył się błędem"))??;



    let czas_trwania = time_start.elapsed();
    let czas_trwania_sekundy: usize = czas_trwania.as_secs() as usize;
    let czas_trwania_milisekundy:usize = czas_trwania.subsec_millis() as usize;
    #[cfg(feature = "statystyki")]
    println!("[{func_id}]\n{formatted_time2}   Ogarnąłem {} plików zrobionych z {} plików w czasie {}.{}s",przeprocesowane_pliki.1,przeprocesowane_pliki.0,czas_trwania_sekundy,czas_trwania_milisekundy);
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
    fn remove_dir_contents(){
        let path:String = "test_files/test_image_out".to_string();
        println!("Najpierw super czyszczenie ;)");
        std::fs::remove_dir_all(&path).unwrap();

        println!("usunąłem folder {}",&path);
        std::fs::create_dir(&path).unwrap();

        println!("stworzyłem folder {}",&path);
    }

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
        // let rozdzielczosc=vec![rozd_4k,rozd_2k,rozd_1k];
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