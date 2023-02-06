use headlessui_sycamore::components::{Disclosure, DisclosureButton, DisclosurePanel};
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let open1 = create_signal(cx, false);
    let open2 = create_signal(cx, true);

    view! { cx,
        Disclosure(open = open1) {
            DisclosureButton { "Disclosure 1" }
            DisclosurePanel {
                "Disclosure 1 Content"
            }
        }
        Disclosure(open = open2) {
            DisclosureButton { "Disclosure 2" }
            DisclosurePanel {
                "Disclosure 2 Content"
            }
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
