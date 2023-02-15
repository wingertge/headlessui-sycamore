use headlessui_sycamore::components::{
    RadioGroup, RadioGroupDescription, RadioGroupLabel, RadioGroupOption,
};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let value = create_signal(cx, Some("b"));

    view! { cx,
        RadioGroup(value = value) {
            RadioGroupLabel { "Radio Group" }
            RadioGroupDescription { "This is a radio group" }
            RadioGroupOption(value = "a") { "a" }
            RadioGroupOption(value = "b") { "b" }
            RadioGroupOption(value = "c") { "c" }
            h1 { (value.get()) }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx, App {} })
}
