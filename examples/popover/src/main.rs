use headlessui_sycamore::components::{Popover, PopoverOverlay, PopoverPanel};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let open = create_signal(cx, false);

    view! { cx,
        button(on:click = |_| open.set(true)) { "Open" }
        Popover(open = open) {
            PopoverOverlay
            PopoverPanel { "Panel" }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
