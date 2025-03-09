use std::path::PathBuf;
use image::DynamicImage;
use image::RgbImage;
use image::Rgb;
// use image::ImageFormat;
use image::GenericImageView;
use std::fs::create_dir_all;
use crate::utils::comunicat::komunikat;


pub fn polaczenie_rgb(foto:Vec<DynamicImage>,sciezka_wyjsciowa:&PathBuf,file_extensioun:u8,nazwa_pliku:&str,quality:u8,png_filter:u8){
    #[cfg(feature = "statystyki")]
    let nazwa_funkcji="polaczenie_rgb";
    if foto.len() != 3 {
        #[cfg(feature = "statystyki")]
        komunikat(nazwa_funkcji,"Oczekiwano dokładnie trzech obrazów RGB!");
        return;
    }

    let foto_r = &foto[0];
    let foto_g = &foto[1];
    let foto_b = &foto[2];
    let badfgdf = match file_extensioun{
        // 0 => "jpg",
        1 => "png",
        2 => "png",
        3 => "webp",
        4 => "webp",
        5 => "tga",
        _ => "jpg",
    };

    let (mut wysokosc, mut szerokosc)=(foto_r.height(), foto_r.width());
    for obraz in &foto {
        let (xx_wysokosc, xx_szerokosc) = (obraz.height(), obraz.width());
        if xx_wysokosc > wysokosc {
            wysokosc = xx_wysokosc;
        }
        if xx_szerokosc > szerokosc {
            szerokosc = xx_szerokosc;
        }
    }



    let mut poskladane_foto = RgbImage::new(szerokosc, wysokosc);


    // Łączymy obrazy
    #[cfg(feature = "statystyki")]
    komunikat(nazwa_funkcji,"Oczekiwano dokładnie trzech obrazów RGB!");

        for y in 0..wysokosc {
            for x in 0..szerokosc {
                // Pobieramy wartości pikseli z każdego obrazu
                let r = foto_r.get_pixel(x, y)[0]; // Czerwony
                let g = foto_g.get_pixel(x, y)[0]; // Zielony
                let b = foto_b.get_pixel(x, y)[0]; // Niebieski

                // Tworzymy nowy piksel z kanałami RGB
                let nowy_pikselsht = Rgb([r, g, b]); 

                // Ustawiamy piksel w nowym obrazie
                poskladane_foto.put_pixel(x, y, nowy_pikselsht);
            }
        }
            // },
    // }
    let file_name: &str = if nazwa_pliku.is_empty(){
        sciezka_wyjsciowa.file_name().unwrap().to_str().unwrap()
    }else{
        nazwa_pliku
    };

    let mut output_path = std::path::Path::new(sciezka_wyjsciowa).to_path_buf();
    // let file_name = sciezka_wyjsciowa.file_name().unwrap().to_str().unwrap();
    let output_file_name = format!("{}.{}", file_name,badfgdf); // np. image_1.png
    output_path.push(output_file_name);

    if let Some(parent) = output_path.parent() {
        create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
    }
    

    let gafdhsgfhfd = DynamicImage::ImageRgb8(poskladane_foto);
    crate::utils::image_encoder::foto_enkodery(gafdhsgfhfd, output_path, file_extensioun, &quality, &png_filter);





}