use binread::{BinRead, derive_binread};

#[derive(BinRead, Debug)]
#[br(import(size: u32))]
pub struct SnapshotExtraData {
    #[br(if(size >= 8))]
    pub vm_state_size: u64,
    #[br(if(size >= 16))]
    pub virtual_disk_size: Option<u64>,
    #[br(if(size >= 24))]
    pub instruction_count: Option<u64>,
}

#[derive(BinRead, Debug)]
pub struct SnapshotTime {
    pub secs: u32,
    pub nanosecs: u32,
}

#[derive_binread]
#[derive(Debug)]
pub struct Snapshot {
    #[br(temp)]
    l1_table_offset: u64,
    #[br(temp)]
    l1_entry_count: u32,
    #[br(restore_position, seek_before = SeekFrom::Start(l1_table_offset), count = l1_entry_count)]
    pub l1_table: Vec<L1Entry>,
    #[br(temp)]
    unique_id_length: u16,
    #[br(temp)]
    name_length: u16,
    pub time: SnapshotTime,
    pub guest_runtime: u64,
    pub vm_state_size: u32,
    #[br(temp)]
    extra_data_size: u32,
    #[br(pad_size_to = extra_data_size)]
    #[br(args(extra_data_size))]
    pub extra_data: SnapshotExtraData,
    #[br(count = unique_id_length, try_map = String::from_utf8)]
    pub unique_id: String,
    #[br(count = name_length, try_map = String::from_utf8)]
    pub name: String,
}