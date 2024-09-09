// TODO : Implement the command line argumrnts managment 
// TODO : make sure the encryption works fine.
// TODO : Implement RSA too.
use std::fs;
use std::env;
mod encryption;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        process::exit(1);
    }

    let mut output: Option<&String> = None; // Initialize to store the output file path

    for (index, arg) in args.iter().enumerate() {
        
        if arg == "-r" {
            if index + 1 < args.len() {
                println!("You want RSA encryption");
                encryption::rsa_algo::encrypted(&args[index + 1]);
                process::exit(1);
            } else {
                eprintln!("Missing argument for -r");
                help();
                process::exit(1);
            }
        } else if arg == "-f" {
            if index + 1 < args.len() {
                println!("So you want to create a File");
                match fs::File::create(&args[index + 1]) {
                    Ok(_) => println!("The file has been created"),
                    Err(e) => eprintln!("Something went wrong: {}", e),
                }
                process::exit(1);
            } else {
                eprintln!("Missing argument for -f");
                help();
                process::exit(1);
            }
        } else if arg == "-a" {
            // Check if user provided the output file
            if let Some(out) = output {
                if index + 1 < args.len() {
                    println!("You want AES-256 encryption");
                    encryption::aes_256_algo::encrypted(&args[index + 1], out);
                    process::exit(1);
                } else {
                    eprintln!("Missing argument for -a");
                    help();
                    process::exit(1);
                }
            } else {
                eprintln!("Error: You must specify an output file with -o when using AES encryption (-a)");
                process::exit(1);
            }
        } else if arg == "-o" {
            // Save the output file for later use with -a
            if index + 1 < args.len() {
                output = Some(&args[index + 1]);
            } else {
                eprintln!("Missing argument for -o");
                process::exit(1);
            }
        } else if arg == "-h" || arg == "-help" {
            help();
            process::exit(1);
        }
    }
}

fn help() {
    let menu: String = String::from("Usage toucher <arguments> \n -r <path> RSA Encryption \n -f <file_name> create file \n -a <file_path> AES-256 encryption (requires -o <output_file>)");
    println!("Menu \n {:#}", menu);
}

