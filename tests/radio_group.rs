use headlessui_sycamore::components::{
    RadioGroup, RadioGroupDescription, RadioGroupLabel, RadioGroupOption,
};
use sycamore::prelude::*;
use sycamore::reactive::create_scope_immediate;
use test_utils::{
    assert_text_content, document, query_component, query_component_in, send_key, test_container,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));

        let node = view! { cx,
            RadioGroup(value = value) {
                RadioGroupLabel { "Radio Group" }
                RadioGroupDescription { "This is a radio group" }
                RadioGroupOption(value = "a") {
                    RadioGroupLabel { "a" }
                    RadioGroupDescription { "This is option a" }
                }
                RadioGroupOption(value = "b") { "b" }
                RadioGroupOption(value = "c") { "c" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("radio-group");
        let label = query_component("radio-group-label");
        let description = query_component("radio-group-description");

        let children = container.children();

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(children.length(), 5);
        assert_eq!(container.get_attribute("role").unwrap(), "radiogroup");
        assert_eq!(
            container.get_attribute("aria-labelledby").unwrap(),
            label.get_attribute("id").unwrap()
        );
        assert_eq!(
            container.get_attribute("aria-describedby").unwrap(),
            description.get_attribute("id").unwrap()
        );

        assert_eq!(label.tag_name(), "LABEL");
        assert_text_content!(label, "Radio Group");

        assert_eq!(description.tag_name(), "DIV");
        assert_text_content!(description, "This is a radio group");

        let option1 = children.item(2).unwrap();
        assert_eq!(option1.tag_name(), "DIV");
        assert_eq!(option1.get_attribute("role").unwrap(), "radio");
        assert_eq!(option1.get_attribute("tabindex").unwrap(), "0");

        let option1_label = query_component_in(&option1, "radio-group-label");
        let option1_description = query_component_in(&option1, "radio-group-description");

        assert_eq!(
            option1.get_attribute("aria-labelledby").unwrap(),
            option1_label.get_attribute("id").unwrap()
        );
        assert_eq!(
            option1.get_attribute("aria-describedby").unwrap(),
            option1_description.get_attribute("id").unwrap()
        );
        assert_eq!(option1.get_attribute("aria-checked").unwrap(), "");

        for i in 3..5 {
            let option = children.item(i).unwrap();
            assert_eq!(option.get_attribute("tabindex").unwrap(), "-1");
            assert_eq!(option.get_attribute("aria-checked"), None);
        }

        value.set(Some("b"));

        assert_eq!(option1.get_attribute("aria-checked"), None);
        assert_eq!(option1.get_attribute("tabindex").unwrap(), "-1");
        assert_eq!(
            children
                .item(3)
                .unwrap()
                .get_attribute("aria-checked")
                .unwrap(),
            ""
        );
        assert_eq!(
            children.item(3).unwrap().get_attribute("tabindex").unwrap(),
            "0"
        );
    });
}

#[wasm_bindgen_test]
pub fn clicking_works() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));

        let node = view! { cx,
            RadioGroup(value = value) {
                RadioGroupOption(value = "a") { "a" }
                RadioGroupOption(value = "b") { "b" }
                RadioGroupOption(value = "c") { "c" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("radio-group");
        let children = container.children();

        let child1 = children.item(0).unwrap().unchecked_into::<HtmlElement>();
        let child2 = children.item(1).unwrap().unchecked_into::<HtmlElement>();
        let child3 = children.item(2).unwrap().unchecked_into::<HtmlElement>();

        child2.click();
        assert_eq!(value.get().as_ref().unwrap(), "b");
        child3.click();
        assert_eq!(value.get().as_ref().unwrap(), "c");
        child1.click();
        assert_eq!(value.get().as_ref().unwrap(), "a");
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works() {
    create_scope_immediate(|cx| {
        let value = create_signal(cx, Some("a"));

        let node = view! { cx,
            RadioGroup(value = value) {
                RadioGroupOption(value = "a") { "a" }
                RadioGroupOption(value = "b") { "b" }
                RadioGroupOption(value = "c") { "c" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("radio-group");
        let children = container.children();

        let child1 = children.item(0).unwrap();
        let child2 = children.item(1).unwrap();
        let child3 = children.item(2).unwrap();

        send_key(&child1, "ArrowDown");
        assert_eq!(document().active_element().unwrap(), child2);
        assert_eq!(value.get_untracked().as_ref().unwrap(), "b");

        send_key(&child2, "ArrowRight");
        assert_eq!(document().active_element().unwrap(), child3);
        assert_eq!(value.get_untracked().as_ref().unwrap(), "c");

        send_key(&child3, "ArrowUp");
        assert_eq!(document().active_element().unwrap(), child2);
        assert_eq!(value.get_untracked().as_ref().unwrap(), "b");

        send_key(&child2, "ArrowLeft");
        assert_eq!(document().active_element().unwrap(), child1);
        assert_eq!(value.get_untracked().as_ref().unwrap(), "a");
    });
}
