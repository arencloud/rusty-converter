use crate::qcow::header_extension::feature_name::FeatureName;
use binread::derive_binread;

#[derive_binread]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HeaderExtensionKind {
    #[br(magic = 0_u32)]
    End,
    #[br(magic = 0x6803f857_u32)]
    FeatureNameTable,
    #[br(magic = 0xe2792aca_u32)]
    BackingFileFormat,
    #[br(magic = 0x23852875_u32)]
    BitmapsExtension,
    #[br(magic = 0x0537be77_u32)]
    FullDiskEncryption,
    #[br(magic = 0x44415441_u32)]
    ExternalDataPath,
    Other(u32),
}
#[derive_binread]
#[derive(Debug)]
pub enum HeaderExtension {
    #[br(magic = 0_u32)]
    End,
    #[br(magic = 0x6803f857_u32)]
    NameTable(
        #[br(temp)]
        u32,
        #[br(align_after = 8, count = self_0 / 0x30)]
        Vec<FeatureName>,
    ),
    #[br(magic = 0xe2792aca_u32)]
    BackingFileFormat(
        #[br(temp)]
        u32,
        #[br(temp, count = self_0)]
        Vec<u8>,
        #[br(calc = {self_1.retain(|&b| b != 0); String::from_utf8_lossy(&self_1).into_owned()})]
        String,
    ),
    #[br(magic = 0x44415441_u32)]
    ExternalDataPath(
        #[br(temp)]
        u32,
        #[br(temp, count = self_0)]
        Vec<u8>,
        #[br(calc = {self_1.retain(|&b| b != 0); String::from_utf8_lossy(&self_1).into_owned()})]
        String,
    ),
    Unparsed {
        kind: HeaderExtensionKind,
        #[br(temp)]
        data_length: u32,
        #[br(align_after = 8, count = data_length)]
        data: Vec<u8>,
    },
}

impl HeaderExtension {
    pub(crate) fn is_end(&self) -> bool {
        matches!(self, Self::End)
    }
}