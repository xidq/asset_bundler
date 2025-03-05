use image::GenericImageView;
use std::{fs::create_dir_all, path::{PathBuf,Path}};
use crate::image_actions::*;
// use webp::*;
// use chrono::NaiveTime;
use chrono::Local;
const RESOLUTION_MAPPING: [u32;10] =[
    16384,
    8192,
    4096, // 4K
    2048, // 2K
    1024, // 1K
    512,
    256,
    128,
    64,
    32
];







pub fn ogarniacz_fot(roz_png_fil:Vec<u8>,rozdzielczosc: &[bool], qua_fil_alp_alpc:Vec<&u8>, sciezka: &PathBuf,lokalny_path:&Path, output: &str,counter:i32)->i32{
        let mut created_files = counter;

    //universal fn setting!!!!!!!!!!!!!!!!!!!!!!!!!!1
    let func_id = "Ogarniacz Fot w image_actions::konwersja.rs" ;
    let current_time = Local::now();
    let formatted_time = current_time.format("%H:%M:%S").to_string();
    //***************************************************************

    let (rozszerzenie_string,rozszerzenie_do_pliku,definitywnie_alpha,rgb_only,bity):(&str,&str,&u8,bool,u8) = match roz_png_fil[0]{
        0=>("jpg","jpg",&0,true,8),
        1=>("png 8 bit","png",qua_fil_alp_alpc[2],false,8),
        2=>("png 16 bit","png",qua_fil_alp_alpc[2],false,16),
        3=>("webp lossless","webp",qua_fil_alp_alpc[2],true,8),
        4=>("webp lossy","webp",qua_fil_alp_alpc[2],true,8),
        5=>("tga","tga",qua_fil_alp_alpc[2],true,8),
        _=>("nieznane","nieznane",&0,true,8)
    };




    println!("[{func_id}]\n{formatted_time}   zaczynam konwersje {}\n",rozszerzenie_string);

    // Załaduj obraz
    let img = match image::open(sciezka) {
        Ok(img) => img,
        Err(_) => {
            eprintln!("[{func_id}]\n{formatted_time}   Nie udało się załadować obrazu: {:?}", sciezka);
            return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
        }
    };
    
    let foto_znormalizowane = normalizer::foto_bity_normalizer(img, bity,definitywnie_alpha, *qua_fil_alp_alpc[3],rgb_only);

    // Sprawdzamy długość dłuższej krawędzi
    let (width, height) = foto_znormalizowane.dimensions();
    // let longer_side = if width > height { width } else { height };




    // Przetwarzamy plik w zależności od wybranych rozdzielczości
    for (i, &selected) in rozdzielczosc.iter().enumerate() {
        if selected {
            let max_side = RESOLUTION_MAPPING[i];
            // created_files += 1;
            // Oblicz proporcję skali
            let (scaled_width, scaled_height) = if width > height {
                // Jeśli obraz jest szerszy (poziomy)
                let scaled_width = max_side;
                let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
                (scaled_width, scaled_height)
            } else {
                // Jeśli obraz jest wyższy (pionowy)
                let scaled_height = max_side;
                let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
                (scaled_width, scaled_height)
            };

            // Przeskaluj obraz
            let resized_img = match qua_fil_alp_alpc[1] {
                4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
                0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
                1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
                2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
                3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
                _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
            };
            // let resized_img = foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3);

            // Ścieżka zapisu
            let mut output_path = std::path::Path::new(output).to_path_buf();
            let file_name = sciezka.file_name().unwrap().to_str().unwrap();
            let output_file_name = format!("{}_{}_{}_{}.{}", file_name, i + 1,bity,max_side,rozszerzenie_do_pliku); // np. image_1.png
            output_path.push(lokalny_path);
            output_path.push("converted");
            output_path.push(rozszerzenie_do_pliku);
            output_path.push(output_file_name);

            // Tworzenie katalogu wyjściowego, jeśli nie istnieje
            if let Some(parent) = output_path.parent() {
                create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
            }
            // Zapisz plik w formacie PNG
            println!("[{func_id}]\n{formatted_time}   zakończony resize, wchodzę w encoder\n");
            crate::utils::image_encoder::foto_enkodery(resized_img,output_path,roz_png_fil[0],qua_fil_alp_alpc[0],&roz_png_fil[1]);



            created_files += 1;
            println!("[{func_id}]\n{formatted_time}   test iteracji w {rozszerzenie_string} = {created_files}");
        }
    }

    println!("[{func_id}]\n{formatted_time}   plik {rozszerzenie_string} zapisany, przechodzę dalej\n");
    created_files
}










// pub fn konwersja_do_png(rozdzielczosc: &Vec<bool>, jakosc:&u8,filter: &u8, bity:u8, czy_alpha:&u8, czy_alpha_kolor:&u8, sciezka: &PathBuf,lokalny_path:&Path, output: &str,counter:i32, png_filter:u8) -> i32 {
//     let mut created_files = counter;
//     println!("zaczynam konwersje png");

//     // Załaduj obraz
//     let img = match image::open(sciezka) {
//         Ok(img) => img,
//         Err(_) => {
//             eprintln!("Nie udało się załadować obrazu: {:?}", sciezka);
//             return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
//         }
//     };
    
//     let foto_znormalizowane = normalizer::foto_bity_normalizer(img, bity,czy_alpha, *czy_alpha_kolor,false);

//     // Sprawdzamy długość dłuższej krawędzi
//     let (width, height) = foto_znormalizowane.dimensions();
//     let longer_side = if width > height { width } else { height };



//     // Przetwarzamy plik w zależności od wybranych rozdzielczości
//     for (i, &selected) in rozdzielczosc.iter().enumerate() {
//         if selected {
//             let max_side = RESOLUTION_MAPPING[i];
//             // created_files += 1;
//             // Oblicz proporcję skali
//             let (scaled_width, scaled_height) = if width > height {
//                 // Jeśli obraz jest szerszy (poziomy)
//                 let scaled_width = max_side;
//                 let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
//                 (scaled_width, scaled_height)
//             } else {
//                 // Jeśli obraz jest wyższy (pionowy)
//                 let scaled_height = max_side;
//                 let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
//                 (scaled_width, scaled_height)
//             };

//             // Przeskaluj obraz
//             let resized_img = match filter {
//                 4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
//                 0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//                 1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
//                 2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
//                 3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
//                 _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//             };
//             // let resized_img = foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3);

//             // Ścieżka zapisu
//             let mut output_path = std::path::Path::new(output).to_path_buf();
//             let file_name = sciezka.file_name().unwrap().to_str().unwrap();
//             let output_file_name = format!("{}_{}_{}_{}.png", file_name, i + 1,bity,max_side); // np. image_1.png
//             output_path.push(lokalny_path);
//             output_path.push("converted");
//             output_path.push(format!("png_{bity}"));
//             output_path.push(output_file_name);

//             // Tworzenie katalogu wyjściowego, jeśli nie istnieje
//             if let Some(parent) = output_path.parent() {
//                 create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
//             }

//             // Zapisz plik w formacie PNG
//             // if let Err(e) = resized_img.save_with_format(output_path, ImageFormat::Png) {
//             //     eprintln!("Błąd zapisu obrazu PNG: {}", e);
//             //     continue;
//             // }
//             let afdgdsgfsdasddgd = match jakosc{
//                 0 => CompressionType::Fast,
//                 2 => CompressionType::Best,
//                 _ => CompressionType::Default
//             };
//             let daslkgggggsfkas = match png_filter {

//                 1=>FilterType::Sub,
//                 2=>FilterType::Up,
//                 3=>FilterType::Avg,
//                 4=>FilterType::Paeth,
//                 5=>FilterType::Adaptive,
//                 _=>FilterType::NoFilter,
//             };

//             let mut output_file = std::fs::File::create(&output_path).unwrap();
//             let encoder = PngEncoder::new_with_quality(&mut output_file, afdgdsgfsdasddgd, daslkgggggsfkas);
//             let color_type = match resized_img {
//                 DynamicImage::ImageRgb8(_) => image::ExtendedColorType::Rgb8,
//                 DynamicImage::ImageRgba8(_) => image::ExtendedColorType::Rgba8,
//                 DynamicImage::ImageRgb16(_) => image::ExtendedColorType::Rgb16,
//                 DynamicImage::ImageRgba16(_) => image::ExtendedColorType::Rgba16,
//                 DynamicImage::ImageLuma8(_) => image::ExtendedColorType::L8,
//                 DynamicImage::ImageLumaA8(_) => image::ExtendedColorType::La8,
//                 DynamicImage::ImageLuma16(_) => image::ExtendedColorType::L16,
//                 DynamicImage::ImageLumaA16(_) => image::ExtendedColorType::La16,
//                 _ => image::ExtendedColorType::Rgb8
//             };
//             let img_bytes = resized_img.clone().into_bytes();

//             // match 
//             if let Err(e) = encoder.write_image(&img_bytes, resized_img.width(), resized_img.height(),color_type) {
//                 eprintln!("Błąd zapisu obrazu PNG: {}", e);
//                 continue;
//             }

//             // Zwiększamy licznik utworzonych plików
//             println!("test iteracji w png 8/16bit");
//             created_files += 1;
//         }
//     }

//     println!("kończę konwersje png");
//     created_files
// }

// pub fn konwersja_do_jpg(rozdzielczosc: &Vec<bool>, jakosc:&u8,filter: &u8, bity:u8,czy_alpha:&u8, czy_alpha_kolor:&u8, sciezka: &PathBuf,lokalny_path:&Path, output: &str,counter:i32)->i32{
//     let mut created_files = counter;
//     println!("zaczynam konwersje jpg");
//     // Załaduj obraz
//     let img = match image::open(sciezka) {
//         Ok(img) => img,
//         Err(_) => {
//             eprintln!("Nie udało się załadować obrazu: {:?}", sciezka);
//             return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
//         }
//     };
    
//     let foto_znormalizowane = normalizer::foto_bity_normalizer(img, 8,&0, *czy_alpha_kolor,false);

//     // Sprawdzamy długość dłuższej krawędzi
//     let (width, height) = foto_znormalizowane.dimensions();
//     let longer_side = if width > height { width } else { height };



//     // Przetwarzamy plik w zależności od wybranych rozdzielczości
//     for (i, &selected) in rozdzielczosc.iter().enumerate() {
//         if selected {
//             let max_side = RESOLUTION_MAPPING[i];
//             // created_files += 1;
//             // Oblicz proporcję skali
//             let (scaled_width, scaled_height) = if width > height {
//                 // Jeśli obraz jest szerszy (poziomy)
//                 let scaled_width = max_side;
//                 let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
//                 (scaled_width, scaled_height)
//             } else {
//                 // Jeśli obraz jest wyższy (pionowy)
//                 let scaled_height = max_side;
//                 let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
//                 (scaled_width, scaled_height)
//             };

//             let resized_img = match filter {
//                 4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
//                 0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//                 1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
//                 2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
//                 3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
//                 _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//             };


//             // Przeskaluj obraz

//             // Ścieżka zapisu
//             let mut output_path = std::path::Path::new(output).to_path_buf();
//             let file_name = sciezka.file_name().unwrap().to_str().unwrap();
//             let output_file_name = format!("{}_{}_{}.jpg", file_name, i + 1,max_side); // np. image_1.png
//             output_path.push(lokalny_path);
//             output_path.push("converted");
//             output_path.push("jpg");
//             output_path.push(output_file_name);

//             // Tworzenie katalogu wyjściowego, jeśli nie istnieje
//             if let Some(parent) = output_path.parent() {
//                 create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
//             }
//             // let ghjdfgvbngkg = JpegEncoder::new_with_quality(output_path, 100);
//             // Zapisz plik w formacie PNG
//             // if let Err(e) = resized_img.save_with_format(output_path, ImageFormat::Jpeg) {
//             //     eprintln!("Błąd zapisu obrazu JPG: {}", e);
//             //     continue;
//             // }

//             // Zapisz obraz jako JPG z ustawioną jakością
//             let mut output_file = std::fs::File::create(&output_path).unwrap();
//             let mut encoder = JpegEncoder::new_with_quality(&mut output_file, *jakosc);
            
//             if let Err(e) = encoder.encode(&resized_img.to_rgb8(), resized_img.width(), resized_img.height(),image::ExtendedColorType::Rgb8) {
//                 eprintln!("Błąd zapisu obrazu JPG: {}", e);
//                 continue;
//             }

//             // Zwiększamy licznik utworzonych plików
//             println!("test iteracji w jpg");
//             created_files += 1;
//         }
//     }
//     println!("kończę konwersje jpg");
//     created_files
// }

// pub fn konwersja_do_webp(rozdzielczosc: &Vec<bool>, jakosc:&u8,filter: &u8, bity:u8, czy_alpha:&u8, czy_alpha_kolor:&u8, sciezka: &PathBuf,lokalny_path:&Path, output: &str,counter:i32) -> i32 {
//     let mut created_files = counter;
//     println!("zaczynam konwersje webp");

//     // Załaduj obraz
//     let img = match image::open(sciezka) {
//         Ok(img) => img,
//         Err(_) => {
//             eprintln!("Nie udało się załadować obrazu: {:?}", sciezka);
//             return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
//         }
//     };
    
//     let foto_znormalizowane = normalizer::foto_bity_normalizer(img, bity,&czy_alpha, *czy_alpha_kolor,false);

//     // Sprawdzamy długość dłuższej krawędzi
//     let (width, height) = foto_znormalizowane.dimensions();
//     let longer_side = if width > height { width } else { height };



//     // Przetwarzamy plik w zależności od wybranych rozdzielczości
//     for (i, &selected) in rozdzielczosc.iter().enumerate() {
//         if selected {
//             let max_side = RESOLUTION_MAPPING[i];
//             // created_files += 1;
//             // Oblicz proporcję skali
//             let (scaled_width, scaled_height) = if width > height {
//                 // Jeśli obraz jest szerszy (poziomy)
//                 let scaled_width = max_side;
//                 let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
//                 (scaled_width, scaled_height)
//             } else {
//                 // Jeśli obraz jest wyższy (pionowy)
//                 let scaled_height = max_side;
//                 let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
//                 (scaled_width, scaled_height)
//             };

//             // Przeskaluj obraz
//             let resized_img = match filter {
//                 4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
//                 0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//                 1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
//                 2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
//                 3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
//                 _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//             };
//             // let resized_img = foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3);

//             // Ścieżka zapisu
//             let mut output_path = std::path::Path::new(output).to_path_buf();
//             let file_name = std::path::Path::new(sciezka).file_name().unwrap().to_str().unwrap();
//             let output_file_name = format!("{}_{}_{}_{}.webp", file_name, i + 1,bity,max_side); // np. image_1.png
//             output_path.push(lokalny_path);
//             output_path.push("converted");
//             output_path.push("webp_lossless");
//             output_path.push(output_file_name);

//             // Tworzenie katalogu wyjściowego, jeśli nie istnieje
//             if let Some(parent) = output_path.parent() {
//                 create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
//             }
//             // let mut webp_options = image::codecs::webp::WebPEncoder;
//             // let save_options = image::WebPEncoder::new().with_quality(*jakosc as f32 / 100.0);

//             // Zapisz plik w formacie PNG
//             if let Err(e) = resized_img.save_with_format(output_path, ImageFormat::WebP) {
//                 eprintln!("Błąd zapisu obrazu WEBP: {}", e);
//                 continue;
//             }

//             // Zwiększamy licznik utworzonych plików
//             println!("test iteracji w webp lossless");
//             created_files += 1;
//         }
//     }

//     println!("kończę konwersje webp");
//     created_files
// }

// pub fn konwersja_do_webp_lossy(rozdzielczosc: &Vec<bool>, jakosc: &u8, filter: &u8, bity: u8, czy_alpha: &u8, czy_alpha_kolor: &u8, sciezka: &PathBuf,lokalny_path:&Path, output: &str, counter: i32) -> i32 {
//     let mut created_files = counter;
//     println!("zaczynam konwersje webp");

//     // Załaduj obraz
//     let img = match image::open(sciezka) {
//         Ok(img) => img,
//         Err(_) => {
//             eprintln!("Nie udało się załadować obrazu: {:?}", sciezka);
//             return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
//         }
//     };

//     // Normalizowanie obrazu
//     let foto_znormalizowane = normalizer::foto_bity_normalizer(img, bity, &czy_alpha, *czy_alpha_kolor,true);

//     // Sprawdzamy długość dłuższej krawędzi
//     let (width, height) = foto_znormalizowane.dimensions();
//     let longer_side = if width > height { width } else { height };


//     // Przetwarzamy plik w zależności od wybranych rozdzielczości
//     for (i, &selected) in rozdzielczosc.iter().enumerate() {
//         if selected {
//             let max_side = RESOLUTION_MAPPING[i];
//             // created_files += 1;
//             // Oblicz proporcję skali
//             let (scaled_width, scaled_height) = if width > height {
//                 // Jeśli obraz jest szerszy (poziomy)
//                 let scaled_width = max_side;
//                 let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
//                 (scaled_width, scaled_height)
//             } else {
//                 // Jeśli obraz jest wyższy (pionowy)
//                 let scaled_height = max_side;
//                 let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
//                 (scaled_width, scaled_height)
//             };

//             // Przeskaluj obraz
//             let resized_img = match filter {
//                 4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
//                 0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//                 1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
//                 2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
//                 3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
//                 _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//             };

//             // Ścieżka zapisu
//             let mut output_path = std::path::Path::new(output).to_path_buf();
//             let file_name = sciezka.file_name().unwrap().to_str().unwrap();
//             let output_file_name = format!("{}_lossy_{}_{}_{}.webp", file_name, i + 1, bity, max_side); // np. image_1.png
//             output_path.push(lokalny_path);
//             output_path.push("converted");
//             output_path.push("webp_lossy");
//             output_path.push(output_file_name);

//             // Tworzenie katalogu wyjściowego, jeśli nie istnieje
//             if let Some(parent) = output_path.parent() {
//                 create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
//             }

//             // Tworzymy encoder WebP
//             let encoder = Encoder::from_image(&resized_img).unwrap();

//             // Zakoduj obraz z określoną jakością (od 0 do 100)
//             let webp_data = encoder.encode(*jakosc as f32);

//             // Zapisz zakodowany plik WebP
//             if let Err(e) = std::fs::write(&output_path, &*webp_data) {
//                 eprintln!("Błąd zapisu obrazu WEBP: {}", e);
//                 continue;
//             }
//             println!("test iteracji w webp lossy");
//             created_files += 1;

//             // Zwiększamy licznik utworzonych plików
//         }
//     }

//     println!("kończę konwersje webp lossy");
//     created_files
// }

// pub fn konwersja_do_tga(rozdzielczosc: &Vec<bool>, jakosc:&u8,filter: &u8, bity:u8, czy_alpha:&u8, czy_alpha_kolor:&u8, sciezka: &PathBuf,lokalny_path:&Path, output: &str,counter:i32) -> i32 {
//     let mut created_files = counter;
//     println!("zaczynam konwersje tga");

//     // Załaduj obraz
//     let img = match image::open(sciezka) {
//         Ok(img) => img,
//         Err(_) => {
//             eprintln!("Nie udało się załadować obrazu: {:?}", sciezka);
//             return 0; // Zwracamy 0, jeśli obraz nie mógł zostać załadowany
//         }
//     };
    
//     let foto_znormalizowane = normalizer::foto_bity_normalizer(img, bity,&czy_alpha, *czy_alpha_kolor,true);

//     // Sprawdzamy długość dłuższej krawędzi
//     let (width, height) = foto_znormalizowane.dimensions();
//     let longer_side = if width > height { width } else { height };




//     // Przetwarzamy plik w zależności od wybranych rozdzielczości
//     for (i, &selected) in rozdzielczosc.iter().enumerate() {
//         if selected {
//             let max_side = RESOLUTION_MAPPING[i];
//             // created_files += 1;
//             // Oblicz proporcję skali
//             let (scaled_width, scaled_height) = if width > height {
//                 // Jeśli obraz jest szerszy (poziomy)
//                 let scaled_width = max_side;
//                 let scaled_height = (max_side as f32 / width as f32 * height as f32) as u32;
//                 (scaled_width, scaled_height)
//             } else {
//                 // Jeśli obraz jest wyższy (pionowy)
//                 let scaled_height = max_side;
//                 let scaled_width = (max_side as f32 / height as f32 * width as f32) as u32;
//                 (scaled_width, scaled_height)
//             };

//             // Przeskaluj obraz
//             let resized_img = match filter {
//                 4 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
//                 0 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//                 1 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
//                 2 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
//                 3 => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
//                 _ => foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
//             };
//             // let resized_img = foto_znormalizowane.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3);

//             // Ścieżka zapisu
//             let mut output_path = std::path::Path::new(output).to_path_buf();
//             let file_name = sciezka.file_name().unwrap().to_str().unwrap();
//             let output_file_name = format!("{}_{}_{}_{}.tga", file_name, i + 1,bity,max_side); // np. image_1.png
//             output_path.push(lokalny_path);
//             output_path.push("converted");
//             output_path.push("tga");
//             output_path.push(output_file_name);

//             // Tworzenie katalogu wyjściowego, jeśli nie istnieje
//             if let Some(parent) = output_path.parent() {
//                 create_dir_all(parent).unwrap(); // Zignorujemy ewentualne błędy przy tworzeniu katalogu
//             }

//             // Zapisz plik w formacie PNG
//             if let Err(e) = resized_img.save_with_format(output_path, ImageFormat::Tga) {
//                 eprintln!("Błąd zapisu obrazu Tga: {}", e);
//                 continue;
//             }

//             // Zwiększamy licznik utworzonych plików
//             println!("test iteracji w tga");
//             created_files += 1;
//         }
//     }

//     println!("kończę konwersje tga");
//     created_files
// }