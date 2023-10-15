use std::error::Error;
use std::ffi::CString;
use std::path::PathBuf;

use super::decompressed_replay::DecompressedReplay;
use super::replay_header::Header;
use crate::huffman;

#[link(name = "huffman")]
extern "C" {
    fn decompress(src: *const std::ffi::c_uchar) -> *mut u8;
}

unsafe fn decompress_replay_file(input_path: &PathBuf, output_path: &PathBuf) {
    // let data: [u8; 10] = [130, 209, 30, 255, 237, 209, 255, 237, 87, 63];
    // let c_data = CString::new(data).unwrap();
    // unsafe {
    //     decompress(c_data.as_ptr());
    // };
    huffman::build();
}

#[derive(Debug, Clone)]
pub struct Replay {
    path: PathBuf,
}

impl Replay {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// # Safety
    ///
    /// This function is unsafe because it calls a C function.
    pub unsafe fn decompress(&self, output: PathBuf) -> Result<DecompressedReplay, Box<dyn Error>> {
        unsafe {
            decompress_replay_file(&self.path, &output)
        }
        Ok(DecompressedReplay::new(output))
    }
}

impl Header for Replay {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
