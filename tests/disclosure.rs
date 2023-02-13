use headlessui_sycamore::components::{Disclosure, DisclosureButton, DisclosurePanel};
use sycamore::prelude::*;
use sycamore::reactive::create_scope_immediate;
use test_utils::{assert_text_content, query_component, send_key, test_container};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let open = create_signal(cx, false);

        let node = view! { cx,
            Disclosure(open = open) {
                DisclosureButton { "Disclosure 1" }
                DisclosurePanel {
                    "Disclosure 1 Content"
                }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("disclosure");
        let button = query_component("disclosure-button");

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(button.tag_name(), "BUTTON");

        assert_text_content!(button, "Disclosure 1");
        assert_eq!(button.get_attribute("aria-expanded"), None);

        assert_eq!(container.children().length(), 1);

        open.set(true);

        assert_eq!(container.children().length(), 2);

        let panel = query_component("disclosure-panel");

        assert_eq!(panel.tag_name(), "DIV");
        assert_eq!(button.get_attribute("aria-expanded").unwrap(), "");
        assert_eq!(
            panel.get_attribute("id").unwrap(),
            button.get_attribute("aria-controls").unwrap()
        );
        assert_text_content!(panel, "Disclosure 1 Content");

        open.set(false);

        assert_eq!(container.children().length(), 1);
    });
}

#[wasm_bindgen_test]
pub fn clicking_works() {
    create_scope_immediate(|cx| {
        let open = create_signal(cx, false);

        let node = view! { cx,
            Disclosure(open = open) {
                DisclosureButton { "Disclosure 1" }
                DisclosurePanel {
                    "Disclosure 1 Content"
                }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("disclosure");
        let button = query_component("disclosure-button");

        assert_eq!(container.children().length(), 1);
        button.unchecked_ref::<HtmlElement>().click();
        assert_eq!(container.children().length(), 2);
        button.unchecked_ref::<HtmlElement>().click();
        assert_eq!(container.children().length(), 1);
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works() {
    create_scope_immediate(|cx| {
        let open = create_signal(cx, false);

        let node = view! { cx,
            Disclosure(open = open) {
                DisclosureButton { "Disclosure 1" }
                DisclosurePanel {
                    "Disclosure 1 Content"
                }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("disclosure");
        let button = query_component("disclosure-button");

        assert_eq!(container.children().length(), 1);

        send_key(&button, "Enter");
        assert_eq!(container.children().length(), 2);
        send_key(&button, " ");
        assert_eq!(container.children().length(), 1);
    });
}
