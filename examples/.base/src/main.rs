use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        "Hello World"
    }
}

pub fn main() {
    sycamore::render(|cx| view! { cx, App {} })
}
