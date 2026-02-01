#[cfg(test)]
mod tests {
    use crate::storage::vfs::FileChunk;
    use crate::storage::vfs::FileRepository;
    use crate::storage::vfs::backend::Sfs;

    use crabdrive_common::da;
    use crabdrive_common::data::DataAmount;
    use crabdrive_common::uuid::UUID;

    use rand::{Rng, rng};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_sfs_write_read_cycle() {
        // This test writes all files into a temporary directory, which are then deleted directly after.
        let mut sfs = Sfs::new(&":temp:".to_string());

        // Test with 16 files, containing 16 chunks à 100KB of garbage data each.
        // For testing, 100KB should be enough..
        const NUM_FILES: u32 = 16;
        const NUM_CHUNKS: i64 = 16;
        const SIZE_CHUNK: DataAmount = da!(100 KB);

        for _ in 0..NUM_FILES {
            let file_key = UUID::random().to_string();

            let session_id = sfs
                .start_transfer(file_key.clone())
                .expect("Failed to start transfer");

            let mut original_data = Vec::new();

            for i in 0..NUM_CHUNKS {
                // Fill chunks with random data
                let mut rng = rng();
                let mut chunk_data = vec![0u8; SIZE_CHUNK.as_bytes() as usize];
                rng.fill(&mut chunk_data[..]);

                original_data.push(chunk_data.clone());

                let chunk = FileChunk {
                    index: i,
                    data: bytes::Bytes::from(chunk_data),
                };

                assert!(!sfs.chunk_exists(&session_id, i));

                // Write chunk in file system
                sfs.write_chunk(&session_id, chunk)
                    .expect("Failed to write chunk");

                assert!(sfs.chunk_exists(&session_id, i));
            }

            sfs.end_transfer(&session_id)
                .expect("Failed to end transfer");

            for i in 0..NUM_CHUNKS {
                let chunk = sfs
                    .get_chunk(file_key.clone(), i, SIZE_CHUNK)
                    .expect("Failed to read chunk back");

                assert_eq!(
                    chunk.data.as_ref(),
                    &original_data[i as usize],
                    "File {} - Chunk #{} data mismatch",
                    file_key,
                    i
                );
            }
        }
    }
}
