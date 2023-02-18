use headlessui_sycamore::components::{Tab, TabGroup, TabList, TabPanel};
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
        let selected_index = create_signal(cx, 0);

        let view = view! { cx,
            TabGroup(selected_index = selected_index) {
                TabList {
                    Tab(index = 0) { "Tab 1" }
                    Tab(index = 1) { "Tab 2" }
                    Tab(index = 2) { "Tab 3" }
                }
                TabPanel(index = 0) { "Tab 1 Content" }
                TabPanel(index = 1) { "Tab 2 Content" }
                TabPanel(index = 2) { "Tab 3 Content" }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let container = query_component("tab-group");

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(container.children().length(), 2);

        let tab_list = query_component_in(&container, "tab-list");
        let panel = query_component_in(&container, "tab-panel");

        assert_eq!(tab_list.tag_name(), "DIV");
        assert_eq!(panel.tag_name(), "DIV");

        assert_eq!(tab_list.get_attribute("role").unwrap(), "tablist");
        assert_eq!(
            tab_list.get_attribute("aria-orientation").unwrap(),
            "horizontal"
        );

        let tabs = tab_list.children();

        let selected = tabs.item(0).unwrap();

        assert_eq!(selected.get_attribute("tabindex").unwrap(), "0");
        assert_eq!(
            selected.get_attribute("aria-controls").unwrap(),
            panel.get_attribute("id").unwrap()
        );
        assert_eq!(selected.get_attribute("selected").unwrap(), "");
        assert_eq!(selected.get_attribute("role").unwrap(), "tab");

        for tab in (1..3).map(|i| tabs.item(i).unwrap()) {
            assert_eq!(tab.get_attribute("role").unwrap(), "tab");
            assert_eq!(tab.get_attribute("tabindex").unwrap(), "-1");
            assert_eq!(tab.get_attribute("selected"), None);
        }

        assert_eq!(panel.get_attribute("role").unwrap(), "tabpanel");
        assert_eq!(
            panel.get_attribute("aria-labelledby").unwrap(),
            selected.get_attribute("id").unwrap()
        );
        assert_text_content!(panel, "Tab 1 Content");

        selected_index.set(2);

        let prev_selected = selected;
        let selected = tabs.item(2).unwrap();
        let panel = query_component_in(&container, "tab-panel");

        assert_eq!(prev_selected.get_attribute("tabindex").unwrap(), "-1");
        assert_eq!(prev_selected.get_attribute("selected"), None);

        assert_eq!(selected.get_attribute("tabindex").unwrap(), "0");
        assert_eq!(selected.get_attribute("selected").unwrap(), "");
        assert_eq!(
            selected.get_attribute("aria-controls").unwrap(),
            panel.get_attribute("id").unwrap()
        );

        assert_text_content!(panel, "Tab 3 Content");
    });
}

#[wasm_bindgen_test]
pub fn clicking_works() {
    create_scope_immediate(|cx| {
        let view = view! { cx,
            TabGroup(default_index = 1) {
                TabList {
                    Tab(index = 0) { "Tab 1" }
                    Tab(index = 1) { "Tab 2" }
                    Tab(index = 2) { "Tab 3" }
                }
                TabPanel(index = 0) { "Tab 1 Content" }
                TabPanel(index = 1) { "Tab 2 Content" }
                TabPanel(index = 2) { "Tab 3 Content" }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let tabs = query_component("tab-list").children();

        assert_eq!(tabs.item(1).unwrap().get_attribute("selected").unwrap(), "");
        assert_text_content!(query_component("tab-panel"), "Tab 2 Content");

        tabs.item(2)
            .unwrap()
            .unchecked_into::<HtmlElement>()
            .click();

        assert_eq!(tabs.item(2).unwrap().get_attribute("selected").unwrap(), "");
        assert_text_content!(query_component("tab-panel"), "Tab 3 Content");

        tabs.item(0)
            .unwrap()
            .unchecked_into::<HtmlElement>()
            .click();

        assert_eq!(tabs.item(0).unwrap().get_attribute("selected").unwrap(), "");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works_horizontal() {
    create_scope_immediate(|cx| {
        let view = view! { cx,
            TabGroup(default_index = 1) {
                TabList {
                    Tab(index = 0) { "Tab 1" }
                    Tab(index = 1) { "Tab 2" }
                    Tab(index = 2) { "Tab 3" }
                }
                TabPanel(index = 0) { "Tab 1 Content" }
                TabPanel(index = 1) { "Tab 2 Content" }
                TabPanel(index = 2) { "Tab 3 Content" }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let tabs = query_component("tab-list").children();

        send_key(&tabs.item(1).unwrap(), "ArrowRight");
        assert_text_content!(query_component("tab-panel"), "Tab 3 Content");
        send_key(&tabs.item(2).unwrap(), "ArrowLeft");
        assert_text_content!(query_component("tab-panel"), "Tab 2 Content");
        send_key(&tabs.item(1).unwrap(), "Home");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");

        send_key(&tabs.item(0).unwrap(), "ArrowDown");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");
        send_key(&tabs.item(0).unwrap(), "ArrowUp");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");

        // Can't test End because of the weird bug where it gets registered as 'e'
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works_vertical() {
    create_scope_immediate(|cx| {
        let view = view! { cx,
            TabGroup(default_index = 1, horizontal = false) {
                TabList {
                    Tab(index = 0) { "Tab 1" }
                    Tab(index = 1) { "Tab 2" }
                    Tab(index = 2) { "Tab 3" }
                }
                TabPanel(index = 0) { "Tab 1 Content" }
                TabPanel(index = 1) { "Tab 2 Content" }
                TabPanel(index = 2) { "Tab 3 Content" }
            }
        };

        sycamore::render_to(|_| view, &test_container());

        let tabs = query_component("tab-list").children();

        send_key(&tabs.item(1).unwrap(), "ArrowDown");
        assert_text_content!(query_component("tab-panel"), "Tab 3 Content");
        send_key(&tabs.item(2).unwrap(), "ArrowUp");
        assert_text_content!(query_component("tab-panel"), "Tab 2 Content");
        send_key(&tabs.item(1).unwrap(), "Home");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");

        send_key(&tabs.item(0).unwrap(), "ArrowRight");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");
        send_key(&tabs.item(0).unwrap(), "ArrowLeft");
        assert_text_content!(query_component("tab-panel"), "Tab 1 Content");

        // Can't test End because of the weird bug where it gets registered as 'e'
    });
}
