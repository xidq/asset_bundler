/// Funkcja szyfrująca/deszyfrująca metodą XOR
/// currently not used
/// 
pub fn xor_encrypt_decrypt(data: &mut [u8], password: &str) {
    let password_bytes: &[u8] = password.as_bytes();
    for (i, byte) in data.iter_mut().enumerate() {
        *byte ^= password_bytes[i % password_bytes.len()];
    }
}