use binread::derive_binread;

#[derive_binread]
#[derive(Debug)]
#[br(repr(u8))]
pub enum FeatureKind {
    Incompatible = 0,
    Compatible = 1,
    AutoClear = 2,
}

#[derive_binread]
#[derive(Debug)]
pub struct FeatureName {
    pub kind: FeatureKind,
    pub bit_number: u8,
    #[br(temp, count = 0x2e)]
    feature_name_bytes: Vec<u8>,
    #[br(calc = { feature_name_bytes.retain(|&b| b != 0); String::from_utf8_lossy(&feature_name_bytes).into_owned()})]
    pub feature_name: String,
}