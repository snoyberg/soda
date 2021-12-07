use std::{
    fmt::Display,
    io::{Read, Write},
    str::FromStr,
};

use anyhow::*;
use clap::Parser;
use once_cell::sync::Lazy;
use sodiumoxide::crypto::{
    box_::{self, PublicKey, SecretKey},
    sealedbox,
};

fn main() -> Result<()> {
    match SubCommand::parse() {
        SubCommand::Generate => {
            let (public, private) = new_keypair();
            println!("Public key (send to others for encrypting): {}", public);
            println!(
                "Private key (keep for yourself for decrypting): {}",
                private
            );
        }
        SubCommand::Encrypt { public, value } => {
            let value = value.map_or_else(get_stdin_bytes, |s| Ok(s.into_bytes()))?;
            println!("{}", public.encrypt(&value));
        }
        SubCommand::Decrypt { private, value } => {
            let value = value.map_or_else(get_stdin_string, Ok)?;
            let decrypted = private.decrypt(&value)?;
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            stdout.write_all(&decrypted)?;
        }
    }
    Ok(())
}

#[derive(clap::Parser)]
#[clap(version = VERSION_SHA.as_str())]
pub enum SubCommand {
    /// Generate a new keypair
    Generate,
    /// Encrypt a message for the given public key
    Encrypt {
        /// Public key. Only the owner of the corresponding private key can decrypt.
        public: SodaPublic,
        /// Value. If omitted, read from stdin
        value: Option<String>,
    },
    /// Decrypt a message using the given private key
    Decrypt {
        private: SodaPrivate,
        /// Encrypted message. If omitted, read from stdin
        value: Option<String>,
    },
}

static VERSION_SHA: Lazy<String> = Lazy::new(|| {
    let pkgver = env!("CARGO_PKG_VERSION");
    match option_env!("VERGEN_GIT_SHA") {
        None => pkgver.to_owned(),
        Some(gitsha) => format!("{} (Git SHA1 {})", pkgver, gitsha),
    }
});

fn get_stdin_string() -> Result<String> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn get_stdin_bytes() -> Result<Vec<u8>> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;
    Ok(buffer)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SodaPublic {
    inner: PublicKey,
}

impl FromStr for SodaPublic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("sodapub").context("Not a soda public key")?;
        let bytes = sodiumoxide::hex::decode(s)
            .ok()
            .context("Not hex encoded")?;
        let inner = PublicKey::from_slice(&bytes).context("Invalid public key")?;
        Ok(SodaPublic { inner })
    }
}

impl Display for SodaPublic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sodapub{}", sodiumoxide::hex::encode(self.inner))
    }
}

impl SodaPublic {
    /// Encrypts to base64 encoded data
    pub fn encrypt(&self, plain: &[u8]) -> String {
        let encrypted = sealedbox::seal(plain, &self.inner);
        base64::encode(&encrypted)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SodaPrivate {
    inner: SecretKey,
}

impl FromStr for SodaPrivate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("sodapriv")
            .context("Not a soda private key")?;
        let bytes = sodiumoxide::hex::decode(s)
            .ok()
            .context("Not hex encoded")?;
        let inner = SecretKey::from_slice(&bytes).context("Invalid private key")?;
        Ok(SodaPrivate { inner })
    }
}

impl Display for SodaPrivate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "sodapriv{}", sodiumoxide::hex::encode(&self.inner))
    }
}

impl SodaPrivate {
    /// Decrypt base64-encoded data
    pub fn decrypt(&self, data: &str) -> Result<Vec<u8>> {
        let cipher = base64::decode(data).context("Invalid base64 input")?;
        sealedbox::open(&cipher, &self.inner.public_key(), &self.inner)
            .ok()
            .context("Could not decrypt data")
    }
}

pub fn new_keypair() -> (SodaPublic, SodaPrivate) {
    let (x, y) = box_::gen_keypair();
    (SodaPublic { inner: x }, SodaPrivate { inner: y })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_roundtrip() {
        for _ in 0..100 {
            let (public, private) = new_keypair();
            assert_eq!(public, public.to_string().parse().unwrap());
            assert_eq!(private, private.to_string().parse().unwrap());
        }
    }

    #[test]
    fn encrypt_decrypt() {
        let msg = b"this is my message";
        for _ in 0..100 {
            let (public, private) = new_keypair();
            let encrypted = public.encrypt(msg);
            let decrypted = private.decrypt(&encrypted).unwrap();
            assert_eq!(msg.as_slice(), &decrypted);
        }
    }
}
