use headlessui_sycamore::components::{Menu, MenuItem};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Menu {
            MenuItem { "Menu Item 1" }
            MenuItem { "Menu Item 2" }
            MenuItem { "Menu Item 3" }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx, App {} })
}
