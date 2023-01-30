use crate::qcow::headers::QcowHeader;

impl QcowHeader {
    pub fn cluster_size(&self) -> u64 {
        1 << self.cluster_bits
    }
}