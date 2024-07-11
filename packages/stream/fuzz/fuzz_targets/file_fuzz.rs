#![no_main]

extern crate stream;

use libfuzzer_sys::fuzz_target;
use stream::file::FileStream;
use std::io::Write;
use std::io::Read;
use crate::stream::SeekStream;
use tempfile::{tempdir, tempfile};

fuzz_target!(|data: &[u8]| {
    let temp_dir = tempdir().unwrap();
    let mut stream = FileStream::new_write(temp_dir.path().join("fuzz_target")).expect("file open");

    stream.write_all(data).expect("all written");
    stream.seek(0);

    let mut buffer_read = Vec::new(); 
    stream.read_to_end(&mut buffer_read).expect("all read");

    assert_eq!(buffer_read, data);
});
