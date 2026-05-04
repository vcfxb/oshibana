
#[derive(Default)]
pub struct ScryfallPullStatus {
    pub in_progress: bool,
    pub total_bytes: usize,
    pub read_bytes: usize,
    pub card_records_read: usize,
}
