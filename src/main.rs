use std::io;
use std::process::exit;
use num_bigint::{ToBigInt, BigInt};

use rsa_repl::{Config, key_gen};

fn main() {
    loop {
        repl();
    }
}

fn repl() {
    println!("\n1) Generate keys\n2) Encrypt a message with public key\n3) Encrypt a message with private key\n4) Decrypt a message with public key\n5) Decrypt a message with private key\n6) Exit program");
    let user_choice: i32 = loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error, failed to read response");

        let response : i32 = match response.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please inser a number");
                continue;
            },
        };

        if !(1..=6).contains(&response) {
            println!("Please insert a valid number");
            continue;
        };
        break response;
    };
    match user_choice {
        1 => key_gen(),
        2 => rsa(true, true),
        3 => rsa(true, false),
        4 => rsa(false, true),
        5 => rsa(false, false),
        _ => exit(0),
    }
}

fn rsa(encrypting: bool, public: bool) {
    println!("Insert message");
    let mut message: String = String::new();
    if let Err(e) = io::stdin().read_line(&mut message) {
        eprintln!("Error, failed to read response: {e}");
        return;
    }
    let d_or_e: BigInt = loop {
        print!("Insert ");
        if public {
            println!("e:");
        }
        else {
            println!("d:");
        };

        let mut response: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            eprintln!("Error, failed to read response: {e}");
            return;
        };

        let num: BigInt = match BigInt::parse_bytes(&Vec::from(response.trim()), 10) {
              Some(num) => break num,
              None => {
                println!("Inserted invalid value");
                continue;
              }, 
        };
    };
    let n: BigInt = loop {
        println!("Insert n:");
        let mut response: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            eprintln!("Error, failed to read response: {e}");
            return;
        };

        let num: BigInt = match BigInt::parse_bytes(&Vec::from(response.trim()), 10) {
              Some(num) => break num,
              None => {
                println!("Inserted invalid value");
                continue;
              }, 
        };
    };
    let config: Config = Config::build(message.trim(), d_or_e, n, encrypting);
    println!("Decrypted message: {}", config.run());
}
