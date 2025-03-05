use std::path::PathBuf;
use image::DynamicImage;
use image::RgbImage;
use image::Rgb;
use image::ImageFormat;
use image::GenericImageView;
use std::fs::create_dir_all;

pub fn polaczenie_rgb(foto:Vec<DynamicImage>,sciezka_wyjsciowa:&PathBuf,file_extensioun:u8,nazwa_pliku:&String,quality:u8,png_filter:u8){
    if foto.len() != 3 {
        eprintln!("Oczekiwano dokładnie trzech obrazów RGB!");
        return;
    }

    let foto_r = &foto[0];
    let foto_g = &foto[1];
    let foto_b = &foto[2];
    let (badfgdf,dandandan) = match file_extensioun{
        0 => ("jpg","rgb8"),
        1 => ("png","rgb8"),
        2 => ("png","rgb16"),
        3 => ("webp","rgb8"),
        4 => ("webp","rgb8"),
        5 => ("tga","rgb8"),
        _ => ("jpg","rgb8")
    };

    let (wysokosc, szerokosc)=(foto_r.height(), foto_r.width());

    let mut poskladane_foto = RgbImage::new(szerokosc, wysokosc);

    let file_name:&str;
    // Łączymy obrazy
    println!("laczenie obrazow");
    // match dandandan{ 
        // "rgb16"=>{
        //         for y in 0..wysokosc {
        //             for x in 0..szerokosc {
        //                 // Pobieramy wartości pikseli z każdego obrazu
        //                 let r = foto_r.get_pixel(x, y)[0] as u16; // Czerwony
        //                 let g = foto_g.get_pixel(x, y)[0]as u16; // Zielony
        //                 let b = foto_b.get_pixel(x, y)[0]as u16; // Niebieski

        //                 // Tworzymy nowy piksel z kanałami RGB
        //                 let nowy_pikselsht: Rgb<u16> = Rgb([r, g, b]); 

        //                 // Ustawiamy piksel w nowym obrazie
        //                 poskladane_foto.put_pixel(x, y, nowy_pikselsht);
        //             }
        //         }
        //     },
        // _=>{
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
    if nazwa_pliku.is_empty(){
        file_name = sciezka_wyjsciowa.file_name().unwrap().to_str().unwrap()
    }else{
        file_name = &nazwa_pliku
    }

    let mut output_path = std::path::Path::new(sciezka_wyjsciowa).to_path_buf();
    // let file_name = sciezka_wyjsciowa.file_name().unwrap().to_str().unwrap();
    let output_file_name = format!("{}.{}", file_name,badfgdf); // np. image_1.png
    output_path.push(output_file_name);

    if let Some(parent) = output_path.parent() {
        create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
    }
    
    // let dynamic_img: DynamicImage = match dandandan{ 
    //     "rgb8"=>DynamicImage::ImageRgb8(poskladane_foto),
    //     "rgb16"=>DynamicImage::ImageRgb16(poskladane_foto),
    //     _=>DynamicImage::ImageRgb8(poskladane_foto),
    // };

    let gafdhsgfhfd = DynamicImage::ImageRgb8(poskladane_foto);
    crate::utils::image_encoder::foto_enkodery(gafdhsgfhfd, output_path, file_extensioun, &quality, &png_filter);
    // match file_extensioun{
    //     0 => {
    //         if let Err(e) = poskladane_foto.save_with_format(output_path, ImageFormat::Jpeg) {
    //         eprintln!("Błąd zapisu obrazu jpg: {}", e);
    //         }else{println!("zapis do jpg");}
    //     },
    //     1 => {if let Err(e) = poskladane_foto.save_with_format(output_path, ImageFormat::Png) {
    //         eprintln!("Błąd zapisu obrazu png: {}", e);
    //         }else{println!("zapis do png");}
    //     },
    //     2 => {if let Err(e) = poskladane_foto.save_with_format(output_path, ImageFormat::WebP) {
    //         eprintln!("Błąd zapisu obrazu webp: {}", e);
    //         }else{println!("zapis do webp");}
    //     },
    //     3 => {if let Err(e) = poskladane_foto.save_with_format(output_path, ImageFormat::Tga) {
    //         eprintln!("Błąd zapisu obrazu tga: {}", e);
    //         }else{println!("zapis do tga");}
    //     },
        
    //     _ => {eprintln!("Błąd zapisu obrazu unknown:;");}
    // };




}