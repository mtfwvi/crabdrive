use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn post_chunk(
    Path((_node_id, _revision_id, _chunk_index)): Path<(Uuid, Uuid, u64)>,
    _chunk: axum::body::Bytes,
) -> (StatusCode, Json<()>) {
    //TODO implement
    (StatusCode::CREATED, Json(()))
}

pub async fn get_chunk(
    Path((_node_id, _revision_id, _chunk_index)): Path<(Uuid, Uuid, u64)>,
) -> (StatusCode, Vec<u8>) {
    //TODO implement
    (StatusCode::OK, vec![0])
}
