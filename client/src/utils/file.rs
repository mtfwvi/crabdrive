use tracing::error;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::File;
use web_sys::js_sys::{ArrayBuffer};

pub struct Chunk{
    pub chunk: ArrayBuffer,
    pub block: u32,
    pub first_block: bool,
    pub last_block: bool,
}

pub async fn parse_file<F, Fut>(file: File, handle_chunk: F) -> Result<(), JsValue>
where
    F:Fn(Chunk) -> Fut,
    Fut: Future<Output = Result<(), JsValue>>,
 {
    const CHUNK_SIZE: f64 = 1024.0 * 1024.0 * 16.0;

    let file_size = file.size();
    let mut offset = 0.0;
    let mut block = 0;

    loop {
        let blob = file
            .slice_with_f64_and_f64(offset, offset + CHUNK_SIZE)?;

        let buffer = JsFuture::from(blob.array_buffer()).await?;
        let buffer = buffer.dyn_into::<ArrayBuffer>()?;

        let buffer_size = buffer.byte_length();

        offset += buffer_size as f64;
        block += 1;

        let first_block = block == 1;
        let last_block = offset >= file_size;

        error!(offset);

        // TODO
        // chunk handling

        let chunk_info = Chunk {
            chunk: buffer,
            block,
            first_block,
            last_block,
        };

        handle_chunk(chunk_info).await?;

        if last_block || buffer_size == 0 {
            break;
        }
    }

    Ok(())
}
