use num_bigint::{ToBigInt, BigInt};

pub struct Config {
    message: String,
    d_or_e: BigInt,
    n: BigInt,
    encrypting: bool,
}

impl Config {
    pub fn build(message: &str, d_or_e: BigInt, n: BigInt, encrypting: bool) -> Config {
        Config { message: (message.to_string()), d_or_e, n, encrypting}
    }
    pub fn run(self) -> String {
        if self.encrypting {
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
        encrypted_string
    }
    fn decrypt(self) -> String {
       self.message
            .split(';')
	    .filter(|&x| !x.is_empty() && !x.chars().any(|s| !s.is_numeric()))
            .map(|x| 
                BigInt::parse_bytes(&Vec::from(x), 10)
                    .unwrap()
                    .modpow(&self.d_or_e, &self.n)
                    .to_str_radix(10)
                    .parse::<u8>()
                    .unwrap_or(u8::MIN) as char // not sure the unwrap_or is necessary
                )
            .collect()
    }
}

pub fn key_gen() {
    ()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypting() {
        let config = Config::build("ciao".to_string(), 5, 119, true);
        assert_eq!(config.run(), "29;56;20;76;".to_string());
    }
    #[test]
    fn decrypting() {
        let config = Config::build("29;56;20;76".to_string(), 269, 119, false);
        assert_eq!(config.run(), "ciao".to_string());
    }
}
