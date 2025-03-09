use image::DynamicImage;
use image::GenericImageView;
use crate::utils::comunicat::komunikat;


pub fn rgb_to_bw(foto:DynamicImage,rozdzielczosc:u8, ilosc_bitow:u8, filter:u8)->Result<DynamicImage, std::io::Error>{
    #[cfg(feature = "statystyki")]
    let nazwa_funkcji="rgb_to_bw";
    let color_type =foto.color();
    // let mut result_img: DynamicImage = foto.clone();  // Default to the original image


    let result_img = match (color_type,ilosc_bitow){
        (image::ColorType::Rgb8,8) =>{
            let img = foto.to_rgb8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"Rgba16, ALPHA MA BYĆ, input 8 bit, tylko rgb true|false");
            let processed_dynamic = DynamicImage::ImageRgb8(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::Rgb8,16) =>{
            let img = foto.to_rgb8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"Rgba16, ALPHA MA BYĆ, input 8 bit, tylko rgb true|false");
            let processed_dynamic = DynamicImage::ImageRgb8(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        (image::ColorType::Rgb16,8) =>{
            let img = foto.to_rgb16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"Rgba16, ALPHA MA BYĆ, input 8 bit, tylko rgb true|false");
            let processed_dynamic = DynamicImage::ImageRgb16(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::Rgb16,16) =>{
            let img = foto.to_rgb16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"Rgba16, ALPHA MA BYĆ, input 8 bit, tylko rgb true|false");
            let processed_dynamic = DynamicImage::ImageRgb16(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        //alpha
        (image::ColorType::Rgba8,8) =>{
            let img = foto.to_rgba8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageRgba8(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::Rgba8,16) =>{
            let img = foto.to_rgba8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageRgba8(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        (image::ColorType::Rgba16,8) =>{
            let img = foto.to_rgba16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageRgba16(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::Rgba16,16) =>{
            let img = foto.to_rgba16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageRgba16(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        //luma
        (image::ColorType::L8,16) =>{
            let img = foto.to_luma8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLuma8(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        (image::ColorType::L16,8) =>{
            let img = foto.to_luma16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLuma16(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        //alpha
        (image::ColorType::La8,8) =>{
            let img = foto.to_luma_alpha8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLumaA8(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::La8,16) =>{
            let img = foto.to_luma_alpha8();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLumaA8(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        (image::ColorType::La16,8) =>{
            let img = foto.to_luma_alpha16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLumaA16(img);
            let fdhsfg_img = processed_dynamic.to_luma8();
            DynamicImage::ImageLuma8(fdhsfg_img)
        },
        (image::ColorType::La16,16) =>{
            let img = foto.to_luma_alpha16();
            #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"rgbaa16, ALPHA MA BYĆ, input 8 bit, tylko rgba true|false");
            let processed_dynamic = DynamicImage::ImageLumaA16(img);
            let fdhsfg_img = processed_dynamic.to_luma16();
            DynamicImage::ImageLuma16(fdhsfg_img)
        },
        _ => foto.clone()
    };

    let poprawna_rozdzielczosc = match rozdzielczosc{
        0 => 16384,
        1 => 8192,
        2 => 4096, // 4K
        3 => 2048, // 2K
        4 => 1024, // 1K
        5 => 512,
        6 => 256,
        7 => 128,
        8 => 64,
        9 => 32,
        _ => 666
    };
    let (width, height) = result_img.dimensions();

        
            let max_side = poprawna_rozdzielczosc;

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
            let resized_img = match filter {
                4 => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Lanczos3),
                0 => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
                1 => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Triangle),
                2 => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::CatmullRom),
                3 => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Gaussian),
                _ => result_img.resize_exact(scaled_width, scaled_height, image::imageops::FilterType::Nearest),
            };





    #[cfg(feature = "statystyki")]
            komunikat(nazwa_funkcji,"wychodze");

    Ok(resized_img)
}