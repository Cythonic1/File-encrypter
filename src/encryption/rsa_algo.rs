
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rsa::pkcs1::ToRsaPrivateKey; // Add this import for PKCS#1
use rsa::pkcs1::ToRsaPublicKey; // Import this for public key PEM
use rand::rngs::OsRng;

pub fn encrypted() {
    // Use OsRng for cryptographic randomness
    let mut rng = OsRng;
    
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to create priv_key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Print public and private keys in PEM format
    let pub_key_pem = pub_key.to_pkcs1_pem().expect("Failed to encode public key to PEM");
    let priv_key_pem = priv_key.to_pkcs1_pem().expect("Failed to encode private key to PEM");

    println!("The public key in PEM format:\n{:#?}", pub_key_pem);
    println!("The private key in PEM format:\n{:#?}", priv_key_pem);

    let data = b"Hello World";

    // Encrypt the data
    let enc_data = pub_key
        .encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..])
        .expect("Failed to encrypt");
    println!("The encrypted data is: {:?}", enc_data);

    // Decrypt the data
    let dec_data = priv_key
        .decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data)
        .expect("Failed to decrypt");
    println!("The decrypted data is: {:?}", String::from_utf8(dec_data).unwrap());
}

