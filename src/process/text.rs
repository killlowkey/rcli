use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::fs;
use std::io::Read;

trait TextSign {
    /// Sign the data from the reader and return signature
    /// " &mut dyn Read" is dynamic dispatch
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    /// "impl Read" is static dispatch
    fn verify(&self, reader: impl Read, sign: &[u8]) -> Result<bool>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let key = fs::read(key)?;
    let key = &key[..32];
    let key = key.try_into()?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };

    // base64 output
    let encoded = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", encoded);
    Ok(())
}

// 添加这两个宏，先把第一课代码提交上去
#[allow(clippy::let_unit_value)]
#[allow(dead_code)]
pub fn process_text_verify(input: &str, _: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    match format {
        TextSignFormat::Blake3 => todo!(),
        TextSignFormat::Ed25519 => todo!(),
    };
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve perf by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::hash(&buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = Signature::from_bytes(sign.try_into()?);
        let ret = self.key.verify(&buf, &sign).is_ok();
        Ok(ret)
    }
}
