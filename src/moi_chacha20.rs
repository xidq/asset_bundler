use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    XChaCha20Poly1305,
    XNonce
};
use chacha20poly1305::AeadCore;
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use chacha20poly1305::aead::rand_core::RngCore;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24; // XChaCha20Poly1305 uses 24-byte nonces
const PBKDF2_ITERATIONS: u32 = 100_000;
// const SIGNATURE_LEN: usize = 49; // Długość powyższego stringa w bajtach

pub fn encrypt_chacha(data: &[u8], password: &str) -> Vec<u8> {
    // Generate salt
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    
    // Generate nonce
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    
    // Derive key
    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut key
    ).expect("PBKDF2 should not fail");
    
    // Encrypt data
    let cipher = XChaCha20Poly1305::new_from_slice(&key).expect("Invalid key length");
    let ciphertext = cipher.encrypt(&nonce, data).expect("Encryption failure");
    
    // Combine salt + nonce + ciphertext
    let mut result = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&salt);
    result.extend_from_slice(&nonce);
    result.extend(&ciphertext);
    
    result
}








pub fn decrypt(data: &[u8], password: &str) -> Vec<u8> {
    // Weryfikacja minimalnej długości
    assert!(
        data.len() >= SALT_LEN + NONCE_LEN + 16,
        "Dane wejściowe są zbyt krótkie ({} bajtów), wymagane minimum: {} bajtów", 
        data.len(),
        SALT_LEN + NONCE_LEN + 16
    );

    

    
    // Wydzielenie części szyfrowanej
    let encrypted_part = &data/*[SIGNATURE_LEN..]*/;
    
    // Podział na składowe
    let (salt, rest) = encrypted_part.split_at(SALT_LEN);
    let (nonce, ciphertext) = rest.split_at(NONCE_LEN);

    // Logowanie diagnostyczne
    #[cfg(feature = "statystyki")]
    println!("Długość danych: {}", data.len());
    #[cfg(feature = "statystyki")]
    println!("Salt (hex): {}", hex::encode(salt));
    #[cfg(feature = "statystyki")]
    println!("Nonce (hex): {}", hex::encode(nonce));
    #[cfg(feature = "statystyki")]
    println!("Długość ciphertext: {}", ciphertext.len());

    // Deriwacja klucza
    let mut key = [0u8; 32];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key)
        .expect("Błąd generowania klucza PBKDF2");

    // Deszyfrowanie
    let cipher = XChaCha20Poly1305::new_from_slice(&key)
        .unwrap_or_else(|_| panic!("Nieprawidłowa długość klucza: {} bajtów", key.len()));

    cipher.decrypt(XNonce::from_slice(nonce), ciphertext)
        .unwrap_or_else(|e| panic!(
            "Błąd deszyfrowania: {}\nMożliwe przyczyny:\n- Błędne hasło\n- Uszkodzone dane\n- Nieprawidłowy format\n- Błędne offsety",
            e
        ))
}