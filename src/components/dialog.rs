use std::mem;

use sycamore::{
    builder::prelude::{div, h2, p},
    prelude::*,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{
        as_static, class,
        focus_navigation::{focus_first, get_focusable_elements, lock_focus},
        get_ref, scoped_children, FocusStartPoint, SetDynAttr,
    },
};

use super::{BaseProps, DisclosureProperties, TransitionProp};

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
    #[prop(default)]
    transition: Option<TransitionProp<'cx, G>>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let apply_attributes = |element: &G| {
        element.set_dyn_attr(cx, "class", move || class.to_string());
        element.set_children(cx, children);
        element.apply_attributes(cx, &props.attributes);

        element.set_attribute("id".into(), owner_id.into());
        element.set_attribute("data-sh".into(), "dialog".into());
        element.set_attribute("role".into(), "dialog".into());
        element.set_attribute("aria-labelledby".into(), title_id.into());
        element.set_attribute("aria-describedby".into(), description_id.into());
        element.set_attribute("aria-modal".into(), "".into());
    };

    if let Some(mut transition) = props.transition {
        let view = transition(cx, props.open);
        if let Some(element) = view.as_node() {
            apply_attributes(element);
        }
        view
    } else {
        let view = props.element.call(cx);
        let element = view.as_node().unwrap();
        apply_attributes(element);

        view! { cx,
            (if *props.open.get() {
                view.clone()
            } else {
                View::empty()
            })
        }
    }
}

#[derive(Props)]
pub struct DialogDescriptionProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = p.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DialogDescription<'cx, G: Html>(
    cx: Scope<'cx>,
    props: DialogDescriptionProps<'cx, G>,
) -> View<G> {
    let context: &DialogContext = use_context(cx);
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), context.description_id.clone().into());
    element.set_attribute("data-sh".into(), "dialog-description".into());

    view
}

#[derive(Props)]
pub struct DialogOverlayProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = p.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DialogOverlay<'cx, G: Html>(cx: Scope<'cx>, props: DialogOverlayProps<'cx, G>) -> View<G> {
    let properties: &DisclosureProperties = use_context(cx);

    let on_click = |_| properties.open.set(false);

    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["on:click"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "dialog-overlay".into());

    element.event(cx, ev::click, on_click);

    view
}

#[derive(Props)]
pub struct DialogPanelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "dialog-panel".into());
    element.set_attribute("id".into(), context.panel_id.clone().into());

    element.event(cx, ev::keydown, on_key_down);

    view
}

#[derive(Props)]
pub struct DialogTitleProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = h2.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DialogTitle<'cx, G: Html>(cx: Scope<'cx>, props: DialogTitleProps<'cx, G>) -> View<G> {
    let context: &DialogContext = use_context(cx);
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "dialog-title".into());
    element.set_attribute("id".into(), context.title_id.clone().into());

    view
}
