use num_bigint::{ToBigInt, BigInt};

pub struct Config {
    message: String,
    d_or_e: BigInt,
    n: BigInt,
    encrypting: bool,
}

impl Config {
    pub fn build(message: String, d_or_e: u32, n: u32, encrypting: bool) -> Config {
        Config { message, d_or_e: (d_or_e.to_bigint().unwrap()), n: (n.to_bigint().unwrap()), encrypting}
    }
    pub fn run(self) -> String {
        return if self.encrypting {
            self.encrypt()
        } else {
            self.decrypt()
        }
    }
    fn encrypt(self) -> String {
        let mut encrypted_string: String = String::new();
        Vec::from(self.message)
            .iter()
            .for_each(|x| {
                     encrypted_string.push_str(
                        &x
                        .to_bigint()
                        .unwrap()
                        .modpow(&self.d_or_e, &self.n)
                        .to_str_radix(10)
                    );
                    encrypted_string.push(';');
                }
            );
        return encrypted_string;
    }
    fn decrypt(self) -> String {
        return String::new();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_with_public() {
        let config = Config::build("00ciao".to_string(), 5, 119, true);
        println!("{}", config.encrypt());
    }
}
