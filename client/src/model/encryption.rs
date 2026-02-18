use crabdrive_common::{encryption_key::EncryptionKey, storage::NodeId};

pub type RawEncryptionKey = [u8; 32];

// TODO: For security, it's probably smarter to keep handles to the CryptoKey object,
//       rather than always importing & exporting them.

/// The key derived from the password. Used for creating the [`WrappedKey`].
pub type DerivedKey = RawEncryptionKey;
/// This key wraps the [`MasterKey`] with the derived key, for server storage.
pub type WrappedKey = EncryptionKey;
/// The master key is used to encrypt and decrypt metadata keys.
pub type MasterKey = RawEncryptionKey;
/// The file key is used for encrypting and decrypting each chunk
pub type FileKey = RawEncryptionKey; // IV is stored in Revision
/// The encryption key is used to encrypt and decrypt node metadata.
pub type MetadataKey = RawEncryptionKey; // IV is stored inside metadata

pub type ChildKey = (NodeId, MetadataKey);
