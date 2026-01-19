use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, ArrayBuffer, Uint8Array};
use web_sys::{Blob, File};

pub struct ChunkInfo {
    pub chunk: ArrayBuffer,
    pub block: u32,
    pub first_block: bool,
    pub last_block: bool,
}

async fn load_file_by_chunk<F, Fut>(file: File, handle_chunk: F) -> Result<(), JsValue>
where
    F: Fn(ChunkInfo) -> Fut,
    Fut: Future<Output = Result<(), JsValue>>,
{
    const CHUNK_SIZE: f64 = 1024.0 * 1024.0 * 16.0;

    let file_size = file.size();
    let mut offset = 0.0;
    let mut block = 0;


    // go through the file in 16mb chunks
    loop {
        let blob = file.slice_with_f64_and_f64(offset, offset + CHUNK_SIZE)?;

        let buffer = JsFuture::from(blob.array_buffer()).await?;
        let buffer = buffer.dyn_into::<ArrayBuffer>()?;

        let buffer_size = buffer.byte_length();

        offset += buffer_size as f64;
        block += 1;

        let first_block = block == 1;
        let last_block = offset >= file_size;

        let chunk_info = ChunkInfo {
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

fn combine_chunks(buffers: Vec<Uint8Array>) -> Blob {
    let buffers_js = Array::new();

    buffers.iter().for_each(|buffer| {
        buffers_js.push(buffer);
    });

    // this failing does not seem recoverable and should not be possible with correctly typed objects
    Blob::new_with_u8_array_sequence(&buffers_js).unwrap()
}

mod test {
    use crate::utils::file::combine_chunks;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::Uint8Array;

    #[wasm_bindgen_test]
    async fn test_combine_chunks() {
        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];

        let part1 = Uint8Array::new_from_slice(&vec1);
        let part2 = Uint8Array::new_from_slice(&vec2);

        let combined = combine_chunks(vec![part1, part2]);
        let combined = JsFuture::from(combined.array_buffer()).await.unwrap();
        let combined = Uint8Array::from(combined);
        let combined_vec = combined.to_vec();

        vec1.append(&mut vec2);

        assert!(combined_vec.eq(&vec1));
    }
}