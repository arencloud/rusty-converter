use binread::derive_binread;
use crate::qcow::headers::{EncryptionMethod, FileString, read_string};


#[derive_binread]
#[derive(Debug)]
#[br(magic = b"QFI\xfb")]
pub struct QcowV1Header {
    #[br(assert(version == 1))]
    pub version: u32,
    #[br(temp)]
    backing_file_offset: u64,
    #[br(temp)]
    backing_file_size: u32,
    #[br(restore_position, temp, parse_with = read_string, count = backing_file_size, args(backing_file_offset))]
    backing_file_offset: FileString,
    #[br(calc = backing_file_offset.0.clone())]
    pub backing_file: Option<String>,
    pub mtime: u32,
    pub size: u64,
    pub cluster_bits: u8,
    pub l2_bits: u8,
    #[br(temp)]
    padding: [u8; 2],
    pub crypt_method: EncryptionMethod,
    pub l1_table_offset: u64,
}