use headlessui_sycamore::components::{
    Combobox, ComboboxButton, ComboboxInput, ComboboxLabel, ComboboxOption, ComboboxOptions,
};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, Some(String::new()));

    view! { cx,
        Combobox(value = value) {
            ComboboxLabel { "Option" }
            ComboboxInput
            ComboboxButton {
                "Open"
            }
            ComboboxOptions::<String, G> {
                ComboboxOption(value = "Hello".to_string()) { "Hello" }
                ComboboxOption(value = "World".to_string()) { "World" }
                ComboboxOption(value = "Test".to_string()) { "Test" }
            }
        }
    }
}

pub fn main() {
    sycamore::render(|cx| view! { cx, App {} })
}
