use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::fs;




type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypted(file_path: &String, output_file : &String){
    let key = rand::thread_rng().gen::<[u8; 32]>();
    let iv = rand::thread_rng().gen::<[u8; 16]>();
   
    let output = "/home/pythonic/as.txt";
    let file_content = fs::read(file_path).expect("Faild To read the file"); 
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).expect("Faild To create the cipher");

    let cipher_content = cipher.encrypt_vec(&file_content);
    
    let mut encrypted_with_iv = Vec::new();
    encrypted_with_iv.extend_from_slice(&iv);
    encrypted_with_iv.extend_from_slice(&cipher_content);

    let _ = fs::write(output ,encrypted_with_iv).expect("Faild to write the Encryption to the file");
    println!("Encryption completed! Encrypted data written to {}", output_file);
    println!("Here is your key keep it safe : {:?}", key);
}


pub fn decrypted(enc_path : &String, output_file: &String, key: &[u8; 32]){
    // Read the enc data.
    let enc_data = fs::read(enc_path).expect("Faild To read data from file");

    // Extract the iv
    let (iv, enc_content)= enc_data.split_at(16); // Extract the first 16 bytes as it is the iv.
    
    // Prepare the cipher
    let cipher = Aes256Cbc::new_from_slices(key,iv).expect("Faild To create the cipher");

    // Decrypt the data
    let plain_text = cipher.decrypt_vec(enc_content).expect("Faild to decrypted");

    // Write the plain text into the file;
    let _ = fs::write(output_file, plain_text).expect("Faild to write the data");


}
