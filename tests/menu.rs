use headlessui_sycamore::components::{Menu, MenuItem};
use sycamore::prelude::*;
use test_utils::{assert_text_content, document, query_component, send_key, test_container};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn structure_is_correct() {
    create_scope_immediate(|cx| {
        let node = view! { cx,
            Menu {
                MenuItem { "Item 1" }
                MenuItem { "Item 2" }
                MenuItem { "Item 3" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("menu");
        let children = container.children();

        assert_eq!(container.tag_name(), "DIV");
        assert_eq!(children.length(), 3);
        assert_eq!(container.get_attribute("role").unwrap(), "menu");
        assert_eq!(container.get_attribute("tabindex").unwrap(), "0");

        for i in 0..3 {
            let item = children.item(i).unwrap();
            assert_eq!(item.get_attribute("role").unwrap(), "menuitem");
            assert_eq!(item.get_attribute("tabindex").unwrap(), "-1");
            assert_text_content!(item, format!("Item {}", i + 1));
        }
    });
}

#[wasm_bindgen_test]
pub fn keyboard_works() {
    create_scope_immediate(|cx| {
        let clicked = create_signal(cx, -1);

        let node = view! { cx,
            Menu {
                MenuItem(on:click = |_| clicked.set(0)) { "a" }
                MenuItem(on:click = |_| clicked.set(1)) { "b" }
                MenuItem(on:click = |_| clicked.set(2)) { "c" }
            }
        };

        sycamore::render_to(|_| node, &test_container());

        let container = query_component("menu");
        let children = container.children();

        let child1 = children.item(0).unwrap();
        let child2 = children.item(1).unwrap();
        let child3 = children.item(2).unwrap();

        container.unchecked_ref::<HtmlElement>().focus().unwrap();

        assert_eq!(child1, document().active_element().unwrap());

        send_key(&child1, "ArrowDown");
        assert_eq!(child2, document().active_element().unwrap());
        send_key(&child2, " ");
        assert_eq!(*clicked.get(), 1);
        send_key(&child2, "ArrowRight");
        assert_eq!(child3, document().active_element().unwrap());
        send_key(&child3, "Enter");
        assert_eq!(*clicked.get(), 2);

        send_key(&child3, "Home");
        assert_eq!(
            child1,
            document().active_element().unwrap(),
            "a should be focused, but {} is",
            document().active_element().unwrap().text_content().unwrap()
        );
        // `End` gets registered as 'e' on my machine, I don't know why but that's why I can't test this
        /*         send_key(&child1, "End");
        assert_eq!(
            child3,
            document().active_element().unwrap(),
            "c should be focused, but {} is",
            document().active_element().unwrap().text_content().unwrap()
        ); */
        // Can't automatically test selection by starting letter because it has a 100ms delay
    });
}
