use super::{DataConsumer, DataFrameReader};
use crate::hamming;

impl<'a, ProgressCallback: FnMut(u64, u64), Consumer: DataConsumer>
    DataFrameReader<'a, ProgressCallback, Consumer>
{
    pub fn push_bytes(&mut self, bytes: impl IntoIterator<Item = u8>) -> Result<(), String> {
        self.push_to_pre_hamming_decode_buffer(bytes);
        self.hamming_decode_as_many_chunks_as_possible()?;
        self.read_as_many_frames_as_possible()
    }

    fn push_to_pre_hamming_decode_buffer(&mut self, bytes: impl IntoIterator<Item = u8>) {
        let old_len = self.pre_hamming_decode_buffer.len();
        self.pre_hamming_decode_buffer.extend(bytes);
        let extended_by = self.pre_hamming_decode_buffer.len() - old_len;
        self.track_progress(extended_by);
    }

    fn track_progress(&mut self, extended_by: usize) {
        self.progress
            .advance(extended_by as u64, &mut self.progress_callback);
    }

    fn hamming_decode_as_many_chunks_as_possible(&mut self) -> Result<(), String> {
        while self.pre_hamming_decode_buffer.len() >= 8 {
            let mut chunk: Vec<u8> = self.pre_hamming_decode_buffer.drain(0..8).collect();
            assert_eq!(chunk.len(), 8);
            hamming::decode_bytes(chunk.as_mut_slice())
                .map_err(|_| format!("Error detected during Hamming decode"))?;
            // Remove last byte that was used to store a hamming code.
            chunk.pop().unwrap();
            self.post_hamming_decode_buffer.extend(chunk.into_iter());
        }
        Ok(())
    }
}
