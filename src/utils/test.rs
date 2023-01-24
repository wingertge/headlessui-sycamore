use std::error::Error;

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
