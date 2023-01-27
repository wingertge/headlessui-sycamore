use headlessui_sycamore::components::{
    Listbox, ListboxButton, ListboxLabel, ListboxOption, ListboxOptions,
};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, Some("Hello"));

    view! { cx,
        p(id = "listbox-value") { (value.get()) }
        Listbox(value = value) {
            ListboxLabel { "Listbox" }
            ListboxButton { "Open" }
            ListboxOptions::<&str, _> {
                ListboxOption(value = "Hello") { "Hello" }
                ListboxOption(value = "World") { "World" }
                ListboxOption(value = "Test") { "Test" }
            }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
