use axum::extract::Path;
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn post_chunk(
    Path((_revision_id, _chunk_index)): Path<(Uuid, u64)>,
    _chunk: Vec<u8>,
) -> StatusCode {
    //TODO implement
    StatusCode::CREATED
}

pub async fn get_chunk(
    Path((_revision_id, _chunk_index)): Path<(Uuid, u64)>,
) -> (StatusCode, Vec<u8>) {
    //TODO implement
    (StatusCode::OK, vec![0])
}
