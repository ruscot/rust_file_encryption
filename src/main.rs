use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key // Or `Aes128Gcm`
};
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use aes::cipher::generic_array::GenericArray;

use encrypt_file::{
    Config,
    parse_config,
    create_key,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

    let bytes_password: Vec<u8> = create_key(config.password);
    
    let bytes_contents = fs::read(&config.file_path)
            .expect("Should have been able to read the file");
    
    let key = Key::<Aes256Gcm>::from_slice(&bytes_password);

    let cipher = Aes256Gcm::new(&key);
    let nonce = GenericArray::from([42u8; 12]);

    match config.command.as_str() {
        "encrypt" => {
            let ciphertext = cipher.encrypt(&nonce, bytes_contents.as_ref());
            let ciphertext = match ciphertext {
                Ok(ciphertext) => ciphertext,
                Err(error) => panic!("Problem ciphertext encryption: {:?}", error),
            };
            let mut file = File::create(config.file_path)?;
            file.write(&ciphertext)?;
            file.flush()?;
        },
        "decrypt" => {
            let plaintext = cipher.decrypt(&nonce, bytes_contents.as_slice());
            let plaintext = match plaintext {
                Ok(plaintext) => plaintext,
                Err(error) => panic!("Problem ciphertext decryption: {:?}", error),
            };
            let mut file = File::create(config.file_path)?;
            file.write(&plaintext)?;
            file.flush()?;
        },
        _ => {
            panic!("Command not valid encrypt/decrypt allowed");
        }
    }

    Ok(())
}
