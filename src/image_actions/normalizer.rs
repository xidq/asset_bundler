// use cipher::consts::False;
use chrono::Local;
use image::DynamicImage;

pub fn foto_bity_normalizer(orig_foto: DynamicImage, glebia_bitow: u8, czy_alpha: &u8, alpha_kolor: u8, tylko_rgb:bool) -> DynamicImage {
    //universal fn setting!!!!!!!!!!!!!!!!!!!!!!!!!!1
    let func_id = "foto_bity_normalizer w image_actions::normalizer.rs" ;
    let current_time = Local::now();
    let formatted_time = current_time.format("%H:%M:%S").to_string();
    //    println!("[{func_id}]\n{formatted_time}   zaczynam konwersje {}",rozszerzenie_string);
    //***************************************************************

    println!("[{func_id}]\n{formatted_time}   rozpoczęcie");
    // Get the color type of the image
    let color_type = orig_foto.color();
    // let mut result_img: DynamicImage = orig_foto.clone();  // Default to the original image
 
    // Handle case when alpha is disabled (czy_alpha == 0)
    // if *czy_alpha == 0 {
        // println!("[normalize]oficjalnie idę przez alpha 0");
        let color = match alpha_kolor{
            1 => (0, 0, 0), //czorny
            2 => (255,0,0), //czrwony
            3 => (0,255,0), //zielony
            4 => (0,0,255), //niebieski
            _ => (255, 255, 255), //bioły
        };
        let color_16: (u16, u16, u16) = match alpha_kolor{
            1 => (0, 0, 0), //czorny
            2 => (65535,0,0), //czrwony
            3 => (0,65535,0), //zielony
            4 => (0,0,65535), //niebieski
            _ => (65535, 65535, 65535), //bioły
        };

        // let kolor_luma8 = ((color.0 as f32 * 0.299) + (color.1 as f32 * 0.587) + (color.2 as f32 * 0.114)) as u8;
        // let kolor_luma16 = ((u16::from(color.0) * 299 + u16::from(color.1) * 587 + u16::from(color.2) * 114) / 1000) * 257; // Scale to 16-bit
        let result_img = match (color_type,glebia_bitow,tylko_rgb,czy_alpha ){

            // 8bit alpha

            (image::ColorType::Rgba8,_,true|false,0)=> {
                let mut img = orig_foto.to_rgba8();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    // let g = pixel.0[1];
                    // let b = pixel.0[2];
                    let a = pixel.0[3];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::Rgba([color.0, color.1, color.2, 255]);  // Correctly use Rgba
                    }
                }

                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true|false",color_type);
                let processed_dynamic = DynamicImage::ImageRgba8(img);
                let rgb8_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(rgb8_img)
            },

            //16bit alpha

            (image::ColorType::Rgba16,8,true|false,0) =>{
                let mut img = orig_foto.to_rgba16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    // let g = pixel.0[1];
                    // let b = pixel.0[2];
                    let a = pixel.0[3];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::Rgba([color_16.0, color_16.1, color_16.2, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true|false",color_type);
                let processed_dynamic = DynamicImage::ImageRgba16(img);
                let rgb16_to_rgb8_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(rgb16_to_rgb8_img)
            },
            (image::ColorType::Rgba16,16,true|false,0) =>{
                let mut img = orig_foto.to_rgba16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    // let g = pixel.0[1];
                    // let b = pixel.0[2];
                    let a = pixel.0[3];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::Rgba([color_16.0, color_16.1, color_16.2, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb16, tylko rgb? true|false",color_type);
                let processed_dynamic = DynamicImage::ImageRgba16(img);
                let rgb16_img = processed_dynamic.to_rgb16();
                DynamicImage::ImageRgb16(rgb16_img)
            },

            //rgb16 no alpha

            (image::ColorType::Rgb16,8,true|false,0|1) =>{
                let mut img = orig_foto.to_rgba16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    // let g = pixel.0[1];
                    // let b = pixel.0[2];
                    let a = pixel.0[3];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::Rgba([color_16.0, color_16.1, color_16.2, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true|false",color_type);
                let processed_dynamic = DynamicImage::ImageRgba16(img);
                let rgb16_to_rgb8_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(rgb16_to_rgb8_img)
            },

            //luma8 alpha

            (image::ColorType::La8,_,false,0) =>{
                let mut img = orig_foto.to_luma_alpha8();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color.0, 255]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: L8, tylko rgb? false",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA8(img);
                let luma8_img = processed_dynamic.to_luma8();
                DynamicImage::ImageLuma8(luma8_img)
            },
            (image::ColorType::La8,_,true,0) =>{
                let mut img = orig_foto.to_luma_alpha8();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color.0, 255]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA8(img);
                let luma8_to_rgb_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(luma8_to_rgb_img)
            },

            //luma8 no alpha

            (image::ColorType::L8,_,true,0|1) =>{
                let mut img = orig_foto.to_luma_alpha8();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color.0, 255]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA8(img);
                let luma8_to_rgb_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(luma8_to_rgb_img)
            },

            //luma16 alpha

            (image::ColorType::La16,8,true,0) =>{
                let mut img = orig_foto.to_luma_alpha16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color_16.0, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_rgb8_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(luma16_to_rgb8_img)
            },
            (image::ColorType::La16,16,true,0) =>{
                let mut img = orig_foto.to_luma_alpha16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color_16.0, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb16, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_rgb16_img = processed_dynamic.to_rgb16();
                DynamicImage::ImageRgb16(luma16_to_rgb16_img)
            },
            (image::ColorType::La16,8,false,0) =>{
                let mut img = orig_foto.to_luma_alpha16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color_16.0, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: L8, tylko rgb? false",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_luma8_img = processed_dynamic.to_luma8();
                DynamicImage::ImageLuma8(luma16_to_luma8_img)
            },
            (image::ColorType::La16,16,false,0) =>{
                let mut img = orig_foto.to_luma_alpha16();
                for pixel in img.pixels_mut() {
                    // Access the channels using .0, .1, .2, .3 for Rgba<u8>
                    // let r = pixel.0[0];
                    let a = pixel.0[1];
                    if a == 0 {  // If fully transparent, set it to the chosen color
                        *pixel = image::LumaA([color_16.0, 65535]);  // Correctly use Rgba
                    }
                }
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: L16, tylko rgb? false",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_luma16_img = processed_dynamic.to_luma16();
                DynamicImage::ImageLuma16(luma16_to_luma16_img)
            },

            //luma bez alpha

            (image::ColorType::L16,8,true,0|1) =>{
                let img = orig_foto.to_luma16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLuma16(img);
                let luma16_to_rgb8_img = processed_dynamic.to_rgb8();
                DynamicImage::ImageRgb8(luma16_to_rgb8_img)
            },
            (image::ColorType::L16,8,false,0|1) =>{
                let img = orig_foto.to_luma16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: L8, tylko rgb? false",color_type);
                let processed_dynamic = DynamicImage::ImageLuma16(img);
                let luma16_to_luma8_img = processed_dynamic.to_luma8();
                DynamicImage::ImageLuma8(luma16_to_luma8_img)
            },
            (image::ColorType::L16,16,true,0|1) =>{                            
                let img = orig_foto.to_luma16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgb16, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLuma16(img);
                let luma16_to_rgb16_img = processed_dynamic.to_rgb16();
                DynamicImage::ImageRgb16(luma16_to_rgb16_img)
            },
//******************************************************************************************************* */
            //TUTAJ BEZ PRZERABIANIA ALPHA!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//******************************************************************************************************* */

            //rgba16
            (image::ColorType::Rgba16,8,true|false,1)=> {
                let img = orig_foto.to_rgba16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgba8, tylko rgb? true|false",color_type);
                let processed_dynamic = DynamicImage::ImageRgba16(img);
                let rgb8_img = processed_dynamic.to_rgba8();
                DynamicImage::ImageRgba8(rgb8_img)
            },

            //luma to rgb
            (image::ColorType::La16,8,true,1) =>{
                let img = orig_foto.to_luma_alpha16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgba8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_luma8_img = processed_dynamic.to_rgba8();
                DynamicImage::ImageRgba8(luma16_to_luma8_img)
            },
            (image::ColorType::La16,8,false,1) =>{
                let img = orig_foto.to_luma_alpha16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: La8, tylko rgb? false",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_luma8_img = processed_dynamic.to_luma_alpha8();
                DynamicImage::ImageLumaA8(luma16_to_luma8_img)
            },
            (image::ColorType::La16,16,true,1) =>{
                let img = orig_foto.to_luma_alpha16();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgba8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA16(img);
                let luma16_to_luma8_img = processed_dynamic.to_rgba16();
                DynamicImage::ImageRgba16(luma16_to_luma8_img)
            },
            (image::ColorType::La8,_,true,1) =>{
                let img = orig_foto.to_luma_alpha8();
                println!("[{func_id}]\n{formatted_time}   wejście: {:?}, wyjście: Rgba8, tylko rgb? true",color_type);
                let processed_dynamic = DynamicImage::ImageLumaA8(img);
                let luma16_to_luma8_img = processed_dynamic.to_rgba8();
                DynamicImage::ImageRgba8(luma16_to_luma8_img)
            },

            _ => {
                println!("[{func_id}]\n{formatted_time}   pominięto normalizacje, dlaczego? colortype{:?}",color_type);
                orig_foto.clone()}, // If the image is not RGBA, keep it as it is
        };


         println!("[{func_id}]\n{formatted_time}   wychodzę z normalizera\n");
    // } else {
        // Handle the bit depth conversion
        // if glebia_bitow == 8  && color_type != image::ColorType::Rgb8  {
        //     let rgb_img = orig_foto.to_rgb8();
        //     result_img = DynamicImage::ImageRgb8(rgb_img);
            
        // } else if glebia_bitow == 16 && color_type != image::ColorType::Rgb16 {

        //     let rgba_img = orig_foto.to_rgba16();
        //     result_img = DynamicImage::ImageRgba16(rgba_img);
        // }




        // result_img = match glebia_bitow {
        //     8 => match result_img {
        //         DynamicImage::ImageRgba8(_) if czy_alpha != 0 => result_img,
        //         DynamicImage::ImageRgba16(img) => DynamicImage::ImageRgba8(img.to_rgba8()),
        //         DynamicImage::ImageRgb16(img) => DynamicImage::ImageRgb8(img.to_rgb8()),
        //         DynamicImage::ImageLumaA16(img) => DynamicImage::ImageLumaA8(img.to_luma_alpha8()),
        //         DynamicImage::ImageLuma16(img) => DynamicImage::ImageLuma8(img.to_luma8()),
        //         _ => result_img,
        //     },
        //     16 => match result_img {
        //         DynamicImage::ImageRgba8(img) => DynamicImage::ImageRgba16(img.to_rgba16()),
        //         DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb16(img.to_rgb16()),
        //         DynamicImage::ImageLumaA8(img) => DynamicImage::ImageLumaA16(img.to_luma_alpha16()),
        //         DynamicImage::ImageLuma8(img) => DynamicImage::ImageLuma16(img.to_luma16()),
        //         _ => result_img,
        //     },
        //     _ => result_img,
        // };
    // }

    // Return the resulting image as a DynamicImage
    result_img
}
