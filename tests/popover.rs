use headlessui_sycamore::components::{Popover, PopoverOverlay, PopoverPanel};
use sycamore::prelude::*;
use sycamore::reactive::create_scope_immediate;
use test_utils::{assert_text_content, query_component, test_container};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let open = create_signal(cx, false);

        let node = view! { cx,
            Popover(open = open) {
                PopoverOverlay
                PopoverPanel { "Panel" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("popover");
        let overlay = query_component("popover-overlay");

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(container.children().length(), 1);

        assert_eq!(overlay.tag_name(), "DIV");

        open.set(true);

        assert_eq!(container.children().length(), 2);

        let panel = query_component("popover-panel");

        assert_eq!(panel.tag_name(), "DIV");
        assert_text_content!(panel, "Panel");
    });
}
