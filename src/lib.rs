use num_primes::Generator;
use num_bigint::{BigInt, ToBigInt};
use log::info;

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

pub fn key_gen(prime_size: usize) {
    let p = Generator::new_prime(prime_size);
    info!("p: {p}");
    let q = loop {
        let candidate = Generator::new_prime(prime_size);
        if p != candidate {
            break candidate;
        }
    };
    info!("q: {q}");
    let one: u32 = 1; 
    let v = (&p-one) * (&q-one);
    info!("v: {v}");
    let n = &p * &q;
    info!("n: {n}");
    let e = loop {
        let e = Generator::new_prime(prime_size);
        if e != v && e < v {
            break e;   
       } 
    };
    info!("e: {e}");
    let e = BigInt::parse_bytes(e.to_str_radix(10).as_bytes(), 10).unwrap();
    let v = BigInt::parse_bytes(v.to_str_radix(10).as_bytes(), 10).unwrap();
    let d = e.modinv(&v).unwrap();
    info!("d: {d}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypting() {
        let config = Config::build("ciao", "5".parse::<BigInt>().unwrap(), "119".parse::<BigInt>().unwrap(), true);
        assert_eq!(config.run(), "29;56;20;76;".to_string());
    }
    #[test]
    fn decrypting() {
        let config = Config::build("29;56;20;76", "269".parse::<BigInt>().unwrap(), "119".parse::<BigInt>().unwrap(), false);
        assert_eq!(config.run(), "ciao".to_string());
    }
}
