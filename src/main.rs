// TODO : Implement the command line argumrnts managment  : Done
// TODO : make sure the encryption works fine.
// TODO : Implement RSA too.

use clap::{arg, command, value_parser};
use std::fs;
use std::path::PathBuf;
mod encryption;


fn main() {
    // Define the command-line arguments using clap
    let mut cmd = command!()
        .arg(
            arg!(-a --aes_256 <FILE> "AES-256 encryption with input file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-r --rsa <FILE> "RSA encryption with input file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-o --output <FILE> "Output file for encryption")
                .required(false)
                .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-f --file <FILE_NAME> "Create a new file")
                .required(false)
                .value_parser(value_parser!(String))
        ).arg(
            arg!(-k --key <hex_key> "hex key")
                .required(false)
                .value_parser(value_parser!(String))
                .help("Must when decrypt") 
        ).arg (
            arg!(-d --aes_256_decrypyion <FILE> "AES-256 decryption file")
                .required(false)
                .value_parser(value_parser!(PathBuf))
        );

    let matches = cmd.clone().get_matches();

    // Ensure the user entered some arguments
    if !(matches.contains_id("aes_256")
        || matches.contains_id("rsa")
        || matches.contains_id("output")
        || matches.contains_id("file"))
    {
        eprintln!("Error: You must specify at least one argument.");
        cmd.print_help().unwrap();  // Print the help message
        std::process::exit(1);
    }

    // Check for dependency: if AES-256 is specified, output must also be provided
    if matches.contains_id("aes_256") && !matches.contains_id("output") {
        eprintln!("Error: The '-a/--aes_256' argument requires '-o/--output'.");
        cmd.print_help().unwrap();  // Print the help message
        std::process::exit(1);
    }

    if matches.contains_id("aes_256_decrypyion") && (!matches.contains_id("key") || !matches.contains_id("output")){
        eprintln!("Please Enter the -o <FILE> -k <Key> ");
        cmd.print_help().unwrap();
        std::process::exit(1);
    } 
    // Commented Until compelte implemntation
    //// Example usage of the matches
    //if let Some(aes_file) = matches.get_one::<PathBuf>("aes_256") {
    //    println!("You want AES-256 encryption with file: {:?}", aes_file);
    //    if let Some(output_file) = matches.get_one::<PathBuf>("output") {
    //        println!("Output file for AES-256 encryption: {:?}", output_file);
    //        // encryption::aes_256_algo::encrypted(aes_file, output_file);
    //    }
    //}

    if let Some(aes_file) = matches.get_one::<PathBuf>("aes_256") {
        println!("You want RSA encryption with file: {:?}", aes_file);
        if let Some(aes_output) = matches.get_one::<PathBuf>("output"){
            println!("Am here");
            encryption::aes_256_algo::encrypted(aes_file, aes_output);
        }        
    }

    if let Some(file_name) = matches.get_one::<String>("file") {
        println!("Creating file: {:?}", file_name);
        match fs::File::create(file_name) {
            Ok(_) => println!("File created successfully."),
            Err(e) => eprintln!("Error creating file: {}", e),
        }
    }

    if let Some(aes_256_decrypyion) = matches.get_one::<PathBuf>("aes_256_decrypyion"){
         if let Some(key) = matches.get_one::<String>("key"){
                println!("You have the key");
                if let Some(output_file) = matches.get_one::<PathBuf>("output") {
                    println!("Output file for AES-256 encryption: {:?}", output_file);
                    encryption::aes_256_algo::decrypted(aes_256_decrypyion, output_file, key);
                }
            }

    }

}

