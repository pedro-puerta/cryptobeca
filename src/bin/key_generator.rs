use rand::Rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

fn main() {
    let mut rng = rand::thread_rng();
    let mut private_key_bytes: [u8; 32] = [0; 32];
    rng.fill(&mut private_key_bytes);

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes).expect("Invalid private key");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    let private_key_hex = hex::encode(&secret_key[..]);
    let public_key_hex = hex::encode(public_key.serialize_uncompressed());

    println!("Private Key (Hex): {}", private_key_hex);
    println!("Public Key (Hex): {}", public_key_hex);
}
