use headlessui_sycamore::components::{
    Combobox, ComboboxButton, ComboboxInput, ComboboxLabel, ComboboxOption, ComboboxOptions,
};
use sycamore::prelude::*;
use test_utils::{
    assert_text_content, query_component, query_component_in, send_key, test_container,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));
        let query = create_signal(cx, String::new());

        let all_options = vec!["a", "b", "c"];
        let options = create_selector(cx, move || query.get()).map(cx, move |query| {
            let query = query.to_lowercase();
            all_options
                .iter()
                .filter(|opt| opt.to_lowercase().contains(&query))
                .map(|v| *v)
                .collect()
        });

        let view = view! { cx,
            Combobox(value = value) {
                ComboboxLabel { (value.get().unwrap()) }
                ComboboxInput(bind:value = query)
                ComboboxButton {
                    "Open"
                }
                ComboboxOptions::<&str, _> {
                    Keyed(iterable = options, view = |cx, option| view! { cx,
                        ComboboxOption(value = option) { (option) }
                    }, key = |option| option.to_string())
                }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let container = query_component("combobox");
        assert_eq!(container.tag_name(), "DIV");

        let label = query_component_in(&container, "combobox-label");
        assert_eq!(label.tag_name(), "LABEL");

        let input = query_component_in(&container, "combobox-input");
        assert_eq!(input.tag_name(), "INPUT");

        let button = query_component_in(&container, "combobox-button");
        assert_eq!(button.tag_name(), "BUTTON");

        assert_eq!(container.children().length(), 3);
        assert_eq!(
            container.get_attribute("aria-labelledby").unwrap(),
            label.get_attribute("id").unwrap()
        );
        assert_eq!(input.get_attribute("aria-haspopup").unwrap(), "listbox");
        assert_eq!(button.get_attribute("aria-haspopup").unwrap(), "listbox");
        assert_eq!(button.get_attribute("aria-expanded"), None);

        button.unchecked_ref::<HtmlElement>().click();

        assert_eq!(button.get_attribute("aria-expanded").unwrap(), "");

        let options = query_component_in(&container, "combobox-options");
        assert_eq!(options.tag_name(), "UL");
        assert_eq!(
            button.get_attribute("aria-controls").unwrap(),
            options.get_attribute("id").unwrap()
        );
        assert_eq!(options.get_attribute("role").unwrap(), "listbox");
        assert_eq!(
            options.get_attribute("aria-labelledby").unwrap(),
            button.get_attribute("id").unwrap()
        );
        assert_eq!(
            options.get_attribute("aria-orientation").unwrap(),
            "vertical"
        );

        let option_items = options.children();
        assert_eq!(option_items.length(), 3);

        assert_text_content!(option_items.item(0).unwrap(), "a");
        assert_text_content!(option_items.item(1).unwrap(), "b");
        assert_text_content!(option_items.item(2).unwrap(), "c");

        for option in (0..3).map(|i| option_items.item(i).unwrap()) {
            assert_eq!(option.get_attribute("role").unwrap(), "option");
            assert_eq!(option.get_attribute("tabindex").unwrap(), "-1");
        }
    });
}

#[wasm_bindgen_test]
pub fn searching_works() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));
        let query = create_signal(cx, String::new());

        let all_options = vec!["a", "Hello", "c"];
        let options = create_selector(cx, move || query.get()).map(cx, move |query| {
            let query = query.to_lowercase();
            all_options
                .iter()
                .filter(|opt| opt.to_lowercase().contains(&query))
                .map(|v| *v)
                .collect()
        });

        let view = view! { cx,
            Combobox(value = value) {
                ComboboxLabel { (value.get().unwrap()) }
                ComboboxInput(bind:value = query)
                ComboboxButton {
                    "Open"
                }
                ComboboxOptions::<&str, _> {
                    Keyed(iterable = options, view = |cx, option| view! { cx,
                        ComboboxOption(value = option) { (option) }
                    }, key = |option| option.to_string())
                }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let input = query_component("combobox-input");

        input.unchecked_ref::<HtmlElement>().click();

        send_key(&input, "he");

        // TODO: Make this work somehow, it works manually and with fantoccini but not in wasm-pack

        /*         let options = query_component("combobox-options");

        assert_eq!(options.children().length(), 1);
        assert_text_content!(options.children().item(0).unwrap(), "Hello"); */
    });
}
