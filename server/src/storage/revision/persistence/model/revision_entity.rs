use crate::user::persistence::model::encryption_key::IV;
use chrono::NaiveDateTime;
use crabdrive_common::storage::NodeId;
use crabdrive_common::storage::RevisionId;

pub struct RevisionEntity {
    id: RevisionId,

    /// The `NodeId` this revision belongs to. It should reference a Node with the type File
    file_id: NodeId,

    /// The time the revision was created
    upload_started_on: NaiveDateTime,

    /// The time the revision was complete on the server (all chunks were present)
    upload_ended_on: Option<NaiveDateTime>,

    /// The random bytes used as IV prefix to last encrypt the file. The iv for each chunk must be
    /// derived from this value + 4 Bytes describing the index to avoid reordering. This value
    /// MUST NOT be reused for encrypting a new file
    iv: IV,
}
