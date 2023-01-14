use std::mem;

use sycamore::prelude::*;
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    hooks::create_id,
    utils::{class, get_ref, scoped_children},
};

use super::BaseProps;

pub struct DisclosureContext {
    owner_id: String,
    button_id: String,
    panel_id: String,
}

pub struct DisclosureProperties {
    pub open: &'static Signal<bool>,
    pub disabled: ReactiveBool<'static>,
}

#[derive(Props)]
pub struct DisclosureProps<'cx, G: Html> {
    open: &'cx Signal<bool>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn Disclosure<'cx, G: Html>(cx: Scope<'cx>, props: DisclosureProps<'cx, G>) -> View<G> {
    let owner_id = create_id();
    let button_id = create_id();
    let panel_id = create_id();

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(
            cx,
            DisclosureContext {
                owner_id: owner_id.clone(),
                button_id,
                panel_id,
            },
        );
        provide_context(
            cx,
            DisclosureProperties {
                open: unsafe { mem::transmute(props.open) },
                disabled: unsafe { mem::transmute(props.disabled) },
            },
        );
    });

    props.attributes.exclude_keys(&["id"]);
    let class = class(cx, &props.attributes, props.class);
    view! { cx,
        div(..props.attributes, id = owner_id, class = class, data-sh = "disclosure") {
            (children)
        }
    }
}

#[derive(Props)]
pub struct DisclosureButtonProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DisclosureButton<'cx, G: Html>(
    cx: Scope<'cx>,
    props: DisclosureButtonProps<'cx, G>,
) -> View<G> {
    let context: &DisclosureContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);
    let internal_ref = get_ref::<G>(cx, &props.attributes);
    let children = props.children.call(cx);

    create_effect(cx, {
        let id = context.panel_id.clone();
        move || {
            if let Some(node) = internal_ref.try_get_raw() {
                if *properties.open.get() {
                    node.set_attribute("aria-controls".into(), id.clone().into());
                } else {
                    node.remove_attribute("aria-controls".into());
                }
            }
        }
    });

    let disabled = create_memo(cx, {
        let disabled = properties.disabled.clone();
        let props_disabled = props.disabled.clone();
        move || disabled.get() || props_disabled.get()
    });

    let key_down = {
        let props_disabled = props.disabled.clone();
        let disabled = properties.disabled.clone();
        move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if !disabled.get() && !props_disabled.get() {
                    properties.open.set(!*properties.open.get());
                }
            }
            _ => {}
        }
    };

    let on_click = move |_: MouseEvent| {
        if !properties.disabled.get() && !props.disabled.get() {
            properties.open.set(!*properties.open.get());
        }
    };

    props.attributes.exclude_keys(&[
        "id",
        "ref",
        "aria-expanded",
        "data-sh-expanded",
        "on:click",
        "on:keydown",
        "disabled",
    ]);
    let class = class(cx, &props.attributes, props.class);
    view! { cx,
        button(..props.attributes, id = context.button_id, ref = internal_ref, class = class,
            aria-expanded = *properties.open.get(), data-sh-expanded = *properties.open.get(),
            on:click = on_click, on:keydown = key_down, disabled = *disabled.get(),
            data-sh-owner = context.owner_id, data-sh = "disclosure-button"
        ) {
            (children)
        }
    }
}

#[component]
pub fn DisclosurePanel<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let context: &DisclosureContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id", "data-sh-owner"]);
    let class = class(cx, &props.attributes, props.class);
    view! { cx,
        div(..props.attributes, id = context.panel_id, class = class, data-sh = "disclosure-panel",
            data-sh-owner = context.owner_id
        ) {
            (if *properties.open.get() {
                children.clone()
            } else {
                View::empty()
            })
        }
    }
}
