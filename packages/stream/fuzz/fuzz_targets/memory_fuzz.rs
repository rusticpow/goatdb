#![no_main]

extern crate stream;

use libfuzzer_sys::fuzz_target;
use stream::memory::MemoryStream;
use std::io::Write;
use std::io::Read;
use crate::stream::SeekStream;

fuzz_target!(|data: &[u8]| {
    let mut stream = MemoryStream::new();
    stream.write_all(data).expect("all written");
    stream.seek(0);

    let mut buffer_read = Vec::new(); 
    stream.read_to_end(&mut buffer_read).expect("all read");

    assert_eq!(buffer_read, data);
});
