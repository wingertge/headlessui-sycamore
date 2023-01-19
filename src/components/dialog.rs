use std::mem;

use sycamore::prelude::*;
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{
        as_static, class,
        focus_navigation::{focus_first, get_focusable_elements, lock_focus},
        get_ref, scoped_children, FocusStartPoint,
    },
};

use super::{BaseProps, DisclosureProperties};

#[derive(Props)]
pub struct DialogProps<'cx, G: Html> {
    open: &'cx Signal<bool>,
    #[prop(default)]
    on_open: Option<Box<dyn Fn()>>,
    #[prop(default)]
    on_close: Option<Box<dyn Fn()>>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct DialogContext {
    panel_id: String,
    title_id: String,
    description_id: String,
}

#[component]
pub fn Dialog<'cx, G: Html>(cx: Scope<'cx>, props: DialogProps<'cx, G>) -> View<G> {
    let owner_id = create_id();
    let title_id = create_id();
    let description_id = create_id();

    let fsp = FocusStartPoint::new(cx);

    let context = DialogContext {
        panel_id: create_id(),
        title_id: title_id.clone(),
        description_id: description_id.clone(),
    };
    let disclosure = DisclosureProperties {
        open: as_static(props.open),
        disabled: unsafe { mem::transmute(props.disabled) },
    };

    create_effect(cx, move || {
        if *props.open.get() {
            fsp.save();
            if let Some(on_open) = props.on_open.as_ref() {
                on_open();
            }
        } else {
            if let Some(on_close) = props.on_close.as_ref() {
                on_close();
            }
            fsp.load();
        }
    });

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, context);
        provide_context(cx, disclosure);
    });
    let class = class(cx, &props.attributes, props.class);

    props.attributes.exclude_keys(&[
        "id",
        "role",
        "aria-modal",
        "aria-labelledby",
        "aria-describedby",
    ]);

    view! { cx,
        div(..props.attributes, id = owner_id, data-sh = "dialog", role = "dialog", class = class,
            aria-modal = true, aria-labelledby = title_id, aria-describedby = description_id
        ) {
            (if *props.open.get() {
                children.clone()
            } else {
                View::empty()
            })
        }
    }
}

#[component]
pub fn DialogDescription<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let context: &DialogContext = use_context(cx);
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id"]);

    view! { cx,
        p(..props.attributes, id = context.description_id, data-sh = "dialog-description", class = class) {
            (children)
        }
    }
}

#[component]
pub fn DialogOverlay<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let properties: &DisclosureProperties = use_context(cx);

    let on_click = |_| properties.open.set(false);

    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["on:click"]);

    view! { cx,
        div(..props.attributes, data-sh = "dialog-overlay", on:click = on_click, class = class) {
            (children)
        }
    }
}

#[derive(Props)]
pub struct DialogPanelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DialogPanel<'cx, G: Html>(cx: Scope<'cx>, props: DialogPanelProps<'cx, G>) -> View<G> {
    let context: &DialogContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);
    let node = get_ref(cx, &props.attributes);

    create_effect(cx, move || {
        if *properties.open.get() {
            if let Some(nodes) = get_focusable_elements(node) {
                focus_first(nodes);
            }
        }
    });

    let on_key_down = {
        let disabled = props.disabled.clone();
        move |e: KeyboardEvent| {
            if !disabled.get() {
                match e.key().as_str() {
                    "Tab" => {
                        e.prevent_default();
                        lock_focus(node, e.shift_key());
                    }
                    "Escape" => {
                        properties.open.set(false);
                    }
                    _ => {}
                }
            }
        }
    };

    let children = props.children.call(cx);
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        div(..props.attributes, data-sh = "dialog-panel", id = context.panel_id, ref = node,
            class = class, on:keydown = on_key_down
        ) {
            (children)
        }
    }
}

#[component]
pub fn DialogTitle<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let context: &DialogContext = use_context(cx);
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id"]);

    view! { cx,
        h2(..props.attributes, id = context.title_id, data-sh = "dialog-title", class = class) {
            (children)
        }
    }
}
