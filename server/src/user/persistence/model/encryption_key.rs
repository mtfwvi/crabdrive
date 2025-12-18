/// Initialization vector for encryption
pub type IV = [u8; 12];

pub(crate) struct EncryptionKey {
    key: Vec<u8>,
    iv: IV,
}
