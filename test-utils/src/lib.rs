#[macro_export]
macro_rules! dom_test {
    ($test: expr) => {
        let headless = ::std::env::var("RUN_WASM_TESTS_HEADLESS").is_ok();
        // Set the capabilities of the client
        let mut capabilities = ::serde_json::Map::new();
        let firefox_opts;
        let chrome_opts;
        if headless {
            firefox_opts = ::serde_json::json!({ "args": ["--headless"] });
            chrome_opts = ::serde_json::json!({ "args": ["--headless"] });
        } else {
            firefox_opts = ::serde_json::json!({ "args": [] });
            chrome_opts = ::serde_json::json!({ "args": [] });
        }
        capabilities.insert("moz:firefoxOptions".to_string(), firefox_opts);
        capabilities.insert("goog:chromeOptions".to_string(), chrome_opts);

        let mut client = ::fantoccini::ClientBuilder::native()
            .capabilities(capabilities)
            .connect(&"http://localhost:4444")
            .await
            .expect("failed to connect to WebDriver");
        let output = $test(&mut client).await;
        // Close the client no matter what
        client
            .close()
            .await
            .expect("failed to close Fantoccini client");
        // Panic if the test failed
        if let Err(err) = output {
            panic!("test failed: '{}'", err.to_string())
        }
    };
}

/// Returns a [`Element`] referencing the test container with the contents cleared.
pub fn test_container() -> Element {
    if document()
        .query_selector("test-container#test-container")
        .unwrap()
        .is_none()
    {
        document()
            .body()
            .unwrap()
            .insert_adjacent_html(
                "beforeend",
                r#"<test-container id="test-container"></test-container>"#,
            )
            .unwrap();
    }

    let container = query("test-container#test-container");

    container.set_inner_html(""); // erase contents from previous test runs

    container
}

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, KeyboardEvent, KeyboardEventInit, Window};

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> Document {
    window().document().unwrap()
}

/// Query the `Document` for the first `Element` that matches the selectors.
///
/// This is a test utility function only!
///
/// # Panics
///
/// Panics if the selectors string is invalid or if no element was found that
/// matches the selectors
pub fn query(selectors: &str) -> Element {
    document()
        .query_selector(selectors)
        .expect("selectors should be valid")
        .expect("element to be found that matches the selectors")
}

pub fn query_component(name: &str) -> Element {
    query(&format!("[data-sh=\"{name}\"]"))
}

pub fn query_component_in(element: &Element, name: &str) -> Element {
    let query = format!("[data-sh=\"{name}\"]");
    element
        .query_selector(&query)
        .expect("selectors should be valid")
        .expect("element to be found that matches the selectors")
}

/// Query the `Document` for the first `Element` that matches the selectors and
/// then try to cast it into the generic type `T`.
///
/// This is a test utility function only!
///
/// # Panics
///
/// Panics if:
/// - the selectors string is invalid
/// - no element was found that matches the selectors
/// - element found cannot be cast to the generic type `T` used
pub fn query_into<T: AsRef<HtmlElement> + JsCast>(selectors: &str) -> T {
    // dyn_into -> unwrap to eagerly cause a panic if the query doesn't match
    // the generic T.
    query(selectors)
        .dyn_into()
        .expect("element found should be of the same type as used for the generic T")
}

pub fn send_key(element: &Element, key: &str) {
    let key_event = KeyboardEvent::new_with_keyboard_event_init_dict(
        "keydown",
        KeyboardEventInit::new()
            .bubbles(true)
            .cancelable(true)
            .key(key),
    )
    .unwrap();
    element
        .unchecked_ref::<HtmlElement>()
        .dispatch_event(key_event.unchecked_ref())
        .unwrap();
}

/// Asserts that the text content of a `web_sys::Node` is equal to the
/// right expression.
#[macro_export]
macro_rules! assert_text_content {
    ($element: expr, $right: expr $(,)?) => {
        assert_eq!($element.text_content().unwrap(), $right);
    };
}
