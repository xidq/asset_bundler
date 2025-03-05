use std::{path::PathBuf, sync::{Arc,Mutex}};
pub fn get_locked_data_bool(arc_z_typ_clone: &Arc<Mutex<Vec<bool>>>) -> Result<Vec<bool>, std::io::Error> {
    match arc_z_typ_clone.lock() {
        Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' bool")),
    }
}
pub fn get_locked_data_u8(arc_z_typ_clone: &Arc<Mutex<Vec<u8>>>) -> Result<Vec<u8>, std::io::Error> {
    match arc_z_typ_clone.lock() {
        Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' u8")),
    }
}
pub fn get_locked_data_string(arc_z_typ_clone: &Arc<Mutex<Vec<String>>>) -> Result<Vec<String>, std::io::Error> {
    match arc_z_typ_clone.lock() {
        Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' String")),
    }
}
// pub fn get_locked_data_i32(arc_z_typ_clone: &Arc<Mutex<Vec<i32>>>) -> Result<Vec<i32>, std::io::Error> {
//     match arc_z_typ_clone.lock() {
//         Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
//         Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' i32")),
//     }
// }
// pub fn get_locked_data_f32(arc_z_typ_clone: &Arc<Mutex<Vec<f32>>>) -> Result<Vec<f32>, std::io::Error> {
//     match arc_z_typ_clone.lock() {
//         Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
//         Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' f32")),
//     }
// }
pub fn get_locked_data_pathbuf(arc_z_typ_clone: &Arc<Mutex<Vec<PathBuf>>>) -> Result<Vec<PathBuf>, std::io::Error> {
    match arc_z_typ_clone.lock() {
        Ok(locked_data) => Ok(locked_data.clone()),  // Zwracamy dane po odblokowaniu
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Błąd przy operacji 'lock' PathBuf")),
    }
}