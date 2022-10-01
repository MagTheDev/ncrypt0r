use std::{fs::{File, OpenOptions, remove_file}, io::{Read, Write}};

use anyhow::Result;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn encrypt_bytes(s: Vec<u8>, key: String) -> String {
    let mc = new_magic_crypt!(key, 256);

    return mc.encrypt_bytes_to_base64(s.as_slice())
}

fn decrypt_string(s: String, key: String) -> Result<Vec<u8>> {
    let mc = new_magic_crypt!(key, 256);

    Ok(mc.decrypt_base64_to_bytes(s)?)
}

fn encrypt_file(filename: String, key: String) -> Result<()> {
    let mut file = File::open(&filename)?;

    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents)?;

    let encrypted_file_contents = encrypt_bytes(file_contents, key + "ncrypt0r");

    remove_file(&filename)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;
    
    file.write_all(encrypted_file_contents.as_bytes())?;

    Ok(())
}

fn decrypt_file(filename: String, key: String) -> Result<()> {
    let mut file = File::open(&filename)?;

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let decrypted_file_contents = decrypt_string(file_contents, key + "ncrypt0r")?;

    remove_file(&filename)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;
    
    file.write_all(decrypted_file_contents.as_slice())?;

    return Ok(())
}

fn main() {
    let key = "password";

    // encrypt_file("unencrypted_file".to_string(), key.to_string()).unwrap();
    decrypt_file("unencrypted_file".to_string(), key.to_string()).unwrap();
}
