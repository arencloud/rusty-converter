use crate::qcow::headers::QcowHeader;

#[derive(BinRead, Debug)]
#[br(big)]
pub struct Qcow2 {
    pub header: QcowHeader
}