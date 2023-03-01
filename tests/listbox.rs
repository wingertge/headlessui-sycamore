use headlessui_sycamore::components::{
    Listbox, ListboxButton, ListboxLabel, ListboxOption, ListboxOptions,
};
use sycamore::prelude::*;
use test_utils::{query_component, query_component_in, test_container};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));

        let view = view! { cx,
            Listbox(value = value) {
                ListboxLabel { "Listbox" }
                ListboxButton { "Open" }
                ListboxOptions::<&str, _> {
                    ListboxOption(value = "a") { "a" }
                    ListboxOption(value = "b") { "b" }
                    ListboxOption(value = "c") { "c" }
                }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let container = query_component("listbox");
        assert_eq!(container.tag_name(), "DIV");

        let label = query_component_in(&container, "listbox-label");
        assert_eq!(label.tag_name(), "LABEL");

        let button = query_component_in(&container, "listbox-button");
        assert_eq!(button.tag_name(), "BUTTON");

        assert_eq!(
            container.get_attribute("aria-labelledby").unwrap(),
            label.get_attribute("id").unwrap()
        );

        assert_eq!(button.get_attribute("aria-expanded"), None);
        assert_eq!(button.get_attribute("aria-haspopup").unwrap(), "listbox");

        button.unchecked_ref::<HtmlElement>().click();

        assert_eq!(button.get_attribute("aria-expanded").unwrap(), "");

        let options = query_component_in(&container, "listbox-options");
        assert_eq!(options.tag_name(), "UL");

        assert_eq!(options.get_attribute("role").unwrap(), "listbox");
        assert_eq!(
            button.get_attribute("aria-controls").unwrap(),
            options.get_attribute("id").unwrap()
        );
        assert_eq!(
            options.get_attribute("aria-labelledby").unwrap(),
            button.get_attribute("id").unwrap()
        );
        assert_eq!(
            options.get_attribute("aria-orientation").unwrap(),
            "vertical"
        );

        let items = options.children();

        assert_eq!(
            items
                .item(0)
                .unwrap()
                .get_attribute("aria-selected")
                .unwrap(),
            ""
        );

        for item in (0..3).map(|i| items.item(i).unwrap()) {
            assert_eq!(item.get_attribute("role").unwrap(), "option");
        }
    });
}
