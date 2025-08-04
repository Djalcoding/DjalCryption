use std::{
    env,
    process::{self},
};

use dencrypting::encrypting::Query;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut decrypt = false;
    let mut file_path: String = String::new();
    let mut skip = true;
    let mut key = 1;
    for (i, arg) in args.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        };
        if arg == "-h" || arg == "--help" {
            print_help_screen();
            process::exit(0);
        } else if arg == "-v" || arg == "--version" {
            eprintln!("Djalcryption - 1.0");
            process::exit(0);
        } else if arg == "-d" || arg == "--decrypt" {
            decrypt = true;
        } else if arg == "-e" || arg == "--encrypt" {
            decrypt = false;
        } else if arg == "-i" || arg == "--input" {
            if i + 1 != args.len() {
                file_path = args[i + 1].clone();
                skip = true;
            }
        } else if arg == "-k" || arg == "--key" {
            if i + 1 != args.len() {
                key = args[i + 1].clone().parse::<u8>().unwrap_or_else(|err| {
                    eprintln!("Invalid Key \nError : {err}");
                    process::exit(103); //invalid key
                });
                skip = true;
            }
        } else {
            eprintln!("Argument '{arg}' is not supposed to be there");
            process::exit(101); // argument overflow
        }
    }

    if file_path.is_empty() {
        eprintln!("No input file specified");
        process::exit(100);
    }


    if decrypt{
        let decrypting:Query = Query::from(file_path, true);
        println!("{}", decrypting.get_decrypted(key));
    }
    else{
        let encrypting:Query = Query::from(file_path, false);
        println!("{}", encrypting.get_encrpyted(key));
    }
}

fn print_help_screen() {
    eprintln!("-h, --help    -> show this screen");
    eprintln!("-v, --version -> show programm version");
    eprintln!("-e, --encrypt -> tell the program to encrpyt the file (default)");
    eprintln!("-d, --decrypt -> tell the program to decrypt the file");
    eprintln!("-i, --input   -> tell the program what file to use");
    eprintln!("-k, --key     -> tell the programm what key to use, 1 is the default, 0 is the minimum and 255 is the max");
}
