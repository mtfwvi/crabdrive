use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Keys {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl Keys {
    pub fn new(secret: &String) -> Self {
        let secret_bytes = secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(secret_bytes),
            decoding_key: DecodingKey::from_secret(secret_bytes),
        }
    }
}
