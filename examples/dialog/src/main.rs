use headlessui_sycamore::components::{Dialog, DialogDescription, DialogPanel, DialogTitle};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let open = create_signal(cx, false);

    view! { cx,
        button(type = "button", on:click = |_| open.set(true)) { "Open" }
        label { (format!("Is Open: {}", open.get())) }
        Dialog(open = open) {
            DialogPanel {
                DialogTitle { "Dialog Title" }
                DialogDescription { "Dialog Description" }
                "Dialog Content"
            }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
