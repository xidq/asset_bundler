// use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, aead::*};
// use rand::Rng;
// use rand::rngs::OsRng;

//I was trying, but for now there's error coz buffor too long or something

use chacha20poly1305::{
    aead::{
        Aead, 
        KeyInit, 
        OsRng
    }, 
    XChaCha20Poly1305, 
    XNonce
};
use pbkdf2::pbkdf2;
use hmac::Hmac;
use sha2::Sha256;
use chacha20poly1305::aead::rand_core::RngCore;
use chacha20poly1305::AeadCore;

const SALT_LEN: usize = 16;
const PBKDF2_ITERATIONS: u32 = 100_000;

pub fn encrypt_chacha(data: &mut [u8], password: &str) {
    // Generowanie soli
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    
    // Generowanie nonce (192 bity)
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    
    // Deriwacja klucza
    let mut key = [0u8; 32];
    let _ =pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        &salt,
        PBKDF2_ITERATIONS,
        &mut key
    );
    
    // Szyfrowanie
    let cipher = XChaCha20Poly1305::new_from_slice(&key).expect("Nieprawidłowa długość klucza");
    let ciphertext = cipher.encrypt(&nonce, &*data).expect("Błąd szyfrowania");
    
    // Łączenie soli, nonce i ciphertext + tag
    let result = [salt.as_slice(), nonce.as_slice(), &ciphertext].concat();
    
    // Nadpisywanie bufora
    data[..result.len()].copy_from_slice(&result);
}

fn decrypt(data: &mut [u8], password: &str) {
    // Sprawdzenie minimalnej długości
    if data.len() < SALT_LEN + XNonce::default().len() + 16 {
        panic!("Zbyt krótkie dane");
    }
    
    // Wydzielenie składowych
    let (salt, rest) = data.split_at(SALT_LEN);
    let (nonce, ciphertext) = rest.split_at(XNonce::default().len());
    
    // Deriwacja klucza
    let mut key = [0u8; 32];
    let _ = pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut key
    );
    
    // Deszyfrowanie
    let cipher = XChaCha20Poly1305::new_from_slice(&key).expect("Nieprawidłowa długość klucza");
    let plaintext = cipher.decrypt(nonce.into(), ciphertext).expect("Błąd deszyfrowania");
    
    // Nadpisywanie bufora
    data[..plaintext.len()].copy_from_slice(&plaintext);
}


// fn encrypt(plaintext: &[u8], password:&str, nonce: &Nonce)  {
//     let key = ChaCha20Poly1305::generate_key(&mut rand::rngs::OsRng);
//     let cipher = ChaCha20Poly1305::new(&key);
//     let nonce = ChaCha20Poly1305::generate_nonce(&mut rand::rngs::OsRng); // 96-bits; unique per message
//     let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
//     let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
//     assert_eq!(&plaintext, b"plaintext message");
//     // let cipher = ChaCha20Poly1305::new(key);
//     // cipher.encrypt(nonce, plaintext)
//     // .expect("encryption failure!");
// }

// fn decrypt(ciphertext: &[u8], password:&str, nonce: &Nonce) {

//     let cipher = ChaCha20Poly1305::new(password);
//     cipher.decrypt(nonce, ciphertext)
//     .expect("decryption failure!");
// }