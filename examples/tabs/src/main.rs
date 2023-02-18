use headlessui_sycamore::components::{Tab, TabGroup, TabList, TabPanel};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        TabGroup {
            TabList {
                Tab(index = 0) { "Tab 1" }
                Tab(index = 1) { "Tab 2" }
                Tab(index = 2) { "Tab 3" }
            }
            TabPanel(index = 0) { "Tab 1 Content" }
            TabPanel(index = 1) { "Tab 2 Content" }
            TabPanel(index = 2) { "Tab 3 Content" }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
