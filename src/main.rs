use std::io;
use std::process::exit;
use num_bigint::BigInt;
use log::{Level, info, warn, error};



use rsa_repl::{Config, key_gen};

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    info!("Choose a log level:\n1) INFO\n2) WARN\n3) ERROR");
    let user_choice: i32 = loop {
        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            error!("Error, failed to read response: {e}");
            exit(1);
        }

        let response : i32 = match response.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                warn!("Please inser a number");
                continue;
            },
        };

        if !(1..=6).contains(&response) {
            warn!("Please insert a valid number");
            continue;
        };
        break response;
    };
    let level = match user_choice {
        1 => Level::Info,
        2 => Level::Warn,
        _ => Level::Error,
    };
    log::set_max_level(level.to_level_filter());
    loop {
        repl();
    }
}

fn repl() {
    info!("\n1) Generate keys\n2) Encrypt a message with public key\n3) Encrypt a message with private key\n4) Decrypt a message with public key\n5) Decrypt a message with private key\n6) Exit program");
    let user_choice: i32 = loop {
        let mut response = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            error!("Error, failed to read response: {e}");
            exit(1);
        }

        let response : i32 = match response.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                warn!("Please inser a number");
                continue;
            },
        };

        if !(1..=6).contains(&response) {
            warn!("Please insert a valid number");
            continue;
        };
        break response;
    };
    match user_choice {
        1 => {
            info!("Insert how many bites a prime number should have, from 8 to 4096");
            let user_choice = loop {
                let mut resposne : String = String::new();
                if let Err(e) = io::stdin().read_line(&mut resposne) {
                    error!("Failed to read resposne, {e}");
                    return;
                }
                match resposne.trim().parse::<usize>() {
                    Ok(num) => {
                        if (8..=4096).contains(&num) {
                            break num;
                        }
                        else {
                            warn!("Number outside of MIN value {} and MAX value 4096", usize::MIN);
                        }
                    },
                    Err(_) => {
                        warn!("Number outside of MIN value {} and MAX value 4096", usize::MIN);
                    }
                };
                    
            };
            key_gen(user_choice);

        },
        2 => rsa(true, true),
        3 => rsa(true, false),
        4 => rsa(false, true),
        5 => rsa(false, false),
        _ => exit(0),
    }
}

fn rsa(encrypting: bool, public: bool) {
    info!("Insert message");
    let mut message: String = String::new();
    if let Err(e) = io::stdin().read_line(&mut message) {
        info!("Error, failed to read response: {e}");
        return;
    }
    let d_or_e: BigInt = loop {
        if public {
            info!("Insert e:");
        }
        else {
            info!("Insert d:");
        }

        let mut response: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            error!("Error, failed to read response: {e}");
            return;
        };

        match BigInt::parse_bytes(&Vec::from(response.trim()), 10) {
              Some(num) => break num,
              None => {
                warn!("Inserted invalid value");
                continue;
              }, 
        };
    };
    let n: BigInt = loop {
        info!("Insert n:");
        let mut response: String = String::new();
        if let Err(e) = io::stdin().read_line(&mut response) {
            error!("Error, failed to read response: {e}");
            return;
        };

        match BigInt::parse_bytes(&Vec::from(response.trim()), 10) {
            Some(num) => break num,
            None => {
                warn!("Inserted invalid value");
                continue;
            }, 
        };
    };
    let config: Config = Config::build(message.trim(), d_or_e, n, encrypting);
    if encrypting {
        info!("Encrypted message: {}", config.run());
    } 
    else {
        info!("Decrypted message: {} ", config.run());
    }
}
