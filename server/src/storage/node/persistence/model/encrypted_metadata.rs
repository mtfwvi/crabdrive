use crate::user::persistence::model::encryption_key::IV;

pub(crate) struct EncryptedMetadata {
    data: Vec<u8>,
    iv: IV,
}
