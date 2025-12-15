use crabdrive_common::node::NodeId;
use crabdrive_common::version::VersionId;
use crate::user::persistence::model::encryption_key::IV;

pub struct VersionEntity {
    id: VersionId,

    /// The node_id this version belongs to. It should reference a Node with the type File
    file_id: NodeId,

    /// The time the version was created as unix time
    upload_started_on: u64,

    /// The time the version was complete on the server (all chunks were present) as unix time
    upload_ended_on: u64,

    /// The random bytes used as IV prefix to last encrypt the file. The iv for each chunk must be
    /// derived from this value + 4 Bytes describing the index to avoid reordering. This value
    /// MUST NOT be reused for encrypting a new file
    iv: IV,

    /// The amount of chunks the server claimed the file contains. Before commiting a version it 
    /// should be checked all chunks are present in the object storage
    chunk_count_claimed: u64,
}
