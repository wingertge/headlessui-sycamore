use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        "Hello World"
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
