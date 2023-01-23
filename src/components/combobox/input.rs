use super::ComboboxContext;
use crate::{
    components::DisclosureProperties,
    utils::{class, get_ref, SetDynAttr},
};
use sycamore::{builder::prelude::input, prelude::*, web::html::ev};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

#[derive(Props)]
pub struct ComboboxInputProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = input.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "combobox-input".into());

    element.set_attribute("id".into(), context.input_id.clone().into());
    element.set_attribute("aria-haspopup".into(), "listbox".into());
    element.set_attribute("aria-controls".into(), context.options_id.clone().into());
    element.set_dyn_bool(cx, "disabled", move || *disabled.get());
    element.set_dyn_bool(cx, "aria-expanded", move || *properties.open.get());
    element.set_dyn_bool(cx, "data-sh-expanded", move || *properties.open.get());

    element.event(cx, ev::keydown, on_key_down);
    element.event(cx, ev::click, on_click);
    element.event(cx, ev::mouseenter, move |_| context.hovering.set(true));
    element.event(cx, ev::mouseleave, move |_| context.hovering.set(false));

    view
}
