// use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
// use std::fs::File;
use std::vec::Vec;
use rayon::iter::IntoParallelIterator;
use std::io::ErrorKind;
use rayon::iter::ParallelIterator;
// use walkdir::FilterEntry;
use std::thread;
use image::DynamicImage;
use std::time::Instant;
use crate::utils::comunicat::komunikat;
use crate::utils::arc_mutex_strip::{
    get_locked_data_pathbuf, get_locked_data_string, get_locked_data_u8
};
use crate::image_channel_bundler::{normalizacja_do_bw::rgb_to_bw, laczenie_zdjec::polaczenie_rgb};

pub fn image_channel_bundler(
    paths_arc: Arc<Mutex<Vec<PathBuf>>>,
    string_arc: Arc<Mutex<Vec<String>>>,
    u8_arc:Arc<Mutex<Vec<u8>>>
)->Result<Arc<std::sync::Mutex<std::vec::Vec<usize>>>, std::io::Error >{
    let nazwa_funkcji = "image_channel_bundler";

    let paths_arc_clone = Arc::clone(&paths_arc);
    let u8_arc_clone = Arc::clone(&u8_arc);
    let string_arc_clone = Arc::clone(&string_arc);
    komunikat(nazwa_funkcji,"zmierzam do nowego wątku\n");
    let nowy_watek = thread::spawn(move||-> Result<(usize,usize), std::io::Error>{
        komunikat(nazwa_funkcji,"jestem w nowym wątku!\n");

        let tajm_starto = Instant::now();

        let locked_paths = get_locked_data_pathbuf(&paths_arc_clone)?;
        let output_path = &locked_paths[3];
        let input_foto_vec: Vec<&PathBuf> = vec!(&locked_paths[0],&locked_paths[1],&locked_paths[2]);


        let locked_settings = get_locked_data_u8(&u8_arc_clone)?;
        // komunikat(format!("{:?}",locked_settings));
        let rozdzielczosc = locked_settings[0];
        let rozszerzenie = locked_settings[1];
        let ilosc_bitow = locked_settings[2];
        let rescale_filter = locked_settings[3];
        let jakosc = locked_settings[4];
        let png_filter = locked_settings[5];


        let locked_strings = get_locked_data_string(&string_arc_clone)?;
        let nazwa_pliku = &locked_strings[0];


        let procesowane_rgb = input_foto_vec.into_par_iter().map(|gsfhsgf| {
            komunikat(nazwa_funkcji,"jestem w iter!");
            let foto_do_skladania = match image::open(gsfhsgf.as_path()){
                Ok(f) => f,
                Err(_) => return Err(std::io::Error::new(ErrorKind::InvalidInput, "Ścieżka nie jest prefiksem"))
            };

            let tutaj_obrabiamy_foto = match rgb_to_bw(foto_do_skladania, rozdzielczosc, ilosc_bitow,rescale_filter){
                Ok(s) => s,
                Err(_) =>return Err(std::io::Error::new(ErrorKind::InvalidData, "złe dane"))
            };

            Ok(tutaj_obrabiamy_foto)

        }).collect::<Vec<Result<_, _>>>();
        // println!("{:?}",procesowane_rgb);
        let all_successful = procesowane_rgb.iter().all(|result| result.is_ok());
        if all_successful {
            // Zbieramy tylko obrazy, ignorując błędy
            let valid_images: Vec<DynamicImage> = procesowane_rgb.into_iter()
                .filter_map(Result::ok) // Usuwamy błędy i zostawiamy tylko obrazy
                .collect();
            komunikat(nazwa_funkcji,"każdy osobny plik przetworzony ok, wsio w vec co ma byc, teraz do zapisu");
            // Uruchamiamy funkcję z przetworzonymi obrazami
            polaczenie_rgb(valid_images, output_path,rozszerzenie,nazwa_pliku,jakosc,png_filter);
            komunikat(nazwa_funkcji,"Obrazy połączone i zapisane.\n");

        } else {
            komunikat(nazwa_funkcji,"Nie wszystkie operacje zakończyły się sukcesem.\n");
        }







        let czas_trwania = tajm_starto.elapsed();
        let czas_trwania_sekundy: usize = czas_trwania.as_secs() as usize;
        let czas_trwania_milisekundy:usize = czas_trwania.subsec_millis() as usize;
        Ok((czas_trwania_sekundy,czas_trwania_milisekundy))
        
    });
    let przeprocesowane_pliki = nowy_watek.join().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other,"Wątek zakończył się błędem"))??;



    let sssss = przeprocesowane_pliki.0;
    let gdfgdf = przeprocesowane_pliki.1;


    Ok(Arc::new(Mutex::new(vec![
        sssss,
        gdfgdf
        ])))
}





#[cfg(test)]
mod test_laczenia{
    use super::*;
    #[test]
    fn testowanko_laczenia_fot(){
        let vec_path = Arc::new(Mutex::new(vec![PathBuf::from("test_files/laczenie_3_do_1/src/cedar_diffuse_1k.jpg"),PathBuf::from("test_files/laczenie_3_do_1/src/plastered-wall-03_diffuse_4k.png"),PathBuf::from("test_files/laczenie_3_do_1/src/teak_diffuse_1k.jpg"),PathBuf::from("test_files/laczenie_3_do_1/out")]));
        let vec_u8: Arc<Mutex<Vec<u8>>> =Arc::new(Mutex::new(vec![4,1,8,4,5,1]));
        let vec_string =Arc::new(Mutex::new(vec!["lol".to_string()]));
        let ajsgdfsdvkadvkbsfg = image_channel_bundler(vec_path,vec_string,vec_u8);
        println!("{:?}", ajsgdfsdvkadvkbsfg);
    }
}