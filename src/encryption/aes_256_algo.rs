use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::fs;
use std::path::PathBuf;
fn hex_str_to_u8_array(hex_str: &str) -> Result<[u8; 32], &'static str> {
    let hex_str = hex_str.trim().to_lowercase();
    if hex_str.len() != 64 {
        return Err("Hex string must be exactly 64 characters long");
    }

    let mut bytes = [0u8; 32];
    for (i, chunk) in hex_str.as_bytes().chunks(2).enumerate() {
        let hex_byte = std::str::from_utf8(chunk).unwrap();
        bytes[i] = u8::from_str_radix(hex_byte, 16).map_err(|_| "Invalid hex byte")?;
    }
    println!("{:?}" ,bytes);
    Ok(bytes)
}


type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypted(file_path: &PathBuf, output_file : &PathBuf){
    let key = rand::thread_rng().gen::<[u8; 32]>();
    let iv = rand::thread_rng().gen::<[u8; 16]>();
   
    let file_content = fs::read(file_path).expect("Faild To read the file"); 
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).expect("Faild To create the cipher");

    let cipher_content = cipher.encrypt_vec(&file_content);
    
    let mut encrypted_with_iv = Vec::new();
    encrypted_with_iv.extend_from_slice(&iv);
    encrypted_with_iv.extend_from_slice(&cipher_content);

    let _ = fs::write(output_file ,encrypted_with_iv).expect("Faild to write the Encryption to the file");
       // Convert the key to a hexadecimal string
    let key_hex: String = key.iter().map(|b| format!("{:02x}", b)).collect();
    println!("Encryption completed! Encrypted data written to {:?}", output_file);
    println!("Here is your key keep it safe: {}", key_hex);
}


pub fn decrypted(enc_path : &PathBuf, output_file: &PathBuf, key_str: &String){
    // Read the enc data.
    let enc_data = fs::read(enc_path).expect("Faild To read data from file");

    let key:[u8; 32] = hex_str_to_u8_array(key_str).expect("Please Enter valid key");
    // Extract the iv
    let (iv, enc_content)= enc_data.split_at(16); // Extract the first 16 bytes as it is the iv.
    
    // Prepare the cipher
    let cipher = Aes256Cbc::new_from_slices(&key,iv).expect("Faild To create the cipher");

    // Decrypt the data
    let plain_text = cipher.decrypt_vec(enc_content).expect("Faild to decrypted");

    // Write the plain text into the file;
    let _ = fs::write(output_file, plain_text).expect("Faild to write the data");


}
