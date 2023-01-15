use super::ComboboxContext;
use crate::{
    components::DisclosureProperties,
    utils::{class, get_ref},
};
use sycamore::prelude::*;
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

#[derive(Props)]
pub struct ComboboxInputProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ComboboxInput<'cx, G: Html>(cx: Scope<'cx>, props: ComboboxInputProps<'cx, G>) -> View<G> {
    let context: &ComboboxContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);

    let node = get_ref(cx, &props.attributes);

    create_effect(cx, {
        let id = context.options_id.clone();
        move || {
            if let Some(node) = node.try_get_raw() {
                if *properties.open.get() {
                    node.set_attribute("aria-controls".into(), id.clone().into());
                } else {
                    node.remove_attribute("aria-controls".into());
                }
            }
        }
    });

    let on_key_down = {
        let disabled = props.disabled.clone();
        let properties_disabled = properties.disabled.clone();
        move |e: KeyboardEvent| {
            if !properties_disabled.get() && !disabled.get() {
                match e.key().as_str() {
                    "ArrowUp" | "ArrowDown" => {
                        e.prevent_default();
                        properties.open.set(!*properties.open.get_untracked());
                    }
                    _ => {}
                }
            }
        }
    };
    let on_click = {
        let disabled = props.disabled.clone();
        let properties_disabled = properties.disabled.clone();
        move |_| {
            if !properties_disabled.get() && !disabled.get() {
                properties.open.set(!*properties.open.get_untracked());
            }
        }
    };
    let disabled = create_memo(cx, move || {
        properties.disabled.get() || props.disabled.get()
    });
    let class = class(cx, &props.attributes, props.class);
    props.attributes.exclude_keys(&[
        "on:keydown",
        "on:click",
        "id",
        "class",
        "on:mouseenter",
        "on:mouseleave",
        "aria-haspopup",
        "aria-controls",
        "disabled",
        "aria-expanded",
        "data-sh-expanded",
        "data-sh",
    ]);

    view! { cx,
        input(
            on:keydown = on_key_down, on:click = on_click, id = context.input_id, class = class,
            on:mouseenter = move |_| context.hovering.set(true), aria-haspopup = "listbox",
            on:mouseleave = move |_| context.hovering.set(false), aria-controls = context.options_id,
            disabled = *disabled.get(), aria-expanded = *properties.open.get(), ..props.attributes,
            data-sh-expanded = *properties.open.get(), data-sh = "listbox-button"
        )
    }
}
