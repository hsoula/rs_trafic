use std::sync::atomic::{AtomicU64, Ordering};

// Générateur d'IDs global et thread-safe
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub type ObjectId = u64;

pub fn generate_id() -> ObjectId {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}
