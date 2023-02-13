use headlessui_sycamore::components::{
    Checkbox, CheckboxDescription, CheckboxIndicator, CheckboxLabel,
};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let checked = create_signal(cx, false);

    view! { cx,
        Checkbox(checked = checked) {
            CheckboxIndicator { (if *checked.get() { "✓" } else { "✗" }) }
            CheckboxLabel { "Checkbox" }
            CheckboxDescription { "It's a checkbox" }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
