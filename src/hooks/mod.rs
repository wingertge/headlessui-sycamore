use std::sync::atomic::{AtomicU32, Ordering};

static ID: AtomicU32 = AtomicU32::new(0);

pub fn create_id() -> String {
    format!("headlessui-sycamore-{}", generate_id())
}

pub fn generate_id() -> u32 {
    ID.fetch_add(1, Ordering::Relaxed)
}
