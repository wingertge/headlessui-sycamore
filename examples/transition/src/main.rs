use headlessui_sycamore::components::Transition;
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let show = create_signal(cx, false);

    view! { cx,
        p { (format!("{}", show)) }
        button(on:click = |_| show.set(true), class = "p-1 m-1 border-gray-400 border") { "Open" }
        button(on:click = |_| show.set(false), class = "p-1 m-1 border-gray-400 border") { "Close" }
        Transition(
            show = show,
            enter = "transition-opacity duration-1000",
            enter_from = "opacity-0",
            enter_to = "opacity-100",
            leave = "transition-opacity duration-1000",
            leave_from = "opacity-100",
            leave_to = "opacity-0",
        ) {
            "Hello World"
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx, App {} })
}
