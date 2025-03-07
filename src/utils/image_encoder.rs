use std::path::PathBuf;

// use egui::epaint::tessellator::Path;
use image::DynamicImage;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::png::CompressionType;
use image::codecs::png::FilterType;
use image::ImageEncoder;
// use std::fs::create_dir_all;
// use chrono::Local;
use image::ImageFormat;
use webp::Encoder;
use crate::utils::comunicat::komunikat;







pub fn foto_enkodery(fotu:DynamicImage,sciezka:PathBuf,rozszerzenie:u8,quality:&u8,png_filter:&u8){
    // let nazwa_funkcji = "foto_enkodery";




    match rozszerzenie{
        1|2=>{png_enkoder(fotu,sciezka,quality,png_filter)},
        3=>{webp_enkoder(fotu, sciezka);},
        4=>{webp_lossy_enkoder(fotu, sciezka, quality);},
        5=>{tga_enkoder(fotu, sciezka);},
        _=>{jpg_enkoder(fotu,sciezka, quality);}
    }


}

fn jpg_enkoder(fotu:DynamicImage,sciezka:PathBuf,quality:&u8){
    let nazwa_funkcji = "jpg_enkoder";
    komunikat(nazwa_funkcji,"encoder JPG rozpoczęty");
    let mut output_file = std::fs::File::create(&sciezka).unwrap();
    let mut encoder = JpegEncoder::new_with_quality(&mut output_file, *quality);
    
    if let Err(e) = encoder.encode(&fotu.to_rgb8(), fotu.width(), fotu.height(),image::ExtendedColorType::Rgb8) {
        let err_reader = format!("Błąd zapisu obrazu JPG: {}", e);
        komunikat(nazwa_funkcji,&err_reader);
       
    }
    komunikat(nazwa_funkcji,"encoder JPG zakończony\n");
}

fn png_enkoder(fotu:DynamicImage,sciezka:PathBuf,png_compression:&u8,png_filter:&u8){
    let nazwa_funkcji = "png_enkoder";
        komunikat(nazwa_funkcji,"encoder PNG rozpoczęty");
    let afdgdsgfsdasddgd = match png_compression{
        0 => CompressionType::Fast,
        2 => CompressionType::Best,
        _ => CompressionType::Default
    };
    let daslkgggggsfkas = match png_filter {

        1=>FilterType::Sub,
        2=>FilterType::Up,
        3=>FilterType::Avg,
        4=>FilterType::Paeth,
        5=>FilterType::Adaptive,
        _=>FilterType::NoFilter,
    };

    let mut output_file = std::fs::File::create(&sciezka).unwrap();
    let encoder = PngEncoder::new_with_quality(&mut output_file, afdgdsgfsdasddgd, daslkgggggsfkas);
    let color_type = match fotu {
        DynamicImage::ImageRgb8(_) => image::ExtendedColorType::Rgb8,
        DynamicImage::ImageRgba8(_) => image::ExtendedColorType::Rgba8,
        DynamicImage::ImageRgb16(_) => image::ExtendedColorType::Rgb16,
        DynamicImage::ImageRgba16(_) => image::ExtendedColorType::Rgba16,
        DynamicImage::ImageLuma8(_) => image::ExtendedColorType::L8,
        DynamicImage::ImageLumaA8(_) => image::ExtendedColorType::La8,
        DynamicImage::ImageLuma16(_) => image::ExtendedColorType::L16,
        DynamicImage::ImageLumaA16(_) => image::ExtendedColorType::La16,
        _ => image::ExtendedColorType::Rgb8
    };
    let img_bytes = fotu.clone().into_bytes();

    // match 
    if let Err(e) = encoder.write_image(&img_bytes, fotu.width(), fotu.height(),color_type) {
        let err_reader = format!("Błąd zapisu obrazu PNG: {}", e);
        komunikat(nazwa_funkcji,&err_reader);
    }
    komunikat(nazwa_funkcji,"encoder PNG zakończony\n");
}

fn webp_enkoder(fotu:DynamicImage,sciezka:PathBuf){
    let nazwa_funkcji = "webp_enkoder";
    komunikat(nazwa_funkcji,"encoder WEBP rozpoczęty");

    // let mut webp_options = image::codecs::webp::WebPEncoder;
    // let save_options = image::WebPEncoder::new().with_quality(*jakosc as f32 / 100.0);

    // Zapisz plik w formacie PNG
    if let Err(e) = fotu.save_with_format(sciezka, ImageFormat::WebP) {

        let err_reader = format!("Błąd zapisu obrazu WEBP: {}", e);
        komunikat(nazwa_funkcji,&err_reader);
    }
    komunikat(nazwa_funkcji,"encoder WEBP zakończony")
}
fn webp_lossy_enkoder(fotu:DynamicImage,sciezka:PathBuf,quality:&u8){
    let nazwa_funkcji = "webp_lossy_enkoder";
    komunikat(nazwa_funkcji,"encoder WEBP lossy rozpoczęty");
    // Tworzymy encoder WebP
    let encoder = Encoder::from_image(&fotu).unwrap();

    // Zakoduj obraz z określoną jakością (od 0 do 100)
    let webp_data = encoder.encode(*quality as f32);

    // Zapisz zakodowany plik WebP
    if let Err(e) = std::fs::write(&sciezka, &*webp_data) {
        let err_reader = format!("Błąd zapisu obrazu WEBP lossy: {}", e);
        komunikat(nazwa_funkcji,&err_reader);

    }
    komunikat(nazwa_funkcji,"encoder WEBP lossy zakończony\n");
}

fn tga_enkoder(fotu:DynamicImage,sciezka:PathBuf){
    let nazwa_funkcji = "tga_enkoder";
    komunikat(nazwa_funkcji,"encoder TGA rozpoczęty");
    // Zapisz plik w formacie PNG
    if let Err(e) = fotu.save_with_format(sciezka, ImageFormat::Tga) {
        let err_reader = format!("Błąd zapisu obrazu Tga: {}", e);
        komunikat(nazwa_funkcji,&err_reader);

    }
    komunikat(nazwa_funkcji,"encoder TGA zakończony\n");

}