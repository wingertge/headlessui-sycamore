use std::mem;

use sycamore::{
    builder::prelude::{button, div},
    prelude::*,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::{KeyboardEvent, MouseEvent};

use crate::{
    hooks::create_id,
    utils::{as_static, class, get_ref, scoped_children, SetDynAttr},
};

use super::{TransitionContext, TransitionProp};

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
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn Disclosure<'cx, G: Html>(cx: Scope<'cx>, props: DisclosureProps<'cx, G>) -> View<G> {
    let owner_id = create_id();
    let button_id = create_id();
    let panel_id = create_id();

    let context = DisclosureContext {
        owner_id: owner_id.clone(),
        button_id,
        panel_id,
    };

    let children = scoped_children(cx, props.children, move |cx| {
        provide_context(cx, context);
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

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), owner_id.into());
    element.set_attribute("data-sh".into(), "disclosure".into());

    view
}

#[derive(Props)]
pub struct DisclosureButtonProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default = button.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let class = class(cx, &props.attributes, props.class);

    props.attributes.exclude_keys(&[
        "id",
        "ref",
        "aria-expanded",
        "data-sh-expanded",
        "on:click",
        "on:keydown",
        "disabled",
    ]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), context.button_id.clone().into());
    element.set_attribute("data-sh".into(), "disclosure-button".into());
    element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    element.set_dyn_bool(cx, "aria-expanded", move || *properties.open.get());
    element.set_dyn_bool(cx, "data-sh-expanded", move || *properties.open.get());
    element.set_dyn_bool(cx, "disabled", move || *disabled.get());

    element.event(cx, ev::click, on_click);
    element.event(cx, ev::keydown, key_down);

    view
}

#[derive(Props)]
pub struct DisclosurePanelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    transition: Option<TransitionProp<'cx, G>>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn DisclosurePanel<'cx, G: Html>(
    cx: Scope<'cx>,
    props: DisclosurePanelProps<'cx, G>,
) -> View<G> {
    let context: &DisclosureContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&["id", "data-sh-owner"]);
    let class = class(cx, &props.attributes, props.class);

    let apply_attributes = |element: &G| {
        element.set_dyn_attr(cx, "class", move || class.to_string());
        element.set_children(cx, children);
        element.apply_attributes(cx, &props.attributes);

        element.set_attribute("id".into(), context.panel_id.clone().into());
        element.set_attribute("data-sh".into(), "disclosure-panel".into());
        element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    };

    if let Some(transition) = props.transition {
        let mut view = View::empty();
        let node_ref = create_node_ref(cx);
        create_child_scope(cx, |cx| {
            provide_context(
                cx,
                TransitionContext::<G> {
                    node_ref: as_static(node_ref),
                },
            );
            view = transition(cx, as_static(properties.open));
        });
        let element = node_ref.get_raw();
        apply_attributes(&element);
        view
    } else {
        let view = props.element.call(cx);
        let element = view.as_node().unwrap();
        apply_attributes(element);

        view! { cx,
            (if *properties.open.get() {
                view.clone()
            } else {
                View::empty()
            })
        }
    }
}
