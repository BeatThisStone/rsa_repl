use std::io;

use rsa_repl::Config;

fn main() {
    println!("1) Generate keys\n2) Encrypt a message with public key\n3) Encrypt a message with private key\n4) Decrypt a message with public key\n5) Decrypt a message with private: key");

    let user_choice: u32 = loop {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read response");

        let response : u32 = match response.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please inser a number");
                continue;
            },
        };

        if response > 5 || response < 1 {
            println!("Please insert a valid number");
            continue;
        };
        break response;
    };
    println!("{user_choice}");
}
