use chrono::Local;
use std::panic::Location;
// use std::backtrace::Backtrace;

// #[macro_export]
// macro_rules! komunikat {
//     ($tekst:expr) => {{
//         let current_time = Local::now();
//         let formatted_time = current_time.format("%H:%M:%S").to_string();
//         let func_id = std::panic::Location::caller().function();
//         println!("[{}]\n{}   {}", func_id, formatted_time, $tekst);
//     }};
// }

#[track_caller]
pub fn komunikat(fn_name:&str,tekst:&str){
    // let func_id = "convert_images w image_actions::image_actions_main.rs" ;
    let current_time = Local::now();
    let formatted_time = current_time.format("%H:%M:%S").to_string();
    // let formatted_time2 = formatted_time.clone();
    let file_path = Location::caller().file();
    
    // Wyciągamy nazwę pliku z pełnej ścieżki
    let func_id_file = file_path.rsplit('/').next().unwrap_or("<nieznany plik>");

    let func_line = Location::caller().line();




    println!("[linia:{} w {} z {}]\n{}   {}",func_line,fn_name,func_id_file,formatted_time,tekst)
}