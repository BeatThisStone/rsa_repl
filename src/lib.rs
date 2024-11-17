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
    fn decrypt(self) -> Result<String, String> {
        let parse_array: Result<Vec<u128>, _> = self.message
            .split(';')
            .map(|x| x.parse::<u128>())
            .collect();
        if let Err(e) = parse_array {
            return Err("Message could not be parsed, {e}".to_string());
        }
        let num_array: Vec<BigInt> = parse_array
            .unwrap()
            .iter()
            .map(|x| 
                x
                .to_bigint()
                .unwrap()
                .modpow(&self.d_or_e, &self.n))
            .collect();
        return Ok(String::new());
    }
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
