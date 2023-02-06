use headlessui_sycamore::components::{
    Combobox, ComboboxButton, ComboboxInput, ComboboxLabel, ComboboxOption, ComboboxOptions,
};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, Some("Hello"));
    let query = create_signal(cx, String::new());

    let all_options = vec!["Hello", "World", "Test"];
    let options = create_selector(cx, move || query.get()).map(cx, move |query| {
        let query = query.to_lowercase();
        all_options
            .iter()
            .filter(|opt| opt.to_lowercase().contains(&query))
            .map(|v| *v)
            .collect()
    });

    view! { cx,
        Combobox(value = value) {
            ComboboxLabel { (value.get().unwrap()) }
            ComboboxInput(bind:value = query)
            ComboboxButton {
                "Open"
            }
            ComboboxOptions::<&str, G> {
                Keyed(iterable = options, view = |cx, option| view! { cx,
                    ComboboxOption(value = option) { (option) }
                }, key = |option| option.to_string())
            }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
