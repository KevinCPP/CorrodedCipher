#![feature(string_remove_matches)]    

pub mod file;
pub mod menu;
pub mod input;
pub mod hashing;
pub mod password;
pub mod encryption;
pub mod decryption;
pub mod encryption_params;

use menu::display_main_menu;
use clap::{App, Arg, SubCommand};
use file::*;
use hashing::*;
use password::*;
use encryption::*;
use decryption::*;
use encryption_params::*;

fn main() {
    let matches = App::new("My Program")
        .version("1.0")
        .author("Your Name")
        .about("Handles encryption, decryption, and hashing")
        .subcommand(SubCommand::with_name("-E")
            .about("Encryption")
            .arg(Arg::with_name("A")
                .long("algorithm")
                .takes_value(true)
                .possible_values(&["AES128", "AES192", "AES256"])
                .help("Encryption algorithm"))
            .arg(Arg::with_name("P")
                .long("password")
                .takes_value(true)
                .help("Password for encryption"))
            .arg(Arg::with_name("I")
                .long("input")
                .takes_value(true)
                .help("Input file path"))
            .arg(Arg::with_name("O")
                .long("output")
                .takes_value(true)
                .help("Output destination")))
        .subcommand(SubCommand::with_name("-D")
            .about("Decryption")
            .arg(Arg::with_name("A").long("algorithm").takes_value(true).possible_values(&["AES128", "AES192", "AES256"]).help("Decryption algorithm"))
            .arg(Arg::with_name("P").long("password").takes_value(true).help("Password for decryption"))
            .arg(Arg::with_name("I").long("input").takes_value(true).help("Input file path"))
            .arg(Arg::with_name("O").long("output").takes_value(true).help("Output destination")))
        .subcommand(SubCommand::with_name("-H")
            .about("Hashing")
            .arg(Arg::with_name("I")
                .long("input")
                .takes_value(true)
                .help("Input file path")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("-E") {
        // Extract the values for each argument
        let algorithm = matches.value_of("A");
        let password = matches.value_of("P");
        let input_file = matches.value_of("I");
        let output_dir = matches.value_of("O");

        // Check if all necessary arguments are provided
        if algorithm.is_none() || password.is_none() || input_file.is_none() || output_dir.is_none() {
            println!("Invalid input: -A, -P, -I, and -O options are required for encryption.");
            return;
        }

        // Now you can unwrap these because you know they exist
        let algorithm = algorithm.unwrap();
        let password = password.unwrap();
        let input_file = input_file.unwrap();
        let output_dir = output_dir.unwrap();

        // Determine key length based on the selected algorithm
        let key_len = match algorithm {
            "AES128" => 16,
            "AES192" => 24,
            "AES256" => 32,
            _ => {
                println!("Invalid encryption algorithm.");
                return;
            }
        };

        // Derive a key from the password
        let key = derive_key_from_password(password, key_len).expect("Password to Key Derivation failed.");

        // Generate a secure initialization vector
        let iv = generate_secure_iv(IV_LENGTH).expect("IV Generation Failed");

        // Read in the file to be encrypted
        let binary_file = read_binary_file(input_file.to_string()).expect("Failed to read the file.");

        // Perform the actual encryption operation
        let encrypted_bin = match algorithm {
            "AES128" => encrypt_aes_128(&binary_file, &key, &iv).expect("Encryption failed."),
            "AES192" => encrypt_aes_192(&binary_file, &key, &iv).expect("Encryption failed."),
            "AES256" => encrypt_aes_256(&binary_file, &key, &iv).expect("Encryption failed."),
            _ => {
                println!("Invalid encryption scheme entered");
                return;
            }
        };

        // Store the encrypted file
        write_binary_file(output_dir.to_string(), &encrypted_bin).expect("Writing file failed.");        
    } else if let Some(matches) = matches.subcommand_matches("-D") {
        // Extract the values for each argument
        let algorithm = matches.value_of("A");
        let password = matches.value_of("P");
        let input_file = matches.value_of("I");
        let output_dir = matches.value_of("O");

        // Check if all necessary arguments are provided
        if algorithm.is_none() || password.is_none() || input_file.is_none() || output_dir.is_none() {
            println!("Invalid input: -A, -P, -I, and -O options are required for decryption.");
            return;
        }

        // Now you can unwrap these because you know they exist
        let algorithm = algorithm.unwrap();
        let password = password.unwrap();
        let input_file = input_file.unwrap();
        let output_dir = output_dir.unwrap();

        // Determine key length based on the selected algorithm
        let key_len = match algorithm {
            "AES128" => 16,
            "AES192" => 24,
            "AES256" => 32,
            _ => {
                println!("Invalid decryption algorithm.");
                return;
            }
        };

        // Derive a key from the password
        let key = derive_key_from_password(password, key_len).expect("Password to Key Derivation failed.");

        // Read in the file to be decrypted
        let binary_file = read_binary_file(input_file.to_string()).expect("Failed to read the file.");

        // Perform the actual decryption operation
        let decrypted_bin = match algorithm {
            "AES128" => decrypt_aes_128(&binary_file, &key).expect("Encryption failed."),
            "AES192" => decrypt_aes_192(&binary_file, &key).expect("Encryption failed."),
            "AES256" => decrypt_aes_256(&binary_file, &key).expect("Encryption failed."),
            _ => {
                println!("Invalid decryption scheme entered");
                return;
            }
        };

        // Store the decrypted file
        write_binary_file(output_dir.to_string(), &decrypted_bin).expect("Writing file failed.");        
        // Handle decryption
        // Similar handling as decryption
    } else if let Some(matches) = matches.subcommand_matches("-H") {
        let input_file = matches.value_of("I");

        if input_file.is_none() {
            println!("Invalid input: -I option is required for hashing");
            return;
        }

        let input_file = input_file.unwrap();
        
        let binary_file = read_binary_file(input_file.to_string()).expect("Failed to read the file.");

        match sha256_hash(&binary_file) {
            Ok(h) => {
                let hex_string: String = h.iter().map(|byte| format!("{:02x}", byte)).collect();
                println!("SHA-256: {}", hex_string);
            },
            Err(e) => println!("Error: {}", e),
        }

    } else {
        display_main_menu()
    }
}


//fn main() {
//    let matches = App::new("Corroded Cipher")
//        .version("1.0")
//        .author("Your Name")
//        .about("Handles encryption, decryption, and hashing")
//        .subcommand(Subcommand::new("E")
//            .about("Encryption")
//            .arg(Arg::new("A")
//                .long("algorithm")
//                .takes_value(true)
//                .possible_values(&["AES128", "AES192", "AES256"])
//                .help("Encryption algorithm"))
//            .arg(Arg::new("P")
//                .long("password")
//                .takes_value(true)
//                .help("Password for encryption"))
//            .arg(Arg::new("I")
//                .long("input")
//                .takes_value(true)
//                .help("Input file path"))
//            .arg(Arg::new("O")
//                .long("output")
//                .takes_value(true)
//                .help("Output destination")))
//        .subcommand(Subcommand::new("D")
//            .about("Decryption")
//            .arg(Arg::new("A").long("algorithm").takes_value(true).possible_values(&["AES128", "AES192", "AES256"]).help("Decryption algorithm"))
//            .arg(Arg::new("P").long("password").takes_value(true).help("Password for decryption"))
//            .arg(Arg::new("I").long("input").takes_value(true).help("Input file path"))
//            .arg(Arg::new("O").long("output").takes_value(true).help("Output destination")))
//        .subcommand(Subcommand::new("H")
//            .about("Hashing")
//            .arg(Arg::new("I")
//                .long("input")
//                .takes_value(true)
//                .help("Input file path")))
//        .get_matches();
//
//    if let Some(matches) = matches.subcommand_matches("E") {
//        // Handle encryption
//        // Use matches.value_of("A"), matches.value_of("P"), etc. to get the argument values
//    } else if let Some(matches) = matches.subcommand_matches("D") {
//        // Handle decryption
//        // Similar handling as encryption
//    } else if let Some(matches) = matches.subcommand_matches("H") {
//        // Handle hashing
//        // Use matches.value_of("I") to get the input file path
//    } else {
//        display_main_menu()
//    }
//
//}

