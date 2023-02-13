/* use std::sync::atomic::{AtomicU32, Ordering};

static ID: AtomicU32 = AtomicU32::new(0);

pub fn create_id() -> String {
    format!("headlessui-sycamore-{}", generate_id())
}

pub fn generate_id() -> u32 {
    ID.fetch_add(1, Ordering::Relaxed)
}
 */

use sycamore::stable_id::create_unique_id;

pub fn create_id() -> String {
    let id = create_unique_id();

    if let (Some(component_id), item_id) = id {
        format!("headlessui-sycamore-{component_id}-{item_id}")
    } else {
        format!("headlessui-sycamore-{}", id.1)
    }
}
