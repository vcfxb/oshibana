use std::sync::atomic::{AtomicBool, AtomicUsize};

#[derive(Default)]
pub struct ScryfallPullStatus {
    pub in_progress: AtomicBool,
    pub total_bytes: AtomicUsize,
    pub read_bytes: AtomicUsize,
    pub card_records_read: AtomicUsize,
}
