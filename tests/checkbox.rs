use headlessui_sycamore::components::{
    Checkbox, CheckboxDescription, CheckboxIndicator, CheckboxLabel,
};
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
        let checked = create_signal(cx, false);

        let node = view! { cx,
            Checkbox(checked = checked) {
                CheckboxIndicator { (if *checked.get() { "Checked" } else { "Not Checked" }) }
                CheckboxLabel { "Checkbox" }
                CheckboxDescription { "It's a checkbox" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("checkbox");
        let indicator = query_component("checkbox-indicator");
        let label = query_component("checkbox-label");
        let description = query_component("checkbox-description");

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(indicator.tag_name(), "BUTTON");
        assert_eq!(label.tag_name(), "LABEL");
        assert_eq!(description.tag_name(), "P");

        assert_eq!(container.children().length(), 3);

        assert_eq!(indicator.get_attribute("role").unwrap(), "checkbox");
        assert_eq!(indicator.get_attribute("aria-checked"), None);
        assert_eq!(indicator.get_attribute("checked"), None);
        assert_eq!(
            indicator.get_attribute("aria-labelledby").unwrap(),
            label.get_attribute("id").unwrap()
        );
        assert_eq!(
            indicator.get_attribute("aria-describedby").unwrap(),
            description.get_attribute("id").unwrap()
        );
        assert_text_content!(indicator, "Not Checked");

        assert_eq!(
            label.get_attribute("for").unwrap(),
            indicator.get_attribute("id").unwrap()
        );
        assert_text_content!(label, "Checkbox");

        assert_text_content!(description, "It's a checkbox");

        checked.set(true);

        assert_eq!(indicator.get_attribute("aria-checked").unwrap(), "");
        assert_eq!(indicator.get_attribute("checked").unwrap(), "");
        assert_text_content!(indicator, "Checked");

        checked.set(false);

        // aria-checked is removed
        assert_eq!(indicator.get_attribute("aria-checked"), None);
        assert_eq!(indicator.get_attribute("checked"), None);
    });
}

#[wasm_bindgen_test]
pub fn clicking_works() {
    create_scope_immediate(|cx| {
        let checked = create_signal(cx, false);

        let node = view! { cx,
            Checkbox(checked = checked) {
                CheckboxIndicator { (if *checked.get() { "Checked" } else { "Not Checked" }) }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let indicator = query_component("checkbox-indicator");

        assert_eq!(indicator.get_attribute("checked"), None);

        indicator.unchecked_ref::<HtmlElement>().click();

        assert_eq!(indicator.get_attribute("checked").unwrap(), "");
        assert_eq!(*checked.get(), true);

        indicator.unchecked_ref::<HtmlElement>().click();
        assert_eq!(indicator.get_attribute("checked"), None);
        assert_eq!(*checked.get(), false);
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works() {
    create_scope_immediate(|cx| {
        let checked = create_signal(cx, false);

        let node = view! { cx,
            Checkbox(checked = checked) {
                CheckboxIndicator { (if *checked.get() { "Checked" } else { "Not Checked" }) }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let indicator = query_component("checkbox-indicator");

        indicator.unchecked_ref::<HtmlElement>().focus().unwrap();

        assert_eq!(indicator.get_attribute("checked"), None);

        send_key(&indicator, " ");

        assert_eq!(*checked.get(), true);
        assert_eq!(indicator.get_attribute("checked").unwrap(), "");

        send_key(&indicator, " ");

        assert_eq!(indicator.get_attribute("checked"), None);
        assert_eq!(*checked.get(), false);

        send_key(&indicator, "Enter");

        assert_eq!(indicator.get_attribute("checked"), None);
        assert_eq!(*checked.get(), false);
    });
}
