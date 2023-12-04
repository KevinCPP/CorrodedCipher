
use crate::file::*;
use crate::input::*;
use crate::hashing::*;
use crate::password::*;
use crate::encryption::*;
use crate::decryption::*;
use crate::encryption_params::*;

const INVALID_OPTION: i64 = -1;

fn print_bytes_as_hex(data: Vec<u8>) {
    let hex_string: Vec<String> = data.iter()
                                       .map(|b| format!("{:02x}", b))
                                       .collect();
    let hex_string = hex_string.join(" ");
    println!("Bytes as hex: {}", hex_string);
}

fn print_string_as_hex(data: String) {
    let bytes = data.into_bytes();
    let hex_string: Vec<String> = bytes.iter()
                                       .map(|b| format!("{:02x}", b))
                                       .collect();
    let hex_string = hex_string.join(" ");
    println!("String as hex: {}", hex_string);
}


pub fn encryption_menu() {
    let menu_str =
        String::from("Encryption Menu:
        \r1. AES128
        \r2. AES192
        \r3. AES256");
    
    // get encryption method to use
    let input = to_i64(prompt(Some(menu_str), None)).unwrap_or(INVALID_OPTION);

    // get file to encrypt
    let input_file_prompt = String::from("Enter the directory of the file you want to encrypt:");
    let input_file_path = remove_special_chars(prompt(Some(input_file_prompt), None)).unwrap_or(String::new());

    // get password to encrypt file with
    let password_prompt = String::from("Enter a password:");
    let password = remove_special_chars(prompt(Some(password_prompt), None)).expect("invalid password entered");
    print_string_as_hex(password.clone());

    // get output destination
    let output_file_prompt = String::from("Enter the directory where you want to save the file:");
    let mut output_file_path = remove_special_chars(prompt(Some(output_file_prompt), None)).unwrap_or(String::new());
    
    if output_file_path.is_empty() {
        println!("Invalid output path provided! Using same path as input file!");
        output_file_path.push_str("_encrypted");
    }


    // Determine key length based on input provided
    let key_len = match input {
        1 => 16,
        2 => 24,
        3 => 32,
        _ => {
            println!("Invalid encryption scheme entered");
            return;
        }
    };
    
    // derive a key from the password
    let key = derive_key_from_password(&password, key_len).expect("Password to Key Derivation failed.");
    print!("key bytes: ");
    print_bytes_as_hex(key.clone());
    // generate a secure initialization vector
    let iv = generate_secure_iv(IV_LENGTH).expect("IV Generation Failed");
    // read in the file to be encrypted
    let binary_file = read_binary_file(input_file_path.clone()).expect("Failed to read the file.");
   
    // perform the actual encryption operation
    let encrypted_bin = match input {
        1 => encrypt_aes_128(&binary_file, &key, &iv).expect("Encryption failed."),
        2 => encrypt_aes_192(&binary_file, &key, &iv).expect("Encryption failed."),
        3 => encrypt_aes_256(&binary_file, &key, &iv).expect("Encryption failed."),
        _ => {
            println!("Invalid encryption scheme entered");
            return;
        }
    };
    
    print_bytes_as_hex(encrypted_bin.clone());
    // store the encrypted file
    write_binary_file(output_file_path.clone(), &encrypted_bin).expect("Writing file failed."); 
}

pub fn decryption_menu() {
    let menu_str =
        String::from("Decryption Menu:
        \r1. AES128
        \r2. AES192
        \r3. AES256");
    
    // get decryption method to use
    let input = to_i64(prompt(Some(menu_str), None)).unwrap_or(INVALID_OPTION);

    // get file to decrypt
    let input_file_prompt = String::from("Enter the directory of the file you want to decrypt:");
    let input_file_path = remove_special_chars(prompt(Some(input_file_prompt), None)).unwrap_or(String::new());

    // get password to decrypt file with
    let password_prompt = String::from("Enter a password:");
    let password = remove_special_chars(prompt(Some(password_prompt), None)).expect("invalid password entered");
    print_string_as_hex(password.clone());

    // get output destination
    let output_file_prompt = String::from("Enter the directory where you want to save the file:");
    let mut output_file_path = remove_special_chars(prompt(Some(output_file_prompt), None)).unwrap_or(String::new());
    
    if output_file_path.is_empty() {
        println!("Invalid output path provided! Using same path as input file!");
        output_file_path.push_str("_decrypted");
    }


    // Determine key length based on input provided
    let key_len = match input {
        1 => 16,
        2 => 24,
        3 => 32,
        _ => {
            println!("Invalid decryption scheme entered");
            return;
        }
    };
    
    // derive a key from the password
    let key = derive_key_from_password(&password, key_len).expect("Password to Key Derivation failed.");
    print!("key bytes: ");
    //print_bytes_as_hex(key.clone());
    // read in the file to be decrypted
    let binary_file = read_binary_file(input_file_path.clone()).expect("Failed to read the file.");
    //print_bytes_as_hex(binary_file.clone());
    
    // perform the actual decryption operation
    let decrypted_bin = match input {
        1 => decrypt_aes_128(&binary_file, &key).expect("Decryption failed."),
        2 => decrypt_aes_192(&binary_file, &key).expect("Decryption failed."),
        3 => decrypt_aes_256(&binary_file, &key).expect("Decryption failed."),
        _ => {
            println!("Invalid decryption scheme entered");
            return;
        }
    };

    // store the decrypted file
    write_binary_file(output_file_path.clone(), &decrypted_bin).expect("Writing file failed."); 
}


pub fn hashing_menu() {
    let menu_str = 
    String::from("Hashing Menu:
    \r1. Perform SHA256 hash");

    let input = to_i64(prompt(Some(menu_str), None)).unwrap_or(INVALID_OPTION);

    // get file to hash
    let input_file_prompt = String::from("Enter the directory of the file you want to hash:");
    let input_file_path = remove_special_chars(prompt(Some(input_file_prompt), None)).unwrap_or(String::new());

    let binary_file = read_binary_file(input_file_path.clone()).expect("Failed to read the file.");
    match sha256_hash(&binary_file) {
        Ok(h) => {
            let hex_string: String = h.iter().map(|byte| format!("{:02x}", byte)).collect();
            println!("SHA-256: {}", hex_string);
        },
        Err(e) => println!("Error: {}", e),
    }

//    print!("{:?}", sha256_hash(&binary_file))
}

pub fn display_main_menu() {
    let menu_str =
        String::from("Main Menu:
        \r1. Encryption
        \r2. Decryption
        \r3. Hashing
        \r4. Exit");

    let input = to_i64(prompt(Some(menu_str), None)).unwrap_or(INVALID_OPTION);

    match input {
        1 => encryption_menu(),
        2 => decryption_menu(),
        3 => hashing_menu(),
        _ => return
    };
}
