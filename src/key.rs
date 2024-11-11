use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{Keypair, RandomizedSigner, SignatureEncoding, Verifier};
use rsa::sha2::{Digest, Sha256};
use hex_literal::hex;

#[derive(Clone, Debug)]
pub struct Key {
    pk: RsaPublicKey,
    signing_key: SigningKey<Sha256>
}

impl Key {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let signing_key = SigningKey::<Sha256>::new(priv_key);
        Key{pk: pub_key, signing_key: signing_key }
    }

    /// PKCS#1 v1.5 https://docs.rs/rsa/latest/rsa/#pkcs1-v15-signatures
    fn sign(&self, data: &[u8]) -> Signature {
        let mut rng = rand::thread_rng();
        self.signing_key.sign_with_rng(&mut rng, data)
    }

    fn hash(data: &[u8]) -> u64 { 
        let mut hasher = Sha256::new();
        hasher.update(data);
        u64::from_be_bytes(hasher.finalize().as_slice().try_into().unwrap())
    }
}