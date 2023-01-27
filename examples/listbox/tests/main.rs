use std::error::Error;

use fantoccini::{Client, Locator};
use test_utils::dom_test;

#[tokio::test]
pub async fn dom_shape_is_correct() {
    dom_test!(test);

    async fn test(c: &mut Client) -> Result<(), Box<dyn Error>> {
        c.goto("http://localhost:8080").await?;
        let container = c
            .wait()
            .for_element(Locator::Css(r#"div[data-sh="listbox"]"#))
            .await?;
        let container_id = container.attr("id").await?.unwrap();
        let label = c
            .wait()
            .for_element(Locator::Css(r#"label[data-sh="listbox-label"]"#))
            .await?;
        let button = c
            .wait()
            .for_element(Locator::Css(r#"button[data-sh="listbox-button"]"#))
            .await?;

        assert_eq!(container.find_all(Locator::Css("*")).await?.len(), 2);
        assert_eq!(
            container.attr("aria-labelledby").await?.unwrap(),
            label.attr("id").await?.unwrap()
        );
        assert_eq!(button.attr("aria-haspopup").await?.unwrap(), "listbox");

        button.click().await?;

        let options = c
            .wait()
            .for_element(Locator::Css(r#"ul[data-sh="listbox-options"]"#))
            .await?;

        assert_eq!(
            options.attr("id").await?.unwrap(),
            button.attr("aria-controls").await?.unwrap(),
            "aria-controls should be set to the id of the options panel"
        );
        assert_eq!(options.attr("role").await?.unwrap(), "listbox");
        assert_eq!(
            options.attr("aria-labelledby").await?.unwrap(),
            button.attr("id").await?.unwrap()
        );
        assert_eq!(options.attr("aria-orientation").await?.unwrap(), "vertical");
        assert_eq!(options.attr("tabindex").await?.unwrap(), "0");

        let option_items = options
            .find_all(Locator::Css(r#"li[data-sh="listbox-option"]"#))
            .await?;

        assert_eq!(option_items.len(), 3);
        assert_eq!(option_items[0].text().await?, "Hello");
        assert_eq!(option_items[1].text().await?, "World");
        assert_eq!(option_items[2].text().await?, "Test");

        for option in option_items.iter() {
            assert_eq!(option.attr("role").await?.unwrap(), "option");
            assert_eq!(option.attr("tabindex").await?.unwrap(), "-1");
            assert_eq!(option.attr("data-sh-owner").await?.unwrap(), container_id);
        }

        option_items[2].click().await?;

        let value = c.find(Locator::Id("listbox-value")).await?;
        assert_eq!(value.text().await?, "Test");

        Ok(())
    }
}
