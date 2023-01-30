use std::io::{Read, Seek};
use binread::{BinRead, BinResult, derive_binread, ReadOptions};
use crate::qcow::header_extension::extension::HeaderExtension;
use crate::qcow::header_extension::feature_name::FeatureKind::{Incompatible, Compatible, AutoClear};

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq)]
#[br(repr(u8))]
pub enum CompressionType {
    Zlib = 0,
    Zstd = 1,
}

#[derive(BinRead, Debug, Clone, Copy, PartialEq, Eq)]
#[br(repr(u32))]
pub enum EncryptionMethod {
    None = 0,
    Aes = 1,
    Luks = 2,
}

#[derive_binread]
#[derive(Debug)]
pub struct Version3Header {
    pub incompatible_features: Incompatible,
    pub compatible_features: Compatible,
    pub autoclear_features: AutoClear,
    pub refcount_order: u32,
    #[br(temp)]
    header_length: u32,
    #[br(if(header_length > 104 && incompatible_features.has_compression_type()))]
    pub compression_type: Compressiontype,
}

#[derive_binread]
#[derive(Debug)]
#[br(magic = b"QFI\xfb")]
pub struct QcowHeader {
    #[br(assert(version == 2 || version == 3))]
    pub version: u32,
    #[br(temp)]
    backing_file_offset: u64,
    #[br(temp)]
    backing_file_size: u32,
    #[br(restore_position, temp, parse_with = read_string, count = backing_file_size, args(backing_file_offset))]
    backing_file_offset: FileString,
    #[br(calc = backing_file_offset.0.clone())]
    pub backing_file: Option<String>,
    pub cluster_bits: u32,
    pub size: u64,
    pub crypt_method: EncryptionMethod,
    pub l1_size: u32,
    pub l1_table_offset: u64,
    pub refcount_table_offset: u64,
    pub refcount_table_clusters: u32,
    pub(crate) number_snapshots: u32,
    pub(crate) snapshots_offset: u64,
    #[br(align_after = 8, if(version == 3))]
    pub v3_header: Option<Version3Header>,
    #[br(parse_with = until_exclusive(|ext: &HeaderExtension| ext.is_end()))]
    pub extensions: Vec<HeaderExtension>,
}

impl Default for CompressionType {
    fn default() -> Self {
        Self::Zlib
    }
}

#[derive(BinRead)]
#[br(import(_offset: u64,))]
pub(crate) struct FileString(#[br(ignore)] pub(crate) Option<String>);

pub(crate) fn read_string<R>(mut reader: &mut R, ro: &ReadOptions, (offset,): (u64,)) -> BinResult<FileString>
    where R: Read + Seek
{
    if offset == 0 {
        Ok(FileString(None))
    } else {
        reader.seek(binread::io::SeekFrom::Start(offset))?;
        let data: Vec<u8> = BinRead::read_options(&mut reader, ro, ())?;
        Ok(FileString(Some(String::from_utf8_lossy(&data).into_owned())))
    }

}