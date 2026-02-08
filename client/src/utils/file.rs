use crate::constants::CHUNK_SIZE;
use crate::model::chunk::DecryptedChunk;
use crate::utils::error::{future_from_js_promise, wrap_js_err};
use anyhow::{Context, Result};
use web_sys::js_sys::{Array, ArrayBuffer, Uint8Array};
use web_sys::{Blob, File};

pub async fn load_file_by_chunk<F, Fut>(file: File, handle_chunk: F) -> Result<()>
where
    F: Fn(&DecryptedChunk) -> Fut,
    Fut: Future<Output = Result<()>>,
{
    let file_size = file.size();
    let mut offset = 0.0;
    let mut block = 0;

    // go through the file in 16mb chunks
    loop {
        let blob = wrap_js_err(file.slice_with_f64_and_f64(offset, offset + CHUNK_SIZE))?;

        let buffer: ArrayBuffer = future_from_js_promise(blob.array_buffer()).await?;

        let buffer_size = buffer.byte_length();

        offset += buffer_size as f64;
        block += 1;

        let first_block = block == 1;
        let last_block = offset >= file_size;

        let chunk_info = DecryptedChunk {
            chunk: buffer,
            index: block,
            first_block,
            last_block,
        };

        handle_chunk(&chunk_info)
            .await
            .context(format!("handle chunk {}", block))?;

        if last_block || buffer_size == 0 {
            break;
        }
    }

    Ok(())
}

pub fn combine_chunks(buffers: Vec<Uint8Array>) -> Result<Blob> {
    let buffers_js = Array::new();

    buffers.iter().for_each(|buffer| {
        buffers_js.push(buffer);
    });

    wrap_js_err(Blob::new_with_u8_array_sequence(&buffers_js))
}

#[cfg(test)]
mod test {
    use crate::utils::file::combine_chunks;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::js_sys::{ArrayBuffer, Uint8Array};

    #[wasm_bindgen_test]
    async fn test_combine_chunks() {
        let mut vec1 = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];

        let part1 = Uint8Array::new_from_slice(&vec1);
        let part2 = Uint8Array::new_from_slice(&vec2);

        let result = combine_chunks(vec![part1, part2]).unwrap();
        let result_buf: ArrayBuffer = JsFuture::from(result.array_buffer())
            .await
            .unwrap()
            .dyn_into()
            .unwrap();

        let result_vec = Uint8Array::new(&result_buf).to_vec();

        vec1.append(&mut vec2);

        assert_eq!(result_vec, vec1);
    }
}
