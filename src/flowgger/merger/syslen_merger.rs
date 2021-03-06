use flowgger::config::Config;
use std::ptr;
use super::Merger;

#[derive(Clone)]
pub struct SyslenMerger;

impl SyslenMerger {
    pub fn new(_config: &Config) -> SyslenMerger {
        SyslenMerger
    }
}

impl Merger for SyslenMerger {
    fn frame(&self, bytes: &mut Vec<u8>) {
        let bytes_len = bytes.len();
        let prefix = &format!("{} ", bytes_len + 1);
        let prefix_len = prefix.len();
        bytes.reserve(prefix_len + 1);
        unsafe {
            let bytes_ptr = bytes.as_mut_ptr();
            ptr::copy(bytes_ptr, bytes_ptr.offset(prefix_len as isize), bytes_len);
            let prefix_ptr = prefix.as_ptr();
            ptr::copy(prefix_ptr, bytes_ptr, prefix_len);
            ptr::write(bytes_ptr.offset((prefix_len + bytes_len) as isize), 0x0a);
            bytes.set_len(prefix_len + bytes_len + 1);
        }
    }
}
