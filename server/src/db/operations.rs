//createFile (parentId, parent_metadata, node_metadata, chunk_count) -> Ok(nodeId)/Error // creates node in temporary db withUploadStarted timestamp
//uploadChunk (nodeId) -> Ok/Error
//finishUpload (nodeId) -> Ok/Error
//replaceFile (nodeId, node_metadata) -> Ok/Error // copies file node in temporary db withUploadStarted timestamp
//uploadChunk (nodeId) -> Ok/Error
//finishUpload (nodeId) -> Ok/Error // replaces the old node with the node in the temporary db (this makes it impossible to end in an invalid state)

